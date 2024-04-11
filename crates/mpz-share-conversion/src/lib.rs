use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEError;
use thiserror::Error;

mod a2m;
mod m2a;

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
}
