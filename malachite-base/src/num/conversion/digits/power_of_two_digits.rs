use num::arithmetic::traits::ArithmeticCheckedShl;
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, PowerOfTwoDigits, WrappingFrom};
use num::logic::traits::SignificantBits;
use std::ops::{BitOr, BitOrAssign, ShrAssign};

fn _to_power_of_two_digits_asc<
    T: Copy + Eq + ShrAssign<u64> + SignificantBits + Zero,
    U: PrimitiveInt + WrappingFrom<T>,
>(
    x: &T,
    log_base: u64,
) -> Vec<U> {
    assert_ne!(log_base, 0);
    if log_base > U::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            U::NAME,
            log_base
        );
    }
    let mut digits = Vec::new();
    if *x == T::ZERO {
    } else if x.significant_bits() <= log_base {
        digits.push(U::wrapping_from(*x));
    } else {
        let mut x = *x;
        let mask = U::low_mask(log_base);
        while x != T::ZERO {
            digits.push(U::wrapping_from(x) & mask);
            x >>= log_base;
        }
    }
    digits
}

fn _to_power_of_two_digits_desc<
    T: Copy + Eq + ShrAssign<u64> + SignificantBits + Zero,
    U: PrimitiveInt + WrappingFrom<T>,
>(
    x: &T,
    log_base: u64,
) -> Vec<U> {
    let mut digits = _to_power_of_two_digits_asc(x, log_base);
    digits.reverse();
    digits
}

fn _from_power_of_two_digits_asc<
    T: ArithmeticCheckedShl<u64, Output = T>
        + BitOrAssign<T>
        + CheckedFrom<U>
        + WrappingFrom<U>
        + Zero,
    U: PrimitiveInt,
    I: Iterator<Item = U>,
>(
    log_base: u64,
    digits: I,
) -> T {
    assert_ne!(log_base, 0);
    if log_base > U::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            U::NAME,
            log_base
        );
    }
    let mut n = T::ZERO;
    let mut shift = 0;
    for digit in digits {
        assert!(digit.significant_bits() <= log_base);
        n |= T::checked_from(digit)
            .and_then(|d| d.arithmetic_checked_shl(shift))
            .expect("value represented by digits is too large");
        shift += log_base;
    }
    n
}

fn _from_power_of_two_digits_desc<
    T: ArithmeticCheckedShl<u64, Output = T> + BitOr<Output = T> + WrappingFrom<U> + Zero,
    U: PrimitiveInt,
    I: Iterator<Item = U>,
>(
    log_base: u64,
    digits: I,
) -> T {
    assert_ne!(log_base, 0);
    if log_base > U::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            U::NAME,
            log_base
        );
    }
    let mut n = T::ZERO;
    for digit in digits {
        assert!(digit.significant_bits() <= log_base);
        let shifted = n
            .arithmetic_checked_shl(log_base)
            .expect("value represented by digits is too large");
        n = shifted | T::wrapping_from(digit);
    }
    n
}

macro_rules! impl_power_of_two_digits {
    ($t:ident) => {
        macro_rules! impl_power_of_two_digits_inner {
            ($u:ident) => {
                impl PowerOfTwoDigits<$u> for $t {
                    /// Returns a `Vec` containing the digits of `self` in ascending order (least-
                    /// to most-significant) where the base is a power of two.
                    ///
                    /// The base-2 logarithm of the base is specified. The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. If `self` is
                    /// 0, the `Vec` is empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// $f(x, \ell) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < 2^\ell$ for all $i$,
                    /// $k=0$ or $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}2^{\ell i}d_i = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::power_of_two_digits`
                    /// module.
                    #[inline]
                    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$u> {
                        _to_power_of_two_digits_asc(self, log_base)
                    }

                    /// Returns a `Vec` containing the digits of `self` in descending order (most-
                    /// to least-significant) where the base is a power of two.
                    ///
                    /// The base-2 logarithm of the base is specified. The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. If `self` is
                    /// 0, the `Vec` is empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// $f(x, \ell) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < 2^\ell$ for all $i$,
                    /// $k=0$ or $d_0 \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}2^{\ell (k-i-1)}d_i = x.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(n)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is
                    /// `self.significant_bits()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::power_of_two_digits`
                    /// module.
                    #[inline]
                    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$u> {
                        _to_power_of_two_digits_desc(self, log_base)
                    }

                    /// Converts an iterator of digits into a value, where the base is a power of
                    /// two.
                    ///
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// ascending order (least- to most-significant). The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. The function
                    /// panics if the input represents a number that can't fit in `$t`.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, \ell) = \sum_{i=0}^{k-1}2^{\ell i}d_i.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by `$t`, or
                    /// if some digit is greater than $2^\ell$.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::power_of_two_digits`
                    /// module.
                    #[inline]
                    fn from_power_of_two_digits_asc<I: Iterator<Item = $u>>(
                        log_base: u64,
                        digits: I,
                    ) -> $t {
                        _from_power_of_two_digits_asc(log_base, digits)
                    }

                    /// Converts an iterator of digits into a value, where the base is a power of
                    /// two.
                    ///
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// descending order (most- to least-significant). The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. The function
                    /// panics if the input represents a number that can't fit in `$t`.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, \ell) = \sum_{i=0}^{k-1}2^{\ell (k-i-1)}d_i.
                    /// $$
                    ///
                    /// # Worst-case complexity
                    /// $T(n) = O(n)$
                    ///
                    /// $M(n) = O(1)$
                    ///
                    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by `$t`, or
                    /// if some digit is greater than $2^\ell$.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::power_of_two_digits`
                    /// module.
                    fn from_power_of_two_digits_desc<I: Iterator<Item = $u>>(
                        log_base: u64,
                        digits: I,
                    ) -> $t {
                        _from_power_of_two_digits_desc(log_base, digits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_power_of_two_digits_inner);
    };
}
apply_to_unsigneds!(impl_power_of_two_digits);
