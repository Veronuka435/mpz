mod evaluator;
mod provider;

#[cfg(test)]
mod tests {
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

        let mul_shares_alice: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();
        let mul_shares_bob: Vec<P256> = (0..count).map(|_| P256::rand(&mut rng)).collect();

        let (mut alice, mut bob) = ideal_ole_pair::<P256>();

        let (mut ctx_provider, mut ctx_evaluator) = test_st_executor(10);

        let add_shares_alice = alice
            .convert(&mut ctx_provider, mul_shares_alice.clone())
            .await
            .unwrap();
        let add_shares_bob = bob
            .convert(&mut ctx_evaluator, mul_shares_bob.clone())
            .await
            .unwrap();

        mul_shares_alice
            .iter()
            .zip(mul_shares_bob)
            .zip(add_shares_alice)
            .zip(add_shares_bob)
            .for_each(|(((&a, b), x), y)| assert_eq!(x + y, a * b));
    }
}
