/// Programs and how to use them
/// 
/// 
/// 
/// 

use std::{collections::BTreeMap, error::Error, fmt::{Debug, Display}};
use nannou_osc::{Message, Type};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
// for file handeling
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rmp_serde;
use crate::{Parameter, ParameterEnd, ParameterEndpoint, ParameterFactory, ParameterStore};

pub type Interpolator = fn(f32) -> f32;
pub type Pid = u32;

#[derive(Debug)]
pub enum LoadStoreError {
    IoError( std::io::Error ),
    SerializeError(rmp_serde::encode::Error),
    DeserializeError(rmp_serde::decode::Error),
}
impl Display for LoadStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadStoreError::IoError(e) => write!(f, "IoError: {}",  e),
            LoadStoreError::SerializeError(e) => write!(f, "SerializeError: {}",  e),
            LoadStoreError::DeserializeError(e) => write!(f, "DeserializeError: {}",  e)
        }
    }
}

impl Error for LoadStoreError {
    
}

/// list of interpolation mappings
const INTERPOLATIONS : [Interpolator; 4] = [
    |x| x,                      // linear
    |x| x*x,                    // ease out // slow out hard in
    |x| (2.0-x)*x,              // hard in slow out
    |x| -2.0*x*x*x+ 3.0*x*x,    // slow in and slow out,s
];

fn ser_interpol <S:Serializer> (interpol: &Interpolator, serializer: S) -> Result<S::Ok, S::Error> { 
    if let Ok(i) = INTERPOLATIONS.binary_search(interpol) {
        serializer.serialize_u64(i as u64)
    } else {
        serializer.serialize_u64(0)
    }
}

fn deser_interpol<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Interpolator, D::Error> { 
    let u : u64 =  Deserialize::deserialize(deserializer)?;
    if u as usize >= INTERPOLATIONS.len() { return Err(serde::de::Error::custom("invalid interpol index")) }
    Ok(INTERPOLATIONS[u as usize])
}

///
/// Stores an configuration of Parameters (Keyframe)
/// and parameters to setup interpolate in between
///
/// Similar to a Keyframe in animation software
/// 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Program {
    delay: f32,                 // delay to fade to this program
    #[serde(serialize_with = "ser_interpol")]
    #[serde(deserialize_with = "deser_interpol")]
    interpol: Interpolator,     // function to manipulate interpolation 
    auto_next: Option<Pid>,     // option to automatically go to next program
    config:  Vec<Parameter>,    // all parameters
}


impl Program {
    /// create a new Program, as a snapshot of the current state in the Parameters Store
    /// ## Parameters
    /// - `delay` : time in s to fade/interpolate into this program
    /// - `inter_id` : id of the interpolation method that should be used
    /// - `auto_next` : if set after finished interpolation automatically start the next program with this id
    /// - `store` : [ParameterStore] to take the snapshot from
    /// 
    pub fn new(delay:f32 , inter_id: usize, auto_next: Option<Pid>, store: &ParameterStore) -> Self {
        // default to linear interpolation on unavailable
        let inter = INTERPOLATIONS.get(inter_id).unwrap_or(&INTERPOLATIONS[0]);

        Self {
            delay, 
            interpol: *inter,
            auto_next,
            config: store.config_copy(),
        }
    }
}

/// switch/fade program parameters 
pub struct ProgramSwitcher {
    /// prev config, used for interpolation
    last_config:  Vec<Parameter>, 
    /// next program
    prog: Program,                
    start_time: f32,              
}

impl ProgramSwitcher {
    /// create a new fading to some program.
    /// - returns [None] if there are parameters missing in the store
    /// or the order of the parameters is wrong
    pub fn new(prog: &Program, time: f32, store: &ParameterStore) -> Option<Self> {
        let start_time = time;
        let last_config = store.config_copy();
        let s = Self {
            last_config,
            prog: prog.clone(),
            start_time,
        };

        // check that the current store contains all parameters in the program
        if s.last_config.len() == prog.config.len() && s.last_config.iter().zip(prog.config.iter()).all(|(a,b)|a.address== b.address ) {
            println!("Started new Program transition {:?}", prog);
            return Some(s);
        }

        None
    }

    /// update the ParameterStore to fade to some config
    pub fn update(&self, time: f32, store: &mut ParameterStore) -> bool{
        let run_time = time - self.start_time;
        let t = (run_time / self.prog.delay).clamp(0.0, 1.0);  // (0.0 -> 1.0) over the time of the delay;
        let factor = (self.prog.interpol)(t);                          // apply some function to the interpolation

        
        // last_config and program is checked when created. 
        // assume store is not changed after the updater was created.
        for (i,(a,b)) in self.last_config.iter().zip(self.prog.config.iter()).enumerate() {
            let vec = a.values.clone().iter().zip(b.values.iter()).map(|(x,y)|  x.interpol(factor, y)  ).collect();
              store.set_value(i, vec );
        }
        
        
        t < 0.9999
    }
}

/// manages storing, and running programs
pub struct  ProgramManager {
    programs: BTreeMap<Pid, Program>,
    locals: ParameterStore,
    current: Option<Box<ProgramSwitcher>>,
    delay: ParameterEndpoint<f32>,
    interpol : ParameterEndpoint<i32>,
    auto_next : ParameterEndpoint<i32>,
}

impl ProgramManager {
    pub fn new() -> Self{

        let mut locals= ParameterStore::new();
        let mut factory = ParameterFactory::new("program".to_string(), &mut locals);
        let delay = factory.build_default(10.0,"delay".to_string());
        let interpol = factory.build_default(0,"interpol".to_string());
        let auto_next = factory.build_default(0,"auto_next".to_string());

        println!("-- ProgramManager --  \n\r{}", locals );

        Self {
            programs: BTreeMap::new(),
            locals,
            current: None,
            delay,
            interpol,
            auto_next,
        }
    }

    /// update the fade to some program if currently running
    pub fn update(&mut self , time: f32, store: &mut ParameterStore) {
        if let Some(up) = &self.current {
            if !up.update(time, store) { // current program is finished (not running)
                // use auto_next if available
                self.current = if let Some(p) = &up.prog.auto_next {
                    self.programs.get(&p).map(|prg| Box::new(ProgramSwitcher::new(prg, time, store).unwrap()))
                } else {
                    None
                }
            }
        }
    }

    /// apply osc massages to the Parameter manages
    /// - change local parameter
    /// - run events (add/store Program and run/load Program)
    pub fn update_osc(&mut self , time: f32, store: &ParameterStore, msg: &Message) {
        // update local variables    
        self.locals.update(msg);

        // println!("{:?}", self.locals ); //print current locals state

        //create new prog on event
        if msg.addr == "/program/add" {
            if let Some(a)= &msg.args {
                if let Some(Type::Int(i)) = a.first() {
                        self.add((*i) as u32,store);
                }
            }
            
        }

        //run stored program on event
        if msg.addr == "/program/run" {
            if let Some(a)= &msg.args {
                if let Some(Type::Int(i)) = a.first() {
                        self.run((*i) as u32,time,store);
                }
            }
            
        }

        if msg.addr == "/program/store" {
            if let Some(a) = &msg.args {
                if let Some(Type::String(s)) = a.first() {
                    if let Err(e) =  self.store(Path::new(s)) {
                        println!("store_error: {}", e);
                    }
                }
            }
        }

        if msg.addr == "/program/load" {
            if let Some(a) = &msg.args {
                if let Some(Type::String(s)) = a.first() {
                    if let Err(e) =  self.load(Path::new(s)) {
                        println!("load_error: {}", e);
                    }
                }
            }
        }

    }

    fn add(&mut self, p: Pid, store: &ParameterStore) {

        let next_pid = self.auto_next.get(&self.locals);
        let next = if next_pid > 0 {Some(next_pid as u32)} else {None};
        
        let delay = self.delay.get(&self.locals);
        let inter_id= self.interpol.get(&self.locals) as usize;

        let prg = Program::new(delay,  inter_id, next, store);

        println!("Adding new Program {}, {:?}", p, prg);
        self.programs.insert(p, prg);
        
    }
    
    /// run a program
    fn run(& mut self, p: Pid, time: f32, store: &ParameterStore) {

        self.current = self.programs.get(&p).map(|prg| Box::new(ProgramSwitcher::new(prg, time, store).unwrap()))
    }

    pub fn store(&self ,path: &Path) -> Result<(),LoadStoreError> {
        let buf = rmp_serde::to_vec(&self.programs).map_err(|e|  LoadStoreError::SerializeError(e))?;
        
        let mut file = File::create(path).map_err(|e|  LoadStoreError::IoError(e))?;
        
        //Used for debug Human Readable format
        let  file_json = File::create(path.with_extension("json")).map_err(|e|  LoadStoreError::IoError(e))?;
        serde_json::to_writer(file_json, &self.programs).expect("json error");
        
        file.write_all(&buf).map_err(|e|  LoadStoreError::IoError(e))
    }

    pub fn load(&mut self ,path: &Path) -> Result<(),LoadStoreError> {
        let file = File::open(path).map_err(|e|  LoadStoreError::IoError(e))?;

        let loaded_programs  = rmp_serde::from_read(file).map_err(|e|  LoadStoreError::DeserializeError(e))?;
        self.programs = loaded_programs;
        Ok(())
    }
}

impl Default for ProgramManager {
     fn default() -> Self {
       Self::new()
    }
 }

/// linear interpolation
pub trait Interpolate : Clone {
    /// linear interpolate from one parameter to another
    /// from 0 to 1 =>  a to b
    /// - interpolation(0,a,b) == a
    /// - interpolation(1,a,b) == b
    fn interpolation(f: f32, a: &Self, b: &Self) -> Self;
    
    fn interpol(&self, f: f32, b: &Self) -> Self {
        Self::interpolation(f, self, b)
    }
}

impl Interpolate for Type  {
    /// linear interpolation for OSC parameters
    fn interpolation(f: f32, a: &Self, b: &Self) -> Self {
        match (a,b) {
            (Type::Int(ax), Type::Int(bx)) => Type::Int(   ((1.0-f)* (*ax) as f32 + f* (*bx) as f32 ) as i32   ),
            (Type::Float(ax), Type::Float(bx)) => Type::Float((1.0-f)*ax + f*bx ),
            (Type::Long(ax), Type::Long(bx)) => Type::Long(   ((1.0-f)* (*ax) as f32 + f* (*bx) as f32 ) as i64   ),
            (Type::Double(ax), Type::Double(bx)) => Type::Double((1.0-f as f64 )*ax + f as f64 *bx ),
            _ => Type::Nil
        }
    }
}