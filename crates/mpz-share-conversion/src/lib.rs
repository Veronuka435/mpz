//! This crate provides tooling for the conversion of shares of finite field elements.
//!
//! It implements 2PC protocols which allow to convert between additive and multiplicative shares.
//!
//! # A2M
//! Suppose Alice holds a share `x` and Bob holds a share `y`. Then the [`A2M`]-protocol (short for
//! additive-to-multiplicative) allows them to convert their shares in such a way that after the
//! protocol Alice holds `a` and Bob holds `b`, such that x + y = a * b holds.
//!
//! # M2A
//! Suppose Alice holds a share `a` and Bob holds a share `b`. Then the [`M2A`]-protocol (short for
//! multiplicative-to-additive) allows them to convert their shares in such a way that after the
//! protocol Alice holds `x` and Bob holds `y`, such that a * b = x + y holds.

#![deny(missing_docs, unreachable_pub, unused_must_use)]
#![deny(unsafe_code)]
#![deny(clippy::all)]

use async_trait::async_trait;
use mpz_common::Context;
use mpz_fields::Field;
use mpz_ole::OLEError;
use std::error::Error;
use thiserror::Error;

pub mod a2m;
pub mod m2a;

/// Allows to convert additive shares of finite field elements into multiplicative
/// shares
#[async_trait]
pub trait A2M<C: Context, F: Field> {
    /// Convert additive into multiplicative shares
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context, which provides IO channels.
    /// * `shares` - The additive input shares.
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError>;
}

/// Allows to convert multiplicative shares of finite field elements into additive
/// shares
#[async_trait]
pub trait M2A<C: Context, F: Field> {
    /// Convert multiplicative shares into additive shares
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context, which provides IO channels.
    /// * `shares` - The multiplicative input shares.
    async fn convert(
        &mut self,
        ctx: &mut C,
        shares: Vec<F>,
    ) -> Result<Vec<F>, ShareConversionError>;
}

/// An error for share conversion.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ShareConversionError {
    #[error(transparent)]
    OLE(#[from] OLEError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Message(Box<dyn Error + Send + 'static>),
}
