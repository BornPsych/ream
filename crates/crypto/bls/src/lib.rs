//! The BLS (Boneh-Lynn-Shacham) cryptographic backend implementation is determined
//! at compile time via feature flags. Two implementations are supported:
//! - "supranational": Uses the supranational/blst library, optimized for performance
//! - "zkcrypto": Uses the zkcrypto/bls12_381 library implementation, optimized for zkVMs

pub mod aggregate_pubkey;
pub mod constants;
pub mod errors;
pub mod pubkey;
pub mod signature;
pub mod traits;

pub use aggregate_pubkey::AggregatePubKey;
pub use pubkey::PubKey;
pub use signature::BLSSignature;

#[cfg(feature = "supranational")]
pub mod supranational;
#[cfg(feature = "zkcrypto")]
pub mod zkcrypto;
