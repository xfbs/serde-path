use serde::de::DeserializeOwned;
use std::sync::Arc;

mod de;
mod error;
pub mod util;
pub use error::*;

pub fn from_path<T: DeserializeOwned>(
    template: &str,
    path: &str,
) -> Result<T, PathDeserializationError> {
    todo!()
}

pub fn from_params<T: DeserializeOwned>(
    params: &[(Arc<str>, util::PercentDecodedStr)],
) -> Result<T, PathDeserializationError> {
    T::deserialize(de::PathDeserializer::new(params))
}
