/*
    This file is part of Curv library
    Copyright 2018 by Kzen Networks
    (https://github.com/KZen-networks/curv)
    License MIT: <https://github.com/KZen-networks/curv/blob/master/LICENSE>
*/

#[macro_use]
extern crate serde_derive;
extern crate blake2b_simd;
extern crate crypto;
extern crate hex;
extern crate merkle;
extern crate serde;
extern crate sha3;
extern crate zeroize;

#[cfg(feature = "ecc")]
pub mod elliptic;

#[cfg(feature = "ec_secp256k1")]
mod secp256k1instance {
    pub use elliptic::curves::secp256_k1::FE;
    pub use elliptic::curves::secp256_k1::GE;
    pub use elliptic::curves::secp256_k1::PK;
    pub use elliptic::curves::secp256_k1::SK;
}

#[cfg(feature = "ec_secp256k1")]
pub use self::secp256k1instance::*;

#[cfg(feature = "ec_ristretto")]
mod curveristrettoinstance {
    pub use elliptic::curves::curve_ristretto::FE;
    pub use elliptic::curves::curve_ristretto::GE;
    pub use elliptic::curves::curve_ristretto::PK;
    pub use elliptic::curves::curve_ristretto::SK;
}

#[cfg(feature = "ec_ristretto")]
pub use self::curveristrettoinstance::*;

#[cfg(feature = "ec_ed25519")]
mod ed25519instance {
    pub use elliptic::curves::ed25519::FE;
    pub use elliptic::curves::ed25519::GE;
    pub use elliptic::curves::ed25519::PK;
    pub use elliptic::curves::ed25519::SK;
}

#[cfg(feature = "ec_ed25519")]
pub use self::ed25519instance::*;

#[cfg(feature = "ec_jubjub")]
mod jubjubinstance {
    pub use elliptic::curves::curve_jubjub::FE;
    pub use elliptic::curves::curve_jubjub::GE;
    pub use elliptic::curves::curve_jubjub::PK;
    pub use elliptic::curves::curve_jubjub::SK;
}

#[cfg(feature = "ec_jubjub")]
pub use self::jubjubinstance::*;

#[cfg(feature = "rust-gmp")]
pub mod arithmetic;
#[cfg(feature = "rust-gmp")]
pub use arithmetic::big_gmp::BigInt;

#[cfg(feature = "ecc")]
pub mod cryptographic_primitives;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ErrorKey {
    InvalidPublicKey,
}

pub enum ErrorSS {
    VerifyShareError,
}
