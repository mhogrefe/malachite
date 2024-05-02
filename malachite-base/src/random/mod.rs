// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

/// A random seed used for reproducible testing.
pub const EXAMPLE_SEED: Seed = Seed::from_bytes([
    0xbf, 0x18, 0x11, 0xce, 0x15, 0xee, 0xfd, 0x20, 0x2f, 0xdf, 0x67, 0x6a, 0x6b, 0xba, 0xaf, 0x04,
    0xff, 0x71, 0xe0, 0xf8, 0x0b, 0x2a, 0xcf, 0x27, 0x85, 0xb3, 0x32, 0xc6, 0x20, 0x80, 0x5e, 0x36,
]);

/// A type representing a random seed.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Seed {
    pub bytes: [u8; 32],
}

impl Seed {
    /// Creates a `Seed` from a slice of 32 bytes.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::Seed;
    ///
    /// Seed::from_bytes([10; 32]);
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 32]) -> Seed {
        Seed { bytes }
    }

    /// Creates a PRNG from a slice of 32 bytes.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// EXAMPLE_SEED.get_rng();
    /// ```
    #[inline]
    pub fn get_rng(self) -> ChaCha20Rng {
        ChaCha20Rng::from_seed(self.bytes)
    }

    /// Uniformly generates a random `Seed`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::Seed;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// assert_eq!(
    ///     EXAMPLE_SEED.next(),
    ///     Seed::from_bytes([
    ///         0x71, 0xef, 0x45, 0x6c, 0xe4, 0xd2, 0xa8, 0xa1, 0x57, 0x20, 0x6e, 0x53, 0xbc, 0x22,
    ///         0x59, 0xee, 0x5d, 0xc8, 0x95, 0x73, 0xbd, 0x95, 0xd9, 0xc9, 0x75, 0x92, 0x1f, 0x48,
    ///         0x97, 0xa9, 0xae, 0x21
    ///     ])
    /// );
    /// ```
    #[inline]
    pub fn next(self) -> Seed {
        let mut bytes = [0; 32];
        self.get_rng().fill_bytes(&mut bytes);
        Seed::from_bytes(bytes)
    }

    /// Generates a new `Seed` from this seed. Passing different `key`s will, with very high
    /// probability, generate different seeds.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::Seed;
    /// use malachite_base::random::EXAMPLE_SEED;
    ///
    /// assert_eq!(
    ///     EXAMPLE_SEED.fork("first"),
    ///     Seed::from_bytes([
    ///         0x20, 0x18, 0x1, 0x3d, 0x96, 0x4d, 0x3e, 0x98, 0x10, 0x9d, 0x35, 0x75, 0x22, 0x89,
    ///         0xf7, 0xe9, 0xbe, 0x2f, 0x9c, 0x15, 0x95, 0x42, 0x1a, 0x79, 0x52, 0xf, 0x56, 0x9a,
    ///         0x7b, 0x8c, 0xd9, 0x34
    ///     ])
    /// );
    /// assert_eq!(
    ///     EXAMPLE_SEED.fork("second"),
    ///     Seed::from_bytes([
    ///         0xe0, 0x36, 0x88, 0x58, 0x6d, 0x67, 0x33, 0xea, 0xf2, 0x1c, 0x88, 0xf9, 0xe3, 0xbd,
    ///         0x52, 0xc0, 0xe5, 0xad, 0x61, 0x81, 0x21, 0xd8, 0x2f, 0x8e, 0xcd, 0xf, 0x89, 0x9d,
    ///         0x32, 0xc5, 0x35, 0x83
    ///     ])
    /// );
    /// ```
    pub fn fork(&self, key: &str) -> Seed {
        let mut seed = self.next();
        let forked_seed = &mut seed.bytes;
        let hash = Sha3_256::digest(key.as_bytes());
        for i in 0..32 {
            forked_seed[i] ^= hash[i];
        }
        seed.next()
    }
}
