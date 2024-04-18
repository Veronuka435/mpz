use crate::{ShareConversionError, M2A};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeProvide;
use std::{fmt::Debug, marker::PhantomData};

/// A provider which implements multiplicative-to-additive share conversion.
///
/// It takes the role of the function provider during OLE.
pub struct M2AProvider<C: Context, F: Field, T: OLEeProvide<C, F>> {
    provider: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeProvide<C, F>> M2AProvider<C, F, T> {
    /// Creates a new [`M2A`] provider.
    ///
    /// # Arguments
    ///
    /// * `provider` - A provider which implements [`OLEeProvide`].
    pub fn new(provider: T) -> Self {
        Self {
            provider,
            field: PhantomData,
            context: PhantomData,
        }
    }
}

impl<C: Context, F: Field, T: OLEeProvide<C, F>> Debug for M2AProvider<C, F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ M2AProvider }}")
    }
}

#[async_trait]
impl<C: Context, F: Field, T: OLEeProvide<C, F> + Send> M2A<C, F> for M2AProvider<C, F, T> {
    async fn convert(&mut self, ctx: &mut C, ak: Vec<F>) -> Result<Vec<F>, ShareConversionError> {
        let mut ck = self.provider.provide(ctx, ak).await?;

        ck.iter_mut().for_each(|c| *c = -*c);

        // This is now what we refer to as `xk` in our protocol
        Ok(ck)
    }
}
