use crate::{a2m::msg::A2MMessage, ShareConversionError, A2M};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeEvaluate;
use serio::{stream::IoStreamExt, Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

/// An evaluator which implements additive-to-multiplicative share conversion.
///
/// It takes the role of the function evaluator during OLE.
pub struct A2MEvaluator<C: Context, F: Field, T: OLEeEvaluate<C, F>> {
    evaluator: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeEvaluate<C, F>> A2MEvaluator<C, F, T> {
    /// Creates a new [`A2M`] evaluator.
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

impl<C: Context, F: Field, T: OLEeEvaluate<C, F>> Debug for A2MEvaluator<C, F, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ A2MEvaluator }}")
    }
}

#[async_trait]
impl<C: Context, F: Field + Serialize + Deserialize, T: OLEeEvaluate<C, F> + Send> A2M<C, F>
    for A2MEvaluator<C, F, T>
{
    async fn convert(&mut self, ctx: &mut C, yk: Vec<F>) -> Result<Vec<F>, ShareConversionError> {
        let dk = self.evaluator.evaluate(ctx, yk).await?;

        let channel = ctx.io_mut();
        let mk: Vec<F> = channel
            .expect_next::<A2MMessage<F>>()
            .await?
            .try_into_message()
            .map_err(|err| ShareConversionError::Message(Box::new(err)))?;

        let out = mk.iter().zip(dk).map(|(m, d)| *m + d).collect();
        Ok(out)
    }
}
