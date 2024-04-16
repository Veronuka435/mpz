use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEError;
use std::error::Error;
use thiserror::Error;

pub mod a2m;
pub mod m2a;

#[async_trait]
pub trait A2M<C: Context, F: Field> {
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError>;
}

#[async_trait]
pub trait M2A<C: Context, F: Field> {
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError>;
}

#[derive(Debug, Error)]
pub enum ShareConversionError {
    #[error(transparent)]
    OLE(#[from] OLEError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Message(Box<dyn Error + Send + 'static>),
}
