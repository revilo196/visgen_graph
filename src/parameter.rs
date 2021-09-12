use crate::osc_convert::FromOscType;
///
/// # Parameters
/// Parameters are used to control behavior/ values, inside diffident modules
///
/// # Store
/// Parameter are stored in an central [ParameterStore]
///
use nannou_osc::{Message, Type};
use std::{collections::BTreeMap, fmt::Debug};

/// private struct that stores:
///   * **value**: current value(s) of the parameter
///   * **address**: full osc-address of the parameter
///
/// similar to [Message]
#[derive(Clone, Debug)]
struct Parameter {
    values: Vec<Type>,
    address: String,
}

impl Parameter {
    /// creates new Parameter with empty values and a given address.
    fn new(path: String) -> Self {
        Self {
            values: Vec::new(),
            address: path,
        }
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
#[derive(Default, Debug)]
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
    pub fn get_value(&self, token: ParameterIndex) -> Option<Vec<Type>> {
        self.parameters
            .get(token)
            .and_then(|f| Some(f.values.clone()))
    }

    /// read access using the path string
    ///
    /// returns [None] for non existing addresses
    pub fn get_path_value(&self, path: &str) -> Option<Vec<Type>> {
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
}


pub trait ParameterEnd<T> {
    fn len(&self) -> usize;
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
        let par = Parameter::new(path);
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
        Self { path: format!("/{}",path), store }
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
