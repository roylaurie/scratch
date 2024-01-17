use thiserror;
use anyhow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown field for {class}: {field}")]
    UnknownField{ class: &'static str, field: String },

    #[error("Required field for {class} was not set: {field}")]
    FieldNotSet{ class: &'static str, field: &'static str}
}

pub use anyhow::{bail, Result};