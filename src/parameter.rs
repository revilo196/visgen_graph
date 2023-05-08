use crate::osc_convert::FromOscType;
///
/// # Parameters
/// Parameters are used to control behavior/ values, inside diffident modules
///
/// # Store
/// Parameter are stored in an central [ParameterStore]
///
use nannou_osc::Message;
use rosc::{OscColor, OscMidiMessage, OscType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::{collections::BTreeMap, fmt::Debug}; // Import `fmt`

#[derive(Serialize, Deserialize)]
#[serde(remote = "OscColor")]
pub struct OscColorDef {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "OscMidiMessage")]
pub struct OscMidiMessageDef {
    pub port: u8,
    pub status: u8,
    pub data1: u8, // maybe use an enum for data?
    pub data2: u8,
}

#[derive(Serialize, Deserialize)]
enum OscTypeDef {
    Int(i32),
    Float(f32),
    String(String),
    Blob(Vec<u8>),
    // use struct for time tag to avoid destructuring
    Time(u32, u32),
    Long(i64),
    Double(f64),
    Char(char),
    #[serde(with = "OscColorDef")]
    Color(OscColor),
    #[serde(with = "OscMidiMessageDef")]
    Midi(OscMidiMessage),
    Bool(bool),
    Nil,
    Inf,
}

fn to_local_osc(osc_type: OscType) -> OscTypeDef {
    match osc_type {
        OscType::Int(i) => OscTypeDef::Int(i),
        OscType::Float(f) => OscTypeDef::Float(f),
        OscType::String(s) => OscTypeDef::String(s),
        OscType::Blob(b) => OscTypeDef::Blob(b),
        OscType::Time(i, j) => OscTypeDef::Time(i, j),
        OscType::Long(l) => OscTypeDef::Long(l),
        OscType::Double(d) => OscTypeDef::Double(d),
        OscType::Char(c) => OscTypeDef::Char(c),
        OscType::Color(rgba) => OscTypeDef::Color(rgba),
        OscType::Midi(m) => OscTypeDef::Midi(m),
        OscType::Bool(b) => OscTypeDef::Bool(b),
        OscType::Nil => OscTypeDef::Nil,
        OscType::Inf => OscTypeDef::Inf,
    }
}

fn to_external_osc(osc_type: OscTypeDef) -> OscType {
    match osc_type {
        OscTypeDef::Int(i) => OscType::Int(i),
        OscTypeDef::Float(f) => OscType::Float(f),
        OscTypeDef::String(s) => OscType::String(s),
        OscTypeDef::Blob(b) => OscType::Blob(b),
        OscTypeDef::Time(i, j) => OscType::Time(i, j),
        OscTypeDef::Long(l) => OscType::Long(l),
        OscTypeDef::Double(d) => OscType::Double(d),
        OscTypeDef::Char(c) => OscType::Char(c),
        OscTypeDef::Color(rgba) => OscType::Color(rgba),
        OscTypeDef::Midi(m) => OscType::Midi(m),
        OscTypeDef::Bool(b) => OscType::Bool(b),
        OscTypeDef::Nil => OscType::Nil,
        OscTypeDef::Inf => OscType::Inf,
    }
}

fn type_vec_ser<S: Serializer>(vec: &Vec<OscType>, serializer: S) -> Result<S::Ok, S::Error> {
    // First convert the vector into a Vec<LocalColor>.
    let vec2: Vec<OscTypeDef> = vec.clone().into_iter().map(to_local_osc).collect();
    // Instead of serializing Vec<ExternalCrateColor>, we serialize Vec<LocalColor>.
    vec2.serialize(serializer)
}

fn type_vec_deser<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<OscType>, D::Error> {
    // Deserialize as if it was a Vec<LocalColor>.
    let vec: Vec<OscTypeDef> = Deserialize::deserialize(deserializer)?;

    // Convert it into an Vec<ExternalCrateColor>
    Ok(vec.into_iter().map(to_external_osc).collect())
}

/// private struct that stores:
///   * **value**: current value(s) of the parameter
///   * **address**: full osc-address of the parameter
///
/// similar to [Message]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameter {
    #[serde(serialize_with = "type_vec_ser")]
    #[serde(deserialize_with = "type_vec_deser")]
    pub(crate) values: Vec<OscType>,
    pub(crate) address: String,
}

impl Parameter {
    /// creates new Parameter with empty values and a given address.
    pub fn new(path: String, values: Vec<OscType>) -> Self {
        Self {
            values,
            address: path,
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}

type ParameterIndex = usize;
///
/// Stores [Parameter]s in an fast to access and fast to update structure.
///
/// the store ist structures similar to an [indextree], Parameters are Stored in an [Vec].
/// Paths are an [BTreeMap], this maps Paths to an Index into the Vec.
///
/// [Parameter]s get added when creating new [ParameterEndpoint]s or using the [ParameterFactory]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ParameterStore {
    parameters: Vec<Parameter>,
    paths: BTreeMap<String, ParameterIndex>,
}

impl ParameterStore {
    /// creates empty ParameterStore
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            paths: BTreeMap::new(),
        }
    }

    /// register a new Parameter in the store
    ///
    /// * [Parameter] is moved inside the store
    /// * *return:* Fast access [ParameterIndex] of the parameter inside the Store
    ///
    /// see: [get_value()]
    fn insert_parameter(&mut self, par: Parameter) -> ParameterIndex {
        if let Some(i) = self.paths.get(&par.address) {
            *i // no insertion if already there
        } else {
            let index = self.parameters.len();
            self.paths.insert(par.address.clone(), index);
            self.parameters.push(par);
            index
        }
    }

    /// fast read access using [ParameterIndex]
    ///
    /// returns [None] for non existing indices
    pub fn get_value(&self, token: ParameterIndex) -> Option<Vec<OscType>> {
        self.parameters.get(token).map(|f| f.values.clone())
    }

    /// read access using the path string
    ///
    /// returns [None] for non existing addresses
    pub fn get_path_value(&self, path: &str) -> Option<Vec<OscType>> {
        if let Some(i) = self.paths.get(path) {
            self.get_value(*i)
        } else {
            None
        }
    }

    /// routes a Message to the matching address Parameter
    /// and updates using the attached Values
    pub fn update(&mut self, msg: &Message) {
        if let Some(arg) = msg.args.clone() {
            if let Some(i) = self.paths.get(&msg.addr) {
                self.parameters[*i].values = arg;
            }
        }
    }

    pub fn set_value(&mut self, token: ParameterIndex, value: Vec<OscType>) {
        self.parameters[token].values = value;
    }

    pub fn config_copy(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }
}

impl fmt::Display for ParameterStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in &self.parameters {
            let res = writeln!(f, "{}", p);

            res?;
        }
        Ok(())
    }
}
pub trait ParameterEnd<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get(&self, store: &ParameterStore) -> T;
    fn get_vec(&self, store: &ParameterStore) -> Vec<T>;
    fn bind<'a>(&'a self, store: &'a ParameterStore) -> ParameterHandle<'a, T>;
}

///
/// defines an way to receive values from the [ParameterStore]
///
#[derive(Clone)]
pub struct ParameterEndpoint<T> {
    length: usize,
    default: T,
    p_index: ParameterIndex,
}

impl<T> ParameterEndpoint<T>
where
    T: FromOscType,
    T: Clone,
{
    pub fn new_len(default: T, length: usize, path: String, store: &mut ParameterStore) -> Self {
        let mut values = Vec::new();
        for _ in 0..length {
            values.push(default.clone().into_osc());
        }

        let par = Parameter::new(path, values);
        let p_index = store.insert_parameter(par);

        Self {
            length,
            default,
            p_index,
        }
    }

    pub fn new(default: T, path: String, store: &mut ParameterStore) -> Self {
        Self::new_len(default, 1, path, store)
    }
}

impl<T> ParameterEnd<T> for ParameterEndpoint<T>
where
    T: FromOscType,
    T: Clone,
{
    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length > 0
    }

    ///creating a handle binds a &[ParameterStore]  making further access easy using [From]
    fn bind<'a>(&'a self, store: &'a ParameterStore) -> ParameterHandle<'a, T> {
        ParameterHandle {
            parameter: self,
            store,
        }
    }

    /// read single value from the parameter from the parameter store
    ///
    /// stored default value is used if value from the store is invalid
    fn get(&self, store: &ParameterStore) -> T {
        if let Some(values) = store.get_value(self.p_index) {
            if let Some(value) = values.first() {
                if let Some(val) = T::from_osc(value.clone()) {
                    return val;
                };
            }
        }
        self.default.clone()
    }

    /// read values from the parameter from the parameter store
    ///
    /// stored default value is used if value from the store is invalid
    fn get_vec(&self, store: &ParameterStore) -> Vec<T> {
        if let Some(types) = store.get_value(self.p_index) {
            if self.len() == types.len() {
                let opt_values: Vec<Option<T>> =
                    types.iter().map(|f| T::from_osc(f.clone())).collect();
                if !opt_values.iter().any(|f| f.is_none()) {
                    return opt_values.iter().map(|f| f.clone().unwrap()).collect();
                }
            }
        }
        let mut default = Vec::new();
        default.resize(self.length, self.default.clone());
        default
    }
}

/// constructs [ParameterEndpoint]s with similar paths/address
/// Parameter are automatically put into the [ParameterStore]
pub struct ParameterFactory<'a> {
    path: String,
    store: &'a mut ParameterStore,
}

impl<'a> ParameterFactory<'a> {
    pub fn new(path: String, store: &'a mut ParameterStore) -> Self {
        Self {
            path: format!("/{}", path),
            store,
        }
    }

    /// Build a new ParameterEndpoint using [Default] trait
    pub fn build<T>(&mut self, name: String) -> ParameterEndpoint<T>
    where
        T: Clone,
        T: Default,
        T: FromOscType,
    {
        let path = format!("{}/{}", self.path, name);
        ParameterEndpoint::new(T::default(), path, self.store)
    }

    /// Build a new ParameterEndpoint using a given default value
    pub fn build_default<T>(&mut self, default: T, name: String) -> ParameterEndpoint<T>
    where
        T: Clone,
        T: FromOscType,
    {
        let path = format!("{}/{}", self.path, name);
        ParameterEndpoint::new(default, path, self.store)
    }

    /// Build a new ParameterEndpoint using a given default value containing multiple values.
    pub fn build_array_default<T>(
        &mut self,
        default: T,
        length: usize,
        name: String,
    ) -> ParameterEndpoint<T>
    where
        T: Clone,
        T: Default,
        T: FromOscType,
    {
        let path = format!("{}/{}", self.path, name);
        ParameterEndpoint::new_len(default, length, path, self.store)
    }

    /// sets the current parent path of the produced parameter
    pub fn path(&mut self, path: String) {
        self.path = path;
    }
}

///creating a handle binds a &[ParameterStore]  making further access easy using [From]
pub struct ParameterHandle<'a, T> {
    parameter: &'a ParameterEndpoint<T>,
    store: &'a ParameterStore,
}

impl<'a> From<ParameterHandle<'a, f32>> for f32 {
    fn from(s: ParameterHandle<'a, f32>) -> Self {
        s.parameter.get(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, f32>> for Vec<f32> {
    fn from(s: ParameterHandle<'a, f32>) -> Self {
        s.parameter.get_vec(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, i32>> for i32 {
    fn from(s: ParameterHandle<'a, i32>) -> Self {
        s.parameter.get(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, i32>> for Vec<i32> {
    fn from(s: ParameterHandle<'a, i32>) -> Self {
        s.parameter.get_vec(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, String>> for String {
    fn from(s: ParameterHandle<'a, String>) -> Self {
        s.parameter.get(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, i64>> for i64 {
    fn from(s: ParameterHandle<'a, i64>) -> Self {
        s.parameter.get(s.store)
    }
}

impl<'a> From<ParameterHandle<'a, f64>> for f64 {
    fn from(s: ParameterHandle<'a, f64>) -> Self {
        s.parameter.get(s.store)
    }
}

use nannou_osc::Type;

impl Into<Vec<oscq_rs::OscQueryParameter>> for &Parameter {
    fn into(self) -> Vec<oscq_rs::OscQueryParameter> {
        let mut vec = Vec::new();
        let addr = self.address.clone();
        for val in self.values.clone() {
            let conv = match val {
                Type::Int(i) => oscq_rs::osc::OscType::Int(i),
                Type::Float(f) => oscq_rs::osc::OscType::Float(f),
                Type::String(s) => oscq_rs::osc::OscType::String(s),
                Type::Blob(b) => oscq_rs::osc::OscType::Blob(b),
                Type::Time(i, j) => oscq_rs::osc::OscType::Time(oscq_rs::osc::OscTime {
                    seconds: i,
                    fractional: j,
                }),
                Type::Long(l) => oscq_rs::osc::OscType::Long(l),
                Type::Double(d) => oscq_rs::osc::OscType::Double(d),
                Type::Char(c) => oscq_rs::osc::OscType::Char(c),
                Type::Color(_r) => todo!(),
                Type::Midi(_m) => todo!(),
                Type::Bool(b) => oscq_rs::osc::OscType::Bool(b),
                Type::Nil => oscq_rs::osc::OscType::Nil,
                Type::Inf => oscq_rs::osc::OscType::Inf,
            };

            vec.push(oscq_rs::OscQueryParameter::new(addr.clone(), conv));
        }

        vec
    }
}

impl ParameterStore {
    pub fn create_query(&self, host_info: oscq_rs::OscHostInfo) -> oscq_rs::OSCNode {
        let mut root = oscq_rs::OSCNode::root(Some(Box::new(host_info)));
        //let mut root = oscq_rs::OSCNode::root(None);
        println!("create_query with {:?}", self.parameters);
        for par in &self.parameters {
            let all: Vec<oscq_rs::OscQueryParameter> = par.into();
            for p in all {
                println!("adding into query {:?}", p);
                root.add(p).unwrap();
            }
        }
        root
    }
}
