use itertools::Itertools;
use num::arithmetic::traits::{CheckedAdd, CheckedLogBase2, CheckedMul, DivAssignMod};
use num::basic::traits::{One, Zero};
use num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, Digits, ExactFrom, PowerOf2Digits, WrappingFrom,
};
use std::cmp::Ord;

pub fn _unsigned_to_digits_asc_naive<
    T: Copy + DivAssignMod<T, ModOutput = T> + Eq + ExactFrom<U> + Zero,
    U: Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: U,
) -> Vec<U> {
    assert!(base > U::ONE);
    let mut digits = Vec::new();
    let mut remainder = *x;
    let base = T::exact_from(base);
    while remainder != T::ZERO {
        digits.push(U::wrapping_from(remainder.div_assign_mod(base)));
    }
    digits
}

fn _to_digits_asc<
    T: ConvertibleFrom<U>
        + Copy
        + DivAssignMod<T, ModOutput = T>
        + Eq
        + ExactFrom<U>
        + PowerOf2Digits<U>
        + Zero,
    U: CheckedLogBase2 + Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_asc(log_base)
    } else {
        _unsigned_to_digits_asc_naive(x, *base)
    }
}

fn _to_digits_desc<
    T: ConvertibleFrom<U>
        + Copy
        + DivAssignMod<T, ModOutput = T>
        + Eq
        + ExactFrom<U>
        + PowerOf2Digits<U>
        + Zero,
    U: CheckedLogBase2 + Copy + One + Ord + WrappingFrom<T>,
>(
    x: &T,
    base: &U,
) -> Vec<U> {
    assert!(T::convertible_from(*base));
    if *x == T::ZERO {
        Vec::new()
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_desc(log_base)
    } else {
        let mut digits = _unsigned_to_digits_asc_naive(x, *base);
        digits.reverse();
        digits
    }
}

fn _from_digits_asc<
    T: Digits<U> + PowerOf2Digits<U>,
    U: CheckedLogBase2 + Copy,
    I: Iterator<Item = U>,
>(
    base: &U,
    digits: I,
) -> Option<T> {
    if let Some(log_base) = base.checked_log_base_2() {
        T::from_power_of_2_digits_asc(log_base, digits)
    } else {
        let mut digits = digits.collect_vec();
        digits.reverse();
        T::from_digits_desc(base, digits.into_iter())
    }
}

fn _from_digits_desc<
    T: CheckedAdd<T, Output = T>
        + CheckedMul<T, Output = T>
        + Copy
        + Digits<U>
        + CheckedFrom<U>
        + Ord
        + PowerOf2Digits<U>
        + Zero,
    U: CheckedLogBase2 + Copy + One + Ord,
    I: Iterator<Item = U>,
>(
    base: &U,
    digits: I,
) -> Option<T> {
    assert!(*base > U::ONE);
    if let Some(log_base) = base.checked_log_base_2() {
        T::from_power_of_2_digits_desc(log_base, digits)
    } else {
        let base = T::checked_from(*base)?;
        let mut x = T::ZERO;
        for digit in digits {
            let digit = T::checked_from(digit)?;
            if digit >= base {
                return None;
            }
            x = x.checked_mul(base)?.checked_add(digit)?;
        }
        Some(x)
    }
}

macro_rules! impl_digits {
    ($t:ident) => {
        macro_rules! impl_digits_inner {
            ($u:ident) => {
                impl Digits<$u> for $t {
                    /// Returns a `Vec` containing the digits of `self` in ascending order (least-
                    /// to most-significant).
                    ///
                    /// The type of each digit is `$u`, and `base` must be convertible to $t$. If
                    /// `self` is 0, the `Vec` is empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_i = x.
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
                    /// Panics if `base` is less than 2 or greater than `$t::MAX`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn to_digits_asc(&self, base: &$u) -> Vec<$u> {
                        _to_digits_asc(self, base)
                    }

                    /// Returns a `Vec` containing the digits of `self` in descending order (most-
                    /// to least-significant).
                    ///
                    /// The type of each digit is `$u`, and `base` must be convertible to $t$. If
                    /// `self` is 0, the `Vec` is empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
                    /// $d_{k-1} \neq 0$, and
                    ///
                    /// $$
                    /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
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
                    /// Panics if `base` is less than 2 or greater than `$t::MAX`.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn to_digits_desc(&self, base: &$u) -> Vec<$u> {
                        _to_digits_desc(self, base)
                    }

                    /// Converts an iterator of digits into a value.
                    ///
                    /// The input digits are in ascending order (least- to most-significant). The
                    /// type of each digit is `$u`, and `base` must be no larger than `$t::MAX`. The
                    /// function returns `None` if the input represents a number that can't fit in
                    /// `$t`, or if `base` is greater than `$t::MAX`.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^id_i.
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
                    /// Panics if `base` is less than 2.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn from_digits_asc<I: Iterator<Item = $u>>(base: &$u, digits: I) -> Option<$t> {
                        _from_digits_asc(base, digits)
                    }

                    /// Converts an iterator of digits into a value.
                    ///
                    /// The input digits are in descending order (most- to least-significant). The
                    /// type of each digit is `$u`, and `base` must be no larger than `$t::MAX`. The
                    /// function returns `None` if the input represents a number that can't fit in
                    /// `$t`, or if `base` is greater than `$t::MAX`.
                    ///
                    /// $$
                    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^{k-i-1}d_i.
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
                    /// Panics if `base` is less than 2.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::conversion::digits::general_digits`
                    /// module.
                    #[inline]
                    fn from_digits_desc<I: Iterator<Item = $u>>(
                        base: &$u,
                        digits: I,
                    ) -> Option<$t> {
                        _from_digits_desc(base, digits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_digits_inner);
    };
}
apply_to_unsigneds!(impl_digits);
