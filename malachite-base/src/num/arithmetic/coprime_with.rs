// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::CoprimeWith;
use crate::num::basic::unsigneds::PrimitiveUnsigned;

pub_test! {coprime_with_check_2<T: PrimitiveUnsigned>(x: T, y: T) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == T::ONE
}}

#[cfg(feature = "test_build")]
pub fn coprime_with_check_2_3<T: PrimitiveUnsigned>(x: T, y: T) -> bool {
    (x.odd() || y.odd())
        && (!x.divisible_by(T::from(3u8)) || !y.divisible_by(T::from(3u8)))
        && x.gcd(y) == T::ONE
}

#[cfg(feature = "test_build")]
pub fn coprime_with_check_2_3_5<T: PrimitiveUnsigned>(x: T, y: T) -> bool {
    if x.even() && y.even() {
        false
    } else {
        let c15 = T::from(15u8);
        let c3 = T::from(3u8);
        let c6 = T::from(6u8);
        let c9 = T::from(9u8);
        let c12 = T::from(12u8);
        let c5 = T::from(5u8);
        let c10 = T::from(10u8);
        let x15 = x % c15;
        let y15 = y % c15;
        if (x15 == T::ZERO || x15 == c3 || x15 == c6 || x15 == c9 || x15 == c12)
            && (y15 == T::ZERO || y15 == c3 || y15 == c6 || y15 == c9 || y15 == c12)
        {
            return false;
        }
        if (x15 == T::ZERO || x15 == c5 || x15 == c10)
            && (y15 == T::ZERO || y15 == c5 || y15 == c10)
        {
            return false;
        }
        x.gcd(y) == T::ONE
    }
}

macro_rules! impl_coprime_with {
    ($t:ident) => {
        impl CoprimeWith<$t> for $t {
            /// Returns whether two numbers are coprime; that is, whether they have no common factor
            /// other than 1.
            ///
            /// Every number is coprime with 1. No number is coprime with 0, except 1.
            ///
            /// $f(x, y) = (\gcd(x, y) = 1)$.
            ///
            /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::coprime_with#coprime_with).
            #[inline]
            fn coprime_with(self, other: $t) -> bool {
                coprime_with_check_2(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_coprime_with);
