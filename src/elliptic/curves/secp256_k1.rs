/*
    Cryptography utilities

    Copyright 2018 by Kzen Networks

    This file is part of Cryptography utilities library
    (https://github.com/KZen-networks/cryptography-utils)

    Cryptography utilities is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/cryptography-utils/blob/master/LICENSE>
*/

// Secp256k1 elliptic curve utility functions (se: https://en.bitcoin.it/wiki/Secp256k1).
//
// In Cryptography utilities, we need to manipulate low level elliptic curve members as Point
// in order to perform operation on them. As the library secp256k1 expose only SecretKey and
// PublicKey, we extend those with simple codecs.
//
// The Secret Key codec: BigInt <> SecretKey
// The Public Key codec: Point <> SecretKey
//
use BigInt;
use Point;

use arithmetic::traits::Converter;

use super::rand::thread_rng;
use super::secp256k1::constants::{CURVE_ORDER, GENERATOR_X, GENERATOR_Y, SECRET_KEY_SIZE};
use super::secp256k1::{PublicKey, Secp256k1, SecretKey};
use super::traits::{PublicKeyCodec, SecretKeyCodec};

pub type EC = Secp256k1;
pub type SK = SecretKey;
pub type PK = PublicKey;

impl SecretKeyCodec for SecretKey {
    fn new_random() -> SecretKey {
        SecretKey::new(&Secp256k1::without_caps(), &mut thread_rng())
    }

    fn from_big_int(n: &BigInt) -> SecretKey {
        let mut v = BigInt::to_vec(n);

        if v.len() < SECRET_KEY_SIZE {
            let mut template = vec![0; SECRET_KEY_SIZE - v.len()];
            template.extend_from_slice(&v);
            v = template;
        }

        SecretKey::from_slice(&Secp256k1::without_caps(), &v).unwrap()
    }

    fn to_big_int(&self) -> BigInt {
        BigInt::from(&self[0..self.len()])
    }

    fn get_q() -> BigInt {
        BigInt::from(CURVE_ORDER.as_ref())
    }
}

impl PublicKeyCodec for PublicKey {
    const KEY_SIZE: usize = 65;
    const HEADER_MARKER: usize = 4;

    fn get_base_point() -> Point {
        Point {
            x: BigInt::from(GENERATOR_X.as_ref()),
            y: BigInt::from(GENERATOR_Y.as_ref()),
        }
    }

    fn bytes_compressed_to_big_int(&self) -> BigInt {
        let serial = self.serialize();
        let result = BigInt::from(&serial[0..33]);
        return result;
    }

    fn to_point(&self) -> Point {
        PublicKey::from_key_slice(&self.serialize_uncompressed())
    }

    /// # Details
    /// This function serialized into a Point a Key in the uncompressed form.
    /// The expected size of the key is an array of 65 elements where:
    /// the first element is the header (4, uncompressed) and X, Y of length 32
    /// use PublicKey::to_key_slice to deserialize
    ///
    fn from_key_slice(key: &[u8]) -> Point {
        assert_eq!(key.len(), PublicKey::KEY_SIZE);
        let header = key[0] as usize;

        assert_eq!(header, PublicKey::HEADER_MARKER);

        // first 32 elements (without the header)
        let x = &key[1..key.len() / 2 + 1];

        // last 32 element
        let y = &key[(key.len() - 1) / 2 + 1..key.len()];

        Point {
            x: BigInt::from(x),
            y: BigInt::from(y),
        }
    }

    fn to_key(p: &Point) -> PublicKey {
        PublicKey::from_slice(&Secp256k1::without_caps(), &PublicKey::to_key_slice(p)).unwrap()
    }

    /// # Details
    /// This function deserialized a Point into a Key in the uncompressed form.
    /// use PublicKey::from_key_slice to serialize
    ///
    fn to_key_slice(p: &Point) -> Vec<u8> {
        let mut v = vec![PublicKey::HEADER_MARKER as u8];
        v.extend(BigInt::to_vec(&p.x));
        v.extend(BigInt::to_vec(&p.y));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicKeyCodec, SecretKeyCodec};

    use elliptic::curves::rand::thread_rng;
    use elliptic::curves::secp256k1::constants::{CURVE_ORDER, GENERATOR_X, GENERATOR_Y};
    use elliptic::curves::secp256k1::{PublicKey, Secp256k1, SecretKey};

    use BigInt;

    #[test]
    fn get_base_point_test() {
        let p = PublicKey::get_base_point();

        assert_eq!(p.x, BigInt::from(GENERATOR_X.as_ref()));
        assert_eq!(p.y, BigInt::from(GENERATOR_Y.as_ref()));
    }

    #[test]
    fn get_q_test() {
        let q = SecretKey::get_q();

        assert_eq!(q, BigInt::from(CURVE_ORDER.as_ref()));
    }

    #[test]
    fn from_secret_key_to_big_int() {
        let sk = SecretKey::new(&Secp256k1::without_caps(), &mut thread_rng());

        let sk_n = sk.to_big_int();
        let sk_back = SecretKey::from_big_int(&sk_n);

        assert_eq!(sk, sk_back);
    }

    #[test]
    #[should_panic]
    #[cfg_attr(rustfmt, rustfmt_skip)] // ignore fmt due to the slice comments
    fn from_invalid_header_key_slice_test() {
        let invalid_key: [u8; PublicKey::KEY_SIZE] = [
            1, // header
            // X
            231, 191, 194, 227, 183, 188, 238, 170, 206, 138, 20, 92, 140, 107, 83, 73,
            111, 170, 217, 69, 17, 64, 121, 65, 219, 97, 147, 181, 197, 239, 158, 56,
            // Y
            62, 15, 115, 56, 226, 122, 3, 180, 192, 166, 171, 137, 121, 23, 29, 225, 234, 220, 154,
            2, 157, 44, 73, 220, 31, 15, 55, 4, 244, 189, 7, 210,
        ];

        PublicKey::from_key_slice(&invalid_key);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)] // ignore fmt due to the slice comments
    fn from_valid_uncompressed_key_slice_to_key_test() {
        let valid_key: [u8; PublicKey::KEY_SIZE] = [
            4, // header
            // X
            54, 57, 149, 239, 162, 148, 175, 246, 254, 239, 75, 154, 152, 10, 82, 234, 224, 85,
            220, 40, 100, 57, 121, 30, 162, 94, 156, 135, 67, 74, 49, 179,
            // Y
            57, 236, 53, 162, 124, 149, 144, 168, 77, 74, 30, 72, 211, 229, 110, 111, 55, 96, 193,
            86, 227, 183, 152, 195, 155, 51, 247, 123, 113, 60, 228, 188,
        ];

        let p = PublicKey::from_key_slice(&valid_key);
        let k = PublicKey::to_key_slice(&p);
        assert_eq!(valid_key.len(), k.len());

        for (i, _elem) in k.iter().enumerate() {
            assert_eq!(valid_key[i], k[i]);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)] // ignore fmt due to the slice comments
    fn from_public_key_to_point_to_slice_to_key() {
        let slice = &[
            4, // header
            // X
            54, 57, 149, 239, 162, 148, 175, 246, 254, 239, 75, 154, 152, 10, 82, 234, 224, 85,
            220, 40, 100, 57, 121, 30, 162, 94, 156, 135, 67, 74, 49, 179,
            // Y
            57, 236, 53, 162, 124, 149, 144, 168, 77, 74, 30, 72, 211, 229, 110, 111, 55, 96, 193,
            86, 227, 183, 152, 195, 155, 51, 247, 123, 113, 60, 228, 188,
        ];

        let uncompressed_key = PublicKey::from_slice(
            &Secp256k1::without_caps(), slice).unwrap();
        let p = uncompressed_key.to_point();
        let key_slice = PublicKey::to_key_slice(&p);

        assert_eq!(slice.len(), key_slice.len());

        for (i, _elem) in key_slice.iter().enumerate() {
            assert_eq!(slice[i], key_slice[i]);
        }

        let expected_key = PublicKey::to_key(&p);
        assert_eq!(expected_key, uncompressed_key);
    }
}
