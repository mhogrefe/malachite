use named::Named;
use num::arithmetic::traits::TrueCheckedShl;
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{LowMask, PowerOfTwoDigits, SignificantBits};

macro_rules! impl_power_of_two_digits {
    ($t:ident) => {
        macro_rules! impl_logic_traits_inner {
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
                    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$u> {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut digits = Vec::new();
                        if *self == 0 {
                        } else if self.significant_bits() <= log_base {
                            digits.push($u::wrapping_from(*self));
                        } else {
                            let mut x = *self;
                            let mask = $u::low_mask(log_base);
                            while x != 0 {
                                digits.push($u::wrapping_from(x) & mask);
                                x >>= log_base;
                            }
                        }
                        digits
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
                    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$u> {
                        let mut digits = self.to_power_of_two_digits_asc(log_base);
                        digits.reverse();
                        digits
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
                    fn from_power_of_two_digits_asc(log_base: u64, digits: &[$u]) -> $t {
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut n = 0;
                        for &digit in digits.iter().rev() {
                            assert!(digit.significant_bits() <= log_base);
                            let shifted = n
                                .true_checked_shl(log_base)
                                .expect("value represented by digits is too large");
                            n = shifted | $t::wrapping_from(digit);
                        }
                        n
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
                        assert_ne!(log_base, 0);
                        if log_base > $u::WIDTH {
                            panic!(
                                "type {:?} is too small for a digit of width {}",
                                $u::NAME,
                                log_base
                            );
                        }
                        let mut n = 0;
                        for &digit in digits {
                            assert!(digit.significant_bits() <= log_base);
                            let shifted = n
                                .true_checked_shl(log_base)
                                .expect("value represented by digits is too large");
                            n = shifted | $t::wrapping_from(digit);
                        }
                        n
                    }
                }
            };
        }

        impl_logic_traits_inner!(u8);
        impl_logic_traits_inner!(u16);
        impl_logic_traits_inner!(u32);
        impl_logic_traits_inner!(u64);
        impl_logic_traits_inner!(u128);
        impl_logic_traits_inner!(usize);
    };
}

impl_power_of_two_digits!(u8);
impl_power_of_two_digits!(u16);
impl_power_of_two_digits!(u32);
impl_power_of_two_digits!(u64);
impl_power_of_two_digits!(u128);
impl_power_of_two_digits!(usize);
