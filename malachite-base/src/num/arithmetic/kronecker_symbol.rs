// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1996, 1998, 2000-2004, 2008, 2010 Free Software Foundation, Inc.
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008 Peter Shrimpton
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::fail_on_untested_path;
use crate::num::arithmetic::traits::{
    JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, NegAssign, Parity, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::SplitInHalf;
use crate::num::logic::traits::NotAssign;
use core::mem::swap;

pub_test! {jacobi_symbol_unsigned_simple<T: PrimitiveUnsigned>(mut a: T, mut n: T) -> i8 {
    assert_ne!(n, T::ZERO);
    assert!(n.odd());
    a %= n;
    let mut t = 1i8;
    while a != T::ZERO {
        while a.even() {
            a >>= 1;
            let r: u8 = n.mod_power_of_2(3).wrapping_into();
            if r == 3 || r == 5 {
                t.neg_assign();
            }
        }
        swap(&mut a, &mut n);
        if (a & n).get_bit(1) {
            t.neg_assign();
        }
        a %= n;
    }
    if n == T::ONE {
        t
    } else {
        0
    }
}}

// Computes (a / b) where b is odd, and a and b are otherwise arbitrary two-limb numbers.
//
// This is equivalent to `mpn_jacobi_2` from `mpn/jacobi_2.c`, GMP 6.2.1, where `JACOBI_2_METHOD ==
// 2` and `bit` is 0.
pub_test! {jacobi_symbol_unsigned_double_fast_2<T: PrimitiveUnsigned>(
    mut x_1: T,
    mut x_0: T,
    mut y_1: T,
    mut y_0: T,
) -> i8 {
    assert!(y_0.odd());
    if y_1 == T::ZERO && y_0 == T::ONE {
        // (x|1) = 1
        return 1;
    }
    let mut bit = false;
    if x_0 == T::ZERO {
        if x_1 == T::ZERO {
            // (0|y) = 0, y > 1
            return 0;
        }
        let c = x_1.trailing_zeros();
        if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
            bit.not_assign();
        }
        x_0 = y_0;
        y_0 = x_1 >> c;
        if y_0 == T::ONE {
            // (1|y) = 1
            return if bit { -1 } else { 1 };
        }
        x_1 = y_1;
        if (x_0 & y_0).get_bit(1) {
            bit.not_assign();
        }
    } else {
        if x_0.even() {
            let c = x_0.trailing_zeros();
            x_0 = (x_1 << (T::WIDTH - c)) | (x_0 >> c);
            x_1 >>= c;
            if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
                bit.not_assign();
            }
        }
        let mut skip_loop = false;
        if x_1 == T::ZERO {
            if y_1 == T::ZERO {
                assert!(y_0.odd());
                assert!(y_0 > T::ONE);
                let j = x_0.jacobi_symbol(y_0);
                return if bit { -j } else { j };
            }
                if (x_0 & y_0).get_bit(1) {
                    bit.not_assign();
                }
                swap(&mut x_0, &mut y_0);
                x_1 = y_1;
                skip_loop = true;
        }
        if !skip_loop {
            'outer: while y_1 != T::ZERO {
                // Compute (x|y)
                while x_1 > y_1 {
                    (x_1, x_0) = T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
                    if x_0 == T::ZERO {
                        let c = x_1.trailing_zeros();
                        if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
                            bit.not_assign();
                        }
                        x_0 = y_0;
                        y_0 = x_1 >> c;
                        x_1 = y_1;
                        if (x_0 & y_0).get_bit(1) {
                            bit.not_assign();
                        }
                        break 'outer;
                    }
                        let c = x_0.trailing_zeros();
                        if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
                            bit.not_assign();
                        }
                        x_0 = (x_1 << (T::WIDTH - c)) | (x_0 >> c);
                        x_1 >>= c;
                }
                if x_1 != y_1 {
                    if x_1 == T::ZERO {
                        if (x_0 & y_0).get_bit(1) {
                            bit.not_assign();
                        }
                        swap(&mut x_0, &mut y_0);
                        x_1 = y_1;
                        break;
                    }
                    if (x_0 & y_0).get_bit(1) {
                        bit.not_assign();
                    }
                    // Compute (y|x)
                    while y_1 > x_1 {
                        (y_1, y_0) = T::xx_sub_yy_to_zz(y_1, y_0, x_1, x_0);
                        if y_0 == T::ZERO {
                            let c = y_1.trailing_zeros();
                            if c.odd() & (x_0 ^ (x_0 >> 1)).get_bit(1) {
                                bit.not_assign();
                            }
                            y_0 = y_1 >> c;
                            if (x_0 & y_0).get_bit(1) {
                                bit.not_assign();
                            }
                            break 'outer;
                        }
                        let c = y_0.trailing_zeros();
                        if c.odd() & (x_0 ^ (x_0 >> 1)).get_bit(1) {
                            bit.not_assign();
                        }
                        y_0 = (y_1 << (T::WIDTH - c)) | (y_0 >> c);
                        y_1 >>= c;
                    }
                    if (x_0 & y_0).get_bit(1) {
                        bit.not_assign();
                    }
                }
                // Compute (x|y)
                if x_1 == y_1 {
                    if x_0 < y_0 {
                        swap(&mut x_0, &mut y_0);
                        if (x_0 & y_0).get_bit(1) {
                            bit.not_assign();
                        }
                    }
                    x_0 -= y_0;
                    if x_0 == T::ZERO {
                        return 0;
                    }
                    let c = x_0.trailing_zeros();
                    if c.odd() & (y_0 ^ (y_0 >> 1)).get_bit(1) {
                        bit.not_assign();
                    }
                    x_0 >>= c;
                    if x_0 == T::ONE {
                        return if bit { -1 } else { 1 };
                    }
                    swap(&mut x_0, &mut y_0);
                    if (x_0 & y_0).get_bit(1) {
                        bit.not_assign();
                    }
                    break;
                }
            }
        }
    }
    // Compute (x|y), with y a single limb.
    assert!(y_0.odd());
    if y_0 == T::ONE {
        // (x|1) = 1
        return if bit { -1 } else { 1 };
    }
    while x_1 != T::ZERO {
        x_1 -= if x_0 < y_0 { T::ONE } else { T::ZERO };
        x_0.wrapping_sub_assign(y_0);
        if x_0 == T::ZERO {
            if x_1 == T::ZERO {
                fail_on_untested_path(
                    "jacobi_symbol_unsigned_double_fast_2, x_1 == T::ZERO fourth time",
                );
                return 0;
            }
            let c = x_1.trailing_zeros();
            if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
                bit.not_assign();
            }
            x_0 = x_1 >> c;
            break;
        }
        let c = x_0.trailing_zeros();
        x_0 = (x_1 << (T::WIDTH - c)) | (x_0 >> c);
        x_1 >>= c;
        if c.odd() && (y_0 ^ (y_0 >> 1)).get_bit(1) {
            bit.not_assign();
        }
    }
    assert!(y_0.odd());
    assert!(y_0 > T::ONE);
    let j = x_0.jacobi_symbol(y_0);
    if bit {
        -j
    } else {
        j
    }
}}

fn jacobi_symbol_signed<
    U: PrimitiveUnsigned,
    S: ModPowerOf2<Output = U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    a: S,
    n: S,
) -> i8 {
    assert!(n > S::ZERO);
    assert!(n.odd());
    let s = a.unsigned_abs().jacobi_symbol(n.unsigned_abs());
    if a < S::ZERO && n.get_bit(1) {
        -s
    } else {
        s
    }
}

fn kronecker_symbol_unsigned<T: PrimitiveUnsigned>(a: T, b: T) -> i8 {
    if b == T::ZERO {
        i8::from(a == T::ONE)
    } else if a.even() && b.even() {
        0
    } else {
        let b_twos = b.trailing_zeros();
        let mut s = a.jacobi_symbol(b >> b_twos);
        if b_twos.odd() {
            let m: u32 = a.mod_power_of_2(3).wrapping_into();
            if m == 3 || m == 5 {
                s.neg_assign();
            }
        }
        s
    }
}

fn kronecker_symbol_signed<U: PrimitiveUnsigned, S: ModPowerOf2<Output = U> + PrimitiveSigned>(
    a: S,
    b: S,
) -> i8 {
    if b == S::ZERO {
        i8::from(a == S::ONE || a == S::NEGATIVE_ONE)
    } else if a.even() && b.even() {
        0
    } else {
        let b_twos = b.trailing_zeros();
        let mut s = a.jacobi_symbol((b >> b_twos).abs());
        if a < S::ZERO && b < S::ZERO {
            s.neg_assign();
        }
        if b_twos.odd() {
            let m: u32 = a.mod_power_of_2(3).wrapping_into();
            if m == 3 || m == 5 {
                s.neg_assign();
            }
        }
        s
    }
}

macro_rules! impl_symbols {
    ($u:ident, $s:ident) => {
        impl LegendreSymbol<$u> for $u {
            /// Computes the Legendre symbol of two numbers.
            ///
            /// This implementation is identical to that of [`JacobiSymbol`], since there is no
            /// computational benefit to requiring that the denominator be prime.
            ///
            /// $$
            /// f(x, y) = \left ( \frac{x}{y} \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Panics
            /// Panics if `n` is even.
            ///
            /// # Examples
            /// See [here](super::kronecker_symbol#legendre_symbol).
            #[inline]
            fn legendre_symbol(self, n: $u) -> i8 {
                self.jacobi_symbol(n)
            }
        }

        impl LegendreSymbol<$s> for $s {
            /// Computes the Legendre symbol of two numbers.
            ///
            /// This implementation is identical to that of [`JacobiSymbol`], since there is no
            /// computational benefit to requiring that the denominator be prime.
            ///
            /// $$
            /// f(x, y) = \left ( \frac{x}{y} \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Panics
            /// Panics if `n` is even or negative.
            ///
            /// # Examples
            /// See [here](super::kronecker_symbol#legendre_symbol).
            #[inline]
            fn legendre_symbol(self, n: $s) -> i8 {
                self.jacobi_symbol(n)
            }
        }

        impl JacobiSymbol<$s> for $s {
            /// Computes the Jacobi symbol of two numbers.
            ///
            /// $$
            /// f(x, y) = \left ( \frac{x}{y} \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Panics
            /// Panics if `n` is even.
            ///
            /// # Examples
            /// See [here](super::kronecker_symbol#jacobi_symbol).
            #[inline]
            fn jacobi_symbol(self, n: $s) -> i8 {
                jacobi_symbol_signed::<$u, $s>(self, n)
            }
        }

        impl KroneckerSymbol<$u> for $u {
            /// Computes the Kronecker symbol of two numbers.
            ///
            /// $$
            /// f(x, y) = \left ( \frac{x}{y} \right ).
            /// $$
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
            /// See [here](super::kronecker_symbol#kronecker_symbol).
            #[inline]
            fn kronecker_symbol(self, n: $u) -> i8 {
                kronecker_symbol_unsigned(self, n)
            }
        }

        impl KroneckerSymbol<$s> for $s {
            /// Computes the Kronecker symbol of two numbers.
            ///
            /// $$
            /// f(x, y) = \left ( \frac{x}{y} \right ).
            /// $$
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
            /// See [here](super::kronecker_symbol#kronecker_symbol).
            #[inline]
            fn kronecker_symbol(self, n: $s) -> i8 {
                kronecker_symbol_signed::<$u, $s>(self, n)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_symbols);

macro_rules! impl_jacobi_symbol_unsigned {
    ($u:ident) => {
        /// Computes the Jacobi symbol of two numbers.
        ///
        /// $$
        /// f(x, y) = \left ( \frac{x}{y} \right ).
        /// $$
        ///
        /// # Worst-case complexity
        /// $T(n) = O(n^2)$
        ///
        /// $M(n) = O(n)$
        ///
        /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
        /// other.significant_bits())`.
        ///
        /// # Panics
        /// Panics if `n` is even or negative.
        ///
        /// # Examples
        /// See [here](super::kronecker_symbol#jacobi_symbol).
        impl JacobiSymbol<$u> for $u {
            #[inline]
            fn jacobi_symbol(self, n: $u) -> i8 {
                jacobi_symbol_unsigned_simple(self, n)
            }
        }
    };
}
impl_jacobi_symbol_unsigned!(u8);
impl_jacobi_symbol_unsigned!(u16);
impl_jacobi_symbol_unsigned!(u32);
impl_jacobi_symbol_unsigned!(u64);
impl_jacobi_symbol_unsigned!(usize);

impl JacobiSymbol<u128> for u128 {
    /// Computes the Jacobi symbol of two `u128`s.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::kronecker_symbol#jacobi_symbol).
    #[inline]
    fn jacobi_symbol(self, n: u128) -> i8 {
        let (x_1, x_0) = self.split_in_half();
        let (y_1, y_0) = n.split_in_half();
        jacobi_symbol_unsigned_double_fast_2(x_1, x_0, y_1, y_0)
    }
}
