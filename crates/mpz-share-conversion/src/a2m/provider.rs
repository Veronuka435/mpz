use crate::{ShareConversionError, A2M};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeProvide;
use std::marker::PhantomData;

pub struct A2MProvider<C: Context, F: Field, T: OLEeProvide<C, F>> {
    provider: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeProvide<C, F>> A2MProvider<C, F, T> {
    pub fn new(provider: T) -> Self {
        Self {
            provider,
            field: PhantomData,
            context: PhantomData,
        }
    }
}

#[async_trait]
impl<C: Context, F: Field, T: OLEeProvide<C, F> + Send> A2M<C, F> for A2MProvider<C, F, T> {
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError> {
        todo!()
    }
}
