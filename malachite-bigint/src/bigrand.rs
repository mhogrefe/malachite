// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the num-bigint library.
//
//      Copyright The Rust Project Developers
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

//! Randomization of big integers, matching num-bigint's value streams exactly.

use crate::Sign::{Minus, NoSign, Plus};
use crate::{BigInt, BigUint};
use alloc::vec;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::prelude::*;

/// A trait for sampling random big integers.
///
/// The `rand` feature must be enabled to use this.
pub trait RandBigInt {
    /// Generate a random [`BigUint`] of the given bit size.
    fn gen_biguint(&mut self, bit_size: u64) -> BigUint;

    /// Generate a random [`BigInt`] of the given bit size.
    fn gen_bigint(&mut self, bit_size: u64) -> BigInt;

    /// Generate a random [`BigUint`] less than the given bound. Fails when the bound is zero.
    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint;

    /// Generate a random [`BigUint`] within the given range. The lower bound is inclusive; the
    /// upper bound is exclusive. Fails when the upper bound is not greater than the lower bound.
    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint;

    /// Generate a random [`BigInt`] within the given range. The lower bound is inclusive; the upper
    /// bound is exclusive. Fails when the upper bound is not greater than the lower bound.
    fn gen_bigint_range(&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt;
}

fn gen_bits<R: Rng + ?Sized>(rng: &mut R, data: &mut [u32], rem: u64) {
    // `fill` is faster than many `gen::<u32>` calls
    rng.fill(data);
    if rem > 0 {
        let last = data.len() - 1;
        data[last] >>= 32 - rem;
    }
}

impl<R: Rng + ?Sized> RandBigInt for R {
    fn gen_biguint(&mut self, bit_size: u64) -> BigUint {
        // num-bigint always consumes the randomness as base-2^32 digits, regardless of its internal
        // digit size, so the generated values are stable; so do we.
        let (digits, rem) = bit_size.div_rem(&32);
        let len = (digits + u64::from(rem > 0))
            .to_usize()
            .expect("capacity overflow");
        let mut data = vec![0u32; len];
        gen_bits(self, &mut data, rem);
        BigUint::from_slice(&data)
    }

    fn gen_bigint(&mut self, bit_size: u64) -> BigInt {
        loop {
            // Generate a random BigUint...
            let biguint = self.gen_biguint(bit_size);
            // ...and then randomly assign it a Sign...
            let sign = if biguint.is_zero() {
                // ...except that if the BigUint is zero, we need to try again with probability 0.5.
                // This is because otherwise, the probability of generating a zero BigInt would be
                // double that of any other number.
                if self.r#gen() {
                    continue;
                }
                NoSign
            } else if self.r#gen() {
                Plus
            } else {
                Minus
            };
            return BigInt::from_biguint(sign, biguint);
        }
    }

    fn gen_biguint_below(&mut self, bound: &BigUint) -> BigUint {
        assert!(!bound.is_zero());
        let bits = bound.bits();
        loop {
            let n = self.gen_biguint(bits);
            if n < *bound {
                return n;
            }
        }
    }

    fn gen_biguint_range(&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint {
        assert!(*lbound < *ubound);
        if lbound.is_zero() {
            self.gen_biguint_below(ubound)
        } else {
            lbound + self.gen_biguint_below(&(ubound - lbound))
        }
    }

    fn gen_bigint_range(&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt {
        assert!(*lbound < *ubound);
        if lbound.is_zero() {
            BigInt::from(self.gen_biguint_below(ubound.magnitude()))
        } else if ubound.is_zero() {
            lbound + BigInt::from(self.gen_biguint_below(lbound.magnitude()))
        } else {
            let delta = ubound - lbound;
            lbound + BigInt::from(self.gen_biguint_below(delta.magnitude()))
        }
    }
}

/// The back-end implementing rand's [`UniformSampler`] for [`BigUint`].
#[derive(Clone, Debug)]
pub struct UniformBigUint {
    base: BigUint,
    len: BigUint,
}

impl UniformSampler for UniformBigUint {
    type X = BigUint;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();
        assert!(low < high);
        Self {
            len: high - low,
            base: low.clone(),
        }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();
        assert!(low <= high);
        Self::new(low, high + 1u32)
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        &self.base + rng.gen_biguint_below(&self.len)
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> Self::X
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        rng.gen_biguint_range(low.borrow(), high.borrow())
    }
}

impl SampleUniform for BigUint {
    type Sampler = UniformBigUint;
}

/// The back-end implementing rand's [`UniformSampler`] for [`BigInt`].
#[derive(Clone, Debug)]
pub struct UniformBigInt {
    base: BigInt,
    len: BigUint,
}

impl UniformSampler for UniformBigInt {
    type X = BigInt;

    #[inline]
    fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();
        assert!(low < high);
        Self {
            len: (high - low).into_parts().1,
            base: low.clone(),
        }
    }

    #[inline]
    fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low_b.borrow();
        let high = high_b.borrow();
        assert!(low <= high);
        Self::new(low, high + 1u32)
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        &self.base + BigInt::from(rng.gen_biguint_below(&self.len))
    }

    #[inline]
    fn sample_single<R: Rng + ?Sized, B1, B2>(low: B1, high: B2, rng: &mut R) -> Self::X
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        rng.gen_bigint_range(low.borrow(), high.borrow())
    }
}

impl SampleUniform for BigInt {
    type Sampler = UniformBigInt;
}

/// A random distribution for [`BigUint`] and [`BigInt`] values of a particular bit size.
///
/// The `rand` feature must be enabled to use this.
#[derive(Clone, Copy, Debug)]
pub struct RandomBits {
    bits: u64,
}

impl RandomBits {
    #[inline]
    pub const fn new(bits: u64) -> Self {
        Self { bits }
    }
}

impl Distribution<BigUint> for RandomBits {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BigUint {
        rng.gen_biguint(self.bits)
    }
}

impl Distribution<BigInt> for RandomBits {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BigInt {
        rng.gen_bigint(self.bits)
    }
}
