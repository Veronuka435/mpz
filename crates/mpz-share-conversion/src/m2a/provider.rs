use std::marker::PhantomData;

use crate::{ShareConversionError, M2A};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeProvide;

pub struct M2AProvider<C: Context, F: Field, T: OLEeProvide<C, F>> {
    provider: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeProvide<C, F>> M2AProvider<C, F, T> {
    pub fn new(provider: T) -> Self {
        Self {
            provider,
            field: PhantomData,
            context: PhantomData,
        }
    }
}

#[async_trait]
impl<C: Context, F: Field, T: OLEeProvide<C, F> + Send> M2A<C, F> for M2AProvider<C, F, T> {
    async fn convert(
        &mut self,
        ctx: &mut C,
        mul_shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError> {
        let mut add_shares = self.provider.provide(ctx, mul_shares).await?;
        add_shares.iter_mut().for_each(|share| *share = -*share);

        Ok(add_shares)
    }
}
