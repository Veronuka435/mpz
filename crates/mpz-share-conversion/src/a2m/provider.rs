use crate::{a2m::msg::A2MMessage, ShareConversionError, A2M};
use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEeProvide;
use rand::thread_rng;
use serio::{sink::SinkExt, Serialize};
use std::marker::PhantomData;

/// A provider which implements additive-to-multiplicative share conversion.
///
/// It takes the role of the function provider during OLE.
pub struct A2MProvider<C: Context, F: Field, T: OLEeProvide<C, F>> {
    provider: T,
    field: PhantomData<F>,
    context: PhantomData<C>,
}

impl<C: Context, F: Field, T: OLEeProvide<C, F>> A2MProvider<C, F, T> {
    /// Creates a new [`A2M`] provider.
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

#[async_trait]
impl<C: Context, F: Field + Serialize, T: OLEeProvide<C, F> + Send> A2M<C, F>
    for A2MProvider<C, F, T>
{
    async fn convert(&mut self, ctx: &mut C, xk: Vec<F>) -> Result<Vec<F>, ShareConversionError> {
        let a_inv_k: Vec<F> = {
            let mut rng = thread_rng();
            (0..xk.len())
                .map(|_| loop {
                    let a_inv = F::rand(&mut rng);
                    if a_inv != F::zero() {
                        break a_inv;
                    }
                })
                .collect()
        };
        let ck = self.provider.provide(ctx, a_inv_k.clone()).await?;

        let mk: Vec<F> = a_inv_k
            .iter()
            .zip(xk)
            .zip(ck)
            .map(|((&a_inv, x), c)| a_inv * x + -c)
            .collect();

        let channel = ctx.io_mut();
        channel.send(A2MMessage::Message(mk)).await?;

        let ak = a_inv_k.iter().map(|a_inv| a_inv.inverse()).collect();
        Ok(ak)
    }
}
