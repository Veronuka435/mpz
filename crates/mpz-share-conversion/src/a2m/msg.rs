use enum_try_as_inner::EnumTryAsInner;
use mpz_fields::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, EnumTryAsInner, Serialize, Deserialize)]
#[derive_err(Debug)]
/// A message type for the A2M conversion protocol.
pub enum A2MMessage<F: Field> {
    /// Field elements sent by the provider.
    Message(Vec<F>),
}

impl<F: Field> From<A2MMessageError<F>> for std::io::Error {
    fn from(err: A2MMessageError<F>) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string())
    }
}
