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

use crate::{BigInt, BigUint, Sign};
#[cfg(feature = "quickcheck")]
use alloc::boxed::Box;
use alloc::vec::Vec;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// num-bigint draws whole digits at the platform digit size; `Limb` matches it.
fn biguint_from_limbs(limbs: Vec<Limb>) -> BigUint {
    Natural::from_owned_limbs_asc(limbs).into()
}

#[cfg(feature = "quickcheck")]
impl quickcheck::Arbitrary for BigUint {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Use arbitrary from Vec
        biguint_from_limbs(Vec::<Limb>::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        // Use shrinker from Vec
        Box::new(
            self.to_u64_digits()
                .shrink()
                .map(|limbs| biguint_from_limbs(limbs.into_iter().map(|x| x as Limb).collect())),
        )
    }
}

#[cfg(feature = "quickcheck")]
impl quickcheck::Arbitrary for BigInt {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let positive = bool::arbitrary(g);
        let sign = if positive { Sign::Plus } else { Sign::Minus };
        Self::from_biguint(sign, BigUint::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let sign = self.sign();
        let unsigned_shrink = self.magnitude().clone().shrink();
        Box::new(unsigned_shrink.map(move |x| Self::from_biguint(sign, x)))
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for BigUint {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(biguint_from_limbs(Vec::<Limb>::arbitrary(u)?))
    }

    fn arbitrary_take_rest(u: arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(biguint_from_limbs(Vec::<Limb>::arbitrary_take_rest(u)?))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        Vec::<Limb>::size_hint(depth)
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for BigInt {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let positive = bool::arbitrary(u)?;
        let sign = if positive { Sign::Plus } else { Sign::Minus };
        Ok(Self::from_biguint(sign, BigUint::arbitrary(u)?))
    }

    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let positive = bool::arbitrary(&mut u)?;
        let sign = if positive { Sign::Plus } else { Sign::Minus };
        Ok(Self::from_biguint(sign, BigUint::arbitrary_take_rest(u)?))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        arbitrary::size_hint::and(bool::size_hint(depth), BigUint::size_hint(depth))
    }
}
