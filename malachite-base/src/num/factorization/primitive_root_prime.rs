// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2013 Mike Hansen
//
//      Copyright © 2024 Vincent Neiger
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::exhaustive::primitive_int_increasing_range;
use crate::num::factorization::factor::{
    MAX_FACTORS_IN_U8, MAX_FACTORS_IN_U16, MAX_FACTORS_IN_U32, MAX_FACTORS_IN_U64,
    MAX_FACTORS_IN_USIZE,
};
use crate::num::factorization::traits::{Factor, IsPrime, PrimitiveRootPrime};

// This is n_primitive_root_prime from ulong_extras/primitive_root_prime.c, FLINT 3.3.0-dev.
fn primitive_root_prime<T: PrimitiveUnsigned + IsPrime + Factor, const N: usize>(p: T) -> T
where
    <T as Factor>::FACTORS: Clone + IntoIterator<Item = (T, u8)>,
{
    assert!(p > T::ONE);
    if p == T::TWO {
        return T::ONE;
    }
    // compute (p - 1) / factors once and for all
    let mut exponents = [T::ZERO; N];
    let pm1 = p - T::ONE;
    for (e, (pf, _)) in exponents.iter_mut().zip((p - T::ONE).factor().into_iter()) {
        *e = pm1 / pf;
    }
    // try 2, 3, ..., p - 1
    let data = T::precompute_mod_pow_data(&p);
    'outer: for a in primitive_int_increasing_range(T::TWO, p) {
        for &exponent in &exponents {
            if exponent == T::ZERO {
                break;
            }
            if a.mod_pow_precomputed(exponent.wrapping_into(), p, &data) == T::ONE {
                continue 'outer;
            }
        }
        return a;
    }
    // If we haven't found a primitive root, it must be because p is not prime. Confirm this by
    // failing an assertion, which will produce an appropriate error message.
    assert!(p.is_prime());
    unreachable!()
}

macro_rules! impl_primitive_root_prime {
    ($t:ident, $n:ident) => {
        impl PrimitiveRootPrime for $t {
            type Output = $t;

            /// Given a prime number, computes a primitive root modulo that number. In other words,
            /// given a prime $p$, finds a generator of the cyclic group
            /// $(\mathbb{Z}/p\mathbb{Z})^\times$.
            ///
            /// If the input is not prime, this function's behavior is unspecified. Since primality
            /// checking can be expensive, the input is not tested for primality.
            ///
            /// Currently the smallest primitive root is returned, but there is no guarantee that
            /// this will be the case in the future.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(2^{n/4})$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is 0, and possibly panics if `self` is not prime.
            ///
            /// # Examples
            /// See [here](super::primitive_root_prime#primitive_root_prime).
            #[inline]
            fn primitive_root_prime(&self) -> $t {
                primitive_root_prime::<$t, $n>(*self)
            }
        }
    };
}
impl_primitive_root_prime!(u8, MAX_FACTORS_IN_U8);
impl_primitive_root_prime!(u16, MAX_FACTORS_IN_U16);
impl_primitive_root_prime!(u32, MAX_FACTORS_IN_U32);
impl_primitive_root_prime!(u64, MAX_FACTORS_IN_U64);
impl_primitive_root_prime!(usize, MAX_FACTORS_IN_USIZE);
