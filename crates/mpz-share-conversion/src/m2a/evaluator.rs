use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeEvaluate;
use std::marker::PhantomData;

use crate::{ShareConversionError, M2A};

pub struct M2AEvaluator<C: Context, F: Field, T: OLEeEvaluate<C, F>> {
    evaluator: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeEvaluate<C, F>> M2AEvaluator<C, F, T> {
    pub fn new(evaluator: T) -> Self {
        Self {
            evaluator,
            field: PhantomData,
            context: PhantomData,
        }
    }
}

#[async_trait]
impl<C: Context, F: Field, T: OLEeEvaluate<C, F> + Send> M2A<C, F> for M2AEvaluator<C, F, T> {
    async fn convert(
        &mut self,
        ctx: &mut C,
        mul_shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError> {
        self.evaluator
            .evaluate(ctx, mul_shares)
            .await
            .map_err(ShareConversionError::from)
    }
}
