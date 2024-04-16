mod evaluator;
mod msg;
mod provider;

pub use evaluator::A2MEvaluator;
pub use msg::{A2MMessage, A2MMessageError};
pub use provider::A2MProvider;

#[cfg(test)]
mod tests {
    use crate::a2m::{A2MEvaluator, A2MProvider};
    use crate::A2M;
    use mpz_common::executor::test_st_executor;
    use mpz_core::{prg::Prg, Block};
    use mpz_fields::{p256::P256, UniformRand};
    use mpz_ole::ideal::ole::ideal_ole_pair;
    use rand::SeedableRng;

    #[tokio::test]
    async fn test_a2m() {
        let count = 12;
        let from_seed = Prg::from_seed(Block::ZERO);
        let mut rng = from_seed;

        let add_shares_provider: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();
        let add_shares_evaluator: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();

        let (ole_provider, ole_evaluator) = ideal_ole_pair::<P256>();

        let mut provider = A2MProvider::new(ole_provider);
        let mut evaluator = A2MEvaluator::new(ole_evaluator);

        let (mut ctx_provider, mut ctx_evaluator) = test_st_executor(10);

        let mul_shares_provider = provider
            .convert(&mut ctx_provider, add_shares_provider.clone())
            .await
            .unwrap();
        let mul_shares_evaluator = evaluator
            .convert(&mut ctx_evaluator, add_shares_evaluator.clone())
            .await
            .unwrap();

        add_shares_provider
            .iter()
            .zip(add_shares_evaluator)
            .zip(mul_shares_provider)
            .zip(mul_shares_evaluator)
            .for_each(|(((&x, y), a), b)| assert_eq!(x + y, a * b));
    }
}
