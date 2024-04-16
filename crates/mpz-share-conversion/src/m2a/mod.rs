mod evaluator;
mod provider;

pub use evaluator::M2AEvaluator;
pub use provider::M2AProvider;

#[cfg(test)]
mod tests {
    use crate::m2a::{M2AEvaluator, M2AProvider};
    use crate::M2A;
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

        let mul_shares_provider: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();
        let mul_shares_evaluator: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();

        let (ole_provider, ole_evaluator) = ideal_ole_pair::<P256>();

        let mut provider = M2AProvider::new(ole_provider);
        let mut evaluator = M2AEvaluator::new(ole_evaluator);

        let (mut ctx_provider, mut ctx_evaluator) = test_st_executor(10);

        let add_shares_provider = provider
            .convert(&mut ctx_provider, mul_shares_provider.clone())
            .await
            .unwrap();
        let add_shares_evaluator = evaluator
            .convert(&mut ctx_evaluator, mul_shares_evaluator.clone())
            .await
            .unwrap();

        mul_shares_provider
            .iter()
            .zip(mul_shares_evaluator)
            .zip(add_shares_provider)
            .zip(add_shares_evaluator)
            .for_each(|(((&a, b), x), y)| assert_eq!(x + y, a * b));
    }
}
