use crate::{ShareConversionError, M2A};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeEvaluate;
use std::marker::PhantomData;

/// An evaluator which implements multiplicative-to-additive share conversion.
///
/// It takes the role of the function evaluator during OLE.
pub struct M2AEvaluator<C: Context, F: Field, T: OLEeEvaluate<C, F>> {
    evaluator: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeEvaluate<C, F>> M2AEvaluator<C, F, T> {
    /// Creates a new [`M2A`] evaluator.
    ///
    /// # Arguments
    ///
    /// * `evaluator` - An evaluator which implements [`OLEeEvaluate`].
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
    async fn convert(&mut self, ctx: &mut C, bk: Vec<F>) -> Result<Vec<F>, ShareConversionError> {
        let dk = self
            .evaluator
            .evaluate(ctx, bk)
            .await
            .map_err(ShareConversionError::from)?;

        // Note that dk == yk
        Ok(dk)
    }
}
