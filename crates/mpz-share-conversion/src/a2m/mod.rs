//! This module provides a semi-honest implementation of the [`A2M`](crate::A2M)-protocol using
//! Oblivious Linear Evaluation with errors (OLEe). This protocol guarantees privacy and
//! correctness for semi-honest adversaries only. In the presence of a malicious adversary neither
//! correctness nor privacy holds.
//!
//! # A2M Protocol
//! Alice has a share `x` and Bob has a share `y`. They want to end up with shares `a` for Alice
//! and `b` for Bob such that x + y = a * b holds. They proceed as follows:
//!
//! - Alice samples a random field element `a`.
//! - Alice (OLE provider) and Bob (OLE evaluator) call F_(OLE)(a^(-1), y) -> c + d, so it holds
//!   that a^(-1) * y = -c + d.
//! - Alice sends m = a^(-1) * x - c to Bob.
//! - Alice returns a.
//! - Bob returns b = m + d = a^(-1) * x - c + d = a^(-1) * x + a^(-1) * y = a^(-1) * (x + y)
//!
//! Now it holds that
//! a * b = a * a(-1) * (x + y) = x + y

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
