use crate::model::error::*;

pub trait Builder: Sized {
    type Type;

    fn creator() -> Self;

    fn editor() -> Self;

    fn builder_mode(&self) -> BuilderMode;

    fn create(self) -> Result<Self::Type>; 

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult>; 

    fn set(&mut self, raw_field: &str, raw_value: String) -> Result<()> {
        todo!()
    }
}

pub trait Build {
    type BuilderType: Builder;

    fn creator() -> Self::BuilderType {
        Self::BuilderType::creator()
    }

    fn editor() -> Self::BuilderType {
        Self::BuilderType::editor()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BuilderMode {
    Creator,
    Editor
}

#[derive(Debug)]
pub struct ModifyResult {
    fields_changed: Vec<&'static Field>
}

impl ModifyResult {
    pub fn new(fields_changed: Vec<&'static Field>) -> Self {
        Self {
            fields_changed,
        }
    }

    pub fn fields_changed(&self) -> &Vec<&'static Field> {
        &self.fields_changed
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FieldValueType {
    String,
    Integer,
    Float,
    Boolean,
    StringArray
}

#[derive(Debug)]
pub struct Field {
    name: &'static str,
    value_type: FieldValueType
}

impl Field {
    pub const fn new(name: &'static str, value_type: FieldValueType) -> Self {
        Self {
            name,
            value_type
        }
    }

    pub const fn name(&self) -> &'static str {
        &self.name
    }

    pub const fn value_type(&self) -> FieldValueType {
        self.value_type
    }
}

pub enum Optional<T> {
    Some(Option<T>),
    None(Option<T>)
}