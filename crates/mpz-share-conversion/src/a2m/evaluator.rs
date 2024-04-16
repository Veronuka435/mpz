use crate::{a2m::msg::A2MMessage, ShareConversionError, A2M};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeEvaluate;
use serio::{stream::IoStreamExt, Deserialize, Serialize};
use std::marker::PhantomData;

pub struct A2MEvaluator<C: Context, F: Field, T: OLEeEvaluate<C, F>> {
    evaluator: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeEvaluate<C, F>> A2MEvaluator<C, F, T> {
    pub fn new(evaluator: T) -> Self {
        Self {
            evaluator,
            field: PhantomData,
            context: PhantomData,
        }
    }
}

#[async_trait]
impl<C: Context, F: Field + Serialize + Deserialize, T: OLEeEvaluate<C, F> + Send> A2M<C, F>
    for A2MEvaluator<C, F, T>
{
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError> {
        let yk = self.evaluator.evaluate(ctx, shares).await?;

        let channel = ctx.io_mut();
        let mk: Vec<F> = channel
            .expect_next::<A2MMessage<F>>()
            .await?
            .try_into_message()
            .map_err(|err| ShareConversionError::Message(Box::new(err)))?;

        let out = yk.iter().zip(mk).map(|(y, m)| *y + m).collect();
        Ok(out)
    }
}
