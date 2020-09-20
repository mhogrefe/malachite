use std::ops::{BitOr, ShrAssign};

use num::arithmetic::traits::ArithmeticCheckedShl;
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{PowerOfTwoDigits, SignificantBits};

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
    T: ArithmeticCheckedShl<u64, Output = T> + BitOr<Output = T> + WrappingFrom<U> + Zero,
    U: PrimitiveInt,
>(
    log_base: u64,
    digits: &[U],
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
    for &digit in digits.iter().rev() {
        assert!(digit.significant_bits() <= log_base);
        let shifted = n
            .arithmetic_checked_shl(log_base)
            .expect("value represented by digits is too large");
        n = shifted | T::wrapping_from(digit);
    }
    n
}

fn _from_power_of_two_digits_desc<
    T: ArithmeticCheckedShl<u64, Output = T> + BitOr<Output = T> + WrappingFrom<U> + Zero,
    U: PrimitiveInt,
>(
    log_base: u64,
    digits: &[U],
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
    for &digit in digits {
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
                    /// Returns a `Vec` containing the digits of `self` in ascending order: least-
                    /// to most-significant, where the base is a power of two. The base-2 logarithm
                    /// of the base is specified. The type of each digit is `$u`, and `log_base`
                    /// must be no larger than the width of `$u`. If `self` is 0, the `Vec` is
                    /// empty; otherwise, it ends with a nonzero digit.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(n)
                    ///
                    /// where n = `self.significant_bits()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&0u8, 6),
                    ///     &[]
                    /// );
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&2u16, 6),
                    ///     &[2]
                    /// );
                    /// // 123_10 = 173_8
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&123u32, 3),
                    ///     &[3, 7, 1]
                    /// );
                    /// ```
                    #[inline]
                    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$u> {
                        _to_power_of_two_digits_asc(self, log_base)
                    }

                    /// Returns a `Vec` containing the digits of `self` in descending order: most-
                    /// to least-significant, where the base is a power of two. The base-2 logarithm
                    /// of the base is specified. The type of each digit is `$u`, and `log_base`
                    /// must be no larger than the width of `$u`. If `self` is 0, the `Vec` is
                    /// empty; otherwise, it begins with a nonzero digit.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(n)
                    ///
                    /// where n = `self.significant_bits()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is
                    /// zero.
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&0u8, 6),
                    ///     &[]
                    /// );
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&2u16, 6),
                    ///     &[2]
                    /// );
                    /// // 123_10 = 173_8
                    /// assert_eq!(
                    ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&123u32, 3),
                    ///     &[1, 7, 3]
                    /// );
                    /// ```
                    #[inline]
                    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$u> {
                        _to_power_of_two_digits_desc(self, log_base)
                    }

                    /// Converts a slice of digits into a value, where the base is a power of two.
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// ascending order: least- to most-significant. The type of each digit is `$u`,
                    /// and `log_base` must be no larger than the width of `$u`. The function panics
                    /// if the input represents a number that can't fit in $t.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// where n = `digits.len()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by $t, or if
                    /// some digit is greater than 2<sup>`log_base`.</sup>
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// let digits: &[u64] = &[0, 0, 0];
                    /// assert_eq!(u8::from_power_of_two_digits_asc(6, digits), 0);
                    ///
                    /// let digits: &[u64] = &[2, 0];
                    /// assert_eq!(u16::from_power_of_two_digits_asc(6, digits), 2);
                    ///
                    /// let digits: &[u16] = &[3, 7, 1];
                    /// assert_eq!(u32::from_power_of_two_digits_asc(3, digits), 123);
                    /// ```
                    #[inline]
                    fn from_power_of_two_digits_asc(log_base: u64, digits: &[$u]) -> $t {
                        _from_power_of_two_digits_asc(log_base, digits)
                    }

                    /// Converts a slice of digits into a value, where the base is a power of two.
                    /// The base-2 logarithm of the base is specified. The input digits are in
                    /// descending order: most- to least-significant. The type of each digit is
                    /// `$u`, and `log_base` must be no larger than the width of `$u`. The function
                    /// panics if the input represents a number that can't fit in $t.
                    ///
                    /// Time: worst case O(n)
                    ///
                    /// Additional memory: worst case O(1)
                    ///
                    /// where n = `digits.len()`
                    ///
                    /// # Panics
                    /// Panics if `log_base` is greater than the width of `$u`, if `log_base` is
                    /// zero, if the digits represent a value that isn't representable by $t, or if
                    /// some digit is greater than 2<sup>`log_base`.</sup>
                    ///
                    /// # Examples
                    /// ```
                    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
                    ///
                    /// let digits: &[u64] = &[0, 0, 0];
                    /// assert_eq!(u8::from_power_of_two_digits_desc(6, digits), 0);
                    ///
                    /// let digits: &[u64] = &[0, 2];
                    /// assert_eq!(u16::from_power_of_two_digits_desc(6, digits), 2);
                    ///
                    /// let digits: &[u16] = &[1, 7, 3];
                    /// assert_eq!(u32::from_power_of_two_digits_desc(3, digits), 123);
                    /// ```
                    fn from_power_of_two_digits_desc(log_base: u64, digits: &[$u]) -> $t {
                        _from_power_of_two_digits_desc(log_base, digits)
                    }

                    /// TODO doc
                    #[inline]
                    fn from_power_of_two_digit_iterator_asc<I: Iterator<Item = $u>>(
                        _log_base: u64,
                        _digits: I,
                    ) -> $t {
                        unimplemented!();
                    }

                    /// TODO doc
                    #[inline]
                    fn from_power_of_two_digit_iterator_desc<I: Iterator<Item = $u>>(
                        _log_base: u64,
                        _digits: I,
                    ) -> $t {
                        unimplemented!();
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_power_of_two_digits_inner);
    };
}
apply_to_unsigneds!(impl_power_of_two_digits);
