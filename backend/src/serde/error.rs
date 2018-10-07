use crate::error::{Error, ErrorCode};
use serde::{ser::SerializeStruct, Serialize, Serializer};

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string())
    }
}
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("error_code", &self.error_code)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}
