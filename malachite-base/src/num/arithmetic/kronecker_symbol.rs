use num::arithmetic::traits::{
    JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, NegAssign, Parity, UnsignedAbs,
};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::traits::Iverson;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::logic::traits::SignificantBits;
use std::mem::swap;

pub_test! {jacobi_symbol_unsigned_simple<T: PrimitiveUnsigned>(mut a: T, mut n: T) -> i8 {
    assert!(n > T::ZERO);
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

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 2` and `result_bit_1` is false.
pub_test! {jacobi_symbol_unsigned_fast_2_2<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    if a.even() {
        let two = (b >> 1u32) ^ b;
        loop {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign()
            }
            if a.odd() {
                break;
            }
        }
    }
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let two = (b >> 1u32) ^ b;
        loop {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign()
            }
            if a.odd() {
                break;
            }
        }
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
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
    let mut s = a.unsigned_abs().jacobi_symbol(n.unsigned_abs());
    if a < S::ZERO {
        let n_mod_4: u8 = n.mod_power_of_2(2).wrapping_into();
        if n_mod_4 == 3 {
            s.neg_assign()
        }
    }
    s
}

fn kronecker_symbol_unsigned<T: PrimitiveUnsigned>(a: T, b: T) -> i8 {
    if b == T::ZERO {
        i8::iverson(a == T::ONE)
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
        i8::iverson(a == S::ONE || a == S::NEGATIVE_ONE)
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
        /// where $T$ is time, $M$ is additional memory, and $n$ is
        /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// See [here](super::kronecker_symbol#jacobi_symbol).
    #[inline]
    fn jacobi_symbol(self, n: u128) -> i8 {
        if n.significant_bits() <= u64::WIDTH {
            jacobi_symbol_unsigned_simple(self, n)
        } else {
            jacobi_symbol_unsigned_fast_2_2(self, n)
        }
    }
}
