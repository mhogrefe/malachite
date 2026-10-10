// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use core::ops::{Shr, ShrAssign};

// The shift amount as a `u64`, or `None` if it is at least `T::WIDTH`, in which case every element
// becomes zero. `T::WIDTH` is at most 128, so it fits in every unsigned type.
fn shift_below_width<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(bits: U) -> Option<u64> {
    if bits >= U::exact_from(T::WIDTH) {
        None
    } else {
        Some(bits.exact_into())
    }
}

// Shifts every element right by `bits`, taking the floor.
fn shr_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    bits: U,
) -> UnsignedVector<T> {
    let Some(bits) = shift_below_width::<T, U>(bits) else {
        return UnsignedVector {
            elements: vec![T::ZERO; v.elements.len()],
        };
    };
    UnsignedVector {
        elements: v.elements.iter().map(|&x| x >> bits).collect(),
    }
}

// Shifts every element right by `bits`, taking the floor, in place.
fn shr_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(v: &mut UnsignedVector<T>, bits: U) {
    let Some(bits) = shift_below_width::<T, U>(bits) else {
        v.elements.fill(T::ZERO);
        return;
    };
    if bits != 0 {
        for x in &mut v.elements {
            *x >>= bits;
        }
    }
}

macro_rules! impl_shr_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned> Shr<$t> for UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2 and takes the floor),
            /// taking it by value. Every element is shifted, and the dimension is unchanged; if
            /// `bits` is at least `T::WIDTH`, every element is zero.
            ///
            /// $f(v, k) = \lfloor v/2^k \rfloor$. The floor is taken entrywise; to round
            /// differently, use
            /// [`entrywise_shr_round`](crate::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// There is no corresponding non-modular left shift, since it could overflow; use
            /// [`mod_power_of_2_shl`](crate::num::arithmetic::traits::ModPowerOf2Shl) or
            /// [`mod_shl`](crate::num::arithmetic::traits::ModShl) instead.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> UnsignedVector<T> {
                self >>= bits;
                self
            }
        }

        impl<T: PrimitiveUnsigned> Shr<$t> for &UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2 and takes the floor),
            /// taking it by reference. Every element is shifted, and the dimension is unchanged; if
            /// `bits` is at least `T::WIDTH`, every element is zero.
            ///
            /// $f(v, k) = \lfloor v/2^k \rfloor$. The floor is taken entrywise; to round
            /// differently, use
            /// [`entrywise_shr_round`](crate::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> UnsignedVector<T> {
                shr_ref(self, bits)
            }
        }

        impl<T: PrimitiveUnsigned> ShrAssign<$t> for UnsignedVector<T> {
            /// Right-shifts an [`UnsignedVector`] (divides it by a power of 2 and takes the floor),
            /// in place. Every element is shifted, and the dimension is unchanged; if `bits` is at
            /// least `T::WIDTH`, every element becomes zero.
            ///
            /// $v \gets \lfloor v/2^k \rfloor$. The floor is taken entrywise; to round differently,
            /// use [`entrywise_shr_round`](crate::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_unsigned);
