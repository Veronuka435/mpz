use crate::{
    msg::ROLEeMessage,
    role::rot::{into_rot_sink, into_rot_stream},
    Check, OLEError, RandomOLEeEvaluate,
};
use async_trait::async_trait;
use futures::SinkExt;
use mpz_core::ProtocolMessage;
use mpz_ole_core::role::ot::ROLEeEvaluator as ROLEeCoreEvaluator;
use mpz_ot::RandomOTReceiver;
use mpz_share_conversion_core::Field;
use utils_aio::{
    sink::IoSink,
    stream::{ExpectStreamExt, IoStream},
};

/// An evaluator for ROLEe.
pub struct ROLEeEvaluator<const N: usize, T: RandomOTReceiver<bool, [u8; N]>, F: Field> {
    rot_receiver: T,
    role_core: ROLEeCoreEvaluator<N, F>,
}

impl<const N: usize, T: RandomOTReceiver<bool, [u8; N]>, F: Field> ROLEeEvaluator<N, T, F> {
    /// Create a new [`ROLEeEvaluator`].
    pub fn new(rot_receiver: T) -> Self {
        // Check that the right N is used depending on the needed bit size of the field.
        let _: () = Check::<N, F>::IS_BITSIZE_CORRECT;

        Self {
            rot_receiver,
            role_core: ROLEeCoreEvaluator::default(),
        }
    }
}

impl<const N: usize, T: RandomOTReceiver<bool, [u8; N]>, F: Field> ProtocolMessage
    for ROLEeEvaluator<N, T, F>
{
    type Msg = ROLEeMessage<T::Msg, F>;
}

#[async_trait]
impl<const N: usize, T, F: Field> RandomOLEeEvaluate<F> for ROLEeEvaluator<N, T, F>
where
    T: RandomOTReceiver<bool, [u8; N]> + Send,
    Self: Send,
{
    async fn evaluate_random<
        Si: IoSink<Self::Msg> + Send + Unpin,
        St: IoStream<Self::Msg> + Send + Unpin,
    >(
        &mut self,
        sink: &mut Si,
        stream: &mut St,
        count: usize,
    ) -> Result<(Vec<F>, Vec<F>), OLEError> {
        let (fi, tfi): (Vec<bool>, Vec<[u8; N]>) = self
            .rot_receiver
            .receive_random(
                &mut into_rot_sink(sink),
                &mut into_rot_stream(stream),
                count * F::BIT_SIZE as usize,
            )
            .await?;

        let (ui, ek): (Vec<F>, Vec<F>) =
            stream.expect_next().await?.try_into_random_provider_msg()?;

        let dk: Vec<F> = self.role_core.sample_d(count);

        sink.send(ROLEeMessage::RandomEvaluatorMsg(dk.clone()))
            .await?;

        let (bk, yk) = self.role_core.generate_output(&fi, &tfi, &ui, &dk, &ek)?;

        Ok((bk, yk))
    }
}