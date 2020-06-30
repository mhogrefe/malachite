use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

use random::StandardRand;

/// A wrapper around `ChaCha20Rng`'s seed type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Seed {
    pub bytes: [u8; 32],
}

impl StandardRand for Seed {
    /// Uniformly generates a random `Seed`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::{EXAMPLE_SEED, StandardRand};
    /// use malachite_base::random::seed::Seed;
    ///
    /// assert_eq!(
    ///     Seed::standard_gen(&mut EXAMPLE_SEED.get_rng()),
    ///     Seed::from_bytes([
    ///         0x71, 0xef, 0x45, 0x6c, 0xe4, 0xd2, 0xa8, 0xa1, 0x57, 0x20, 0x6e, 0x53, 0xbc, 0x22,
    ///         0x59, 0xee, 0x5d, 0xc8, 0x95, 0x73, 0xbd, 0x95, 0xd9, 0xc9, 0x75, 0x92, 0x1f, 0x48,
    ///         0x97, 0xa9, 0xae, 0x21
    ///     ])
    /// );
    /// ```
    #[inline]
    fn standard_gen(rng: &mut ChaCha20Rng) -> Seed {
        let mut bytes = [0; 32];
        rng.fill_bytes(&mut bytes);
        Seed::from_bytes(bytes)
    }
}

impl Seed {
    /// Creates a seed from a slice of 32 bytes.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::seed::Seed;
    ///
    /// Seed::from_bytes([10; 32]);
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: [u8; 32]) -> Seed {
        Seed { bytes }
    }

    /// Creates an RNG from a slice of 32 bytes.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
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

    #[inline]
    fn next(self) -> Seed {
        Seed::standard_gen(&mut self.get_rng())
    }

    /// Generates a new `Seed` from this seed. Passing a different `key` will, with very high
    /// probability, generate a different seed. Determining the initial seed from the resulting seed
    /// will be (cryptographically) difficult, even if the key is known.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::{EXAMPLE_SEED, StandardRand};
    /// use malachite_base::random::seed::Seed;
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
        let seed = self.clone().next();
        let hash = Sha3_256::digest(key.as_bytes());
        let mut forked_seed = [0; 32];
        for i in 0..32 {
            forked_seed[i] = seed.bytes[i] ^ hash[i];
        }
        Seed::from_bytes(forked_seed).next()
    }
}
