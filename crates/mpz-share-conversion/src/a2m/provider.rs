use crate::{a2m::msg::A2MMessage, ShareConversionError, A2M};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeProvide;
use rand::thread_rng;
use serio::{sink::SinkExt, Serialize};
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
impl<C: Context, F: Field + Serialize, T: OLEeProvide<C, F> + Send> A2M<C, F>
    for A2MProvider<C, F, T>
{
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError> {
        let rk: Vec<F> = {
            let mut rng = thread_rng();
            (0..shares.len()).map(|_| F::rand(&mut rng)).collect()
        };
        let xk = self.provider.provide(ctx, rk.clone()).await?;

        let mk: Vec<F> = rk
            .iter()
            .zip(shares)
            .zip(xk)
            .map(|((r, s), x)| *r * s + -x)
            .collect();

        let channel = ctx.io_mut();
        channel.send(A2MMessage::Message(mk)).await?;

        let out = rk.iter().map(|r| r.inverse()).collect();
        Ok(out)
    }
}
