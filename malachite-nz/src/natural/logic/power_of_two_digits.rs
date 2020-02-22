use std::cmp::{min, Ordering};

use malachite_base::limbs::limbs_trailing_zero_limbs;
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{CheckedLogTwo, ModPowerOfTwo, Parity};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, FromOtherTypeSlice, WrappingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitBlockAccess, PowerOfTwoDigits};

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

macro_rules! power_of_two_digits_primitive {
    ($t: ident, $to_power_of_two_digits_asc_naive: ident) => {
        impl PowerOfTwoDigits<$t> for Natural {
            /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
            /// significant, where the base is a power of two. The base-2 logarithm of the base is
            /// specified. The type of each digit is `$u`, and `log_base` must be no larger than the
            /// width of `$u`. If `self` is 0, the `Vec` is empty; otherwise, it ends with a nonzero
            /// digit.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is zero.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{Two, Zero};
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&Natural::from(123u32), 3),
            ///     vec![3, 7, 1]
            /// );
            /// ```
            fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<$t> {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                let limbs = match *self {
                    Natural(Small(ref small)) => {
                        return PowerOfTwoDigits::<$t>::to_power_of_two_digits_asc(
                            small,
                            min(log_base, Limb::WIDTH),
                        )
                    }
                    Natural(Large(ref limbs)) => limbs,
                };
                let mut digits = Vec::new();
                if log_base == 1 {
                    let (last, init) = limbs.split_last().unwrap();
                    for limb in init {
                        for i in 0..Limb::WIDTH {
                            digits.push(if limb.get_bit(i) { 1 } else { 0 });
                        }
                    }
                    let mut last = *last;
                    while last != 0 {
                        digits.push(if last.odd() { 1 } else { 0 });
                        last >>= 1;
                    }
                } else if let Some(log_log_base) = log_base.checked_log_two() {
                    match log_log_base.cmp(&Limb::LOG_WIDTH) {
                        Ordering::Equal => {
                            digits.extend(limbs.iter().cloned().map($t::wrapping_from))
                        }
                        Ordering::Less => {
                            for mut limb in limbs.iter().cloned() {
                                let mask = (1 << log_base) - 1;
                                for _ in 0..1 << (Limb::LOG_WIDTH - log_log_base) {
                                    digits.push($t::wrapping_from(limb & mask));
                                    limb >>= log_base;
                                }
                            }
                        }
                        Ordering::Greater => digits.extend(
                            limbs
                                .chunks(1 << (log_log_base - Limb::LOG_WIDTH))
                                .map($t::from_other_type_slice),
                        ),
                    }
                } else {
                    let mut digit = 0;
                    let mut remaining_digit_bits = log_base;
                    for &limb in limbs {
                        let mut limb = limb;
                        let mut remaining_limb_bits = Limb::WIDTH;
                        while remaining_limb_bits != 0 {
                            let digit_index = log_base - remaining_digit_bits;
                            if remaining_limb_bits <= remaining_digit_bits {
                                digit |= $t::wrapping_from(limb) << digit_index;
                                remaining_digit_bits -= remaining_limb_bits;
                                remaining_limb_bits = 0;
                            } else {
                                digit |= $t::wrapping_from(limb)
                                    .mod_power_of_two(remaining_digit_bits)
                                    << digit_index;
                                limb >>= remaining_digit_bits;
                                remaining_limb_bits -= remaining_digit_bits;
                                remaining_digit_bits = 0;
                            }
                            if remaining_digit_bits == 0 {
                                digits.push(digit);
                                digit = 0;
                                remaining_digit_bits = log_base;
                            }
                        }
                    }
                    if digit != 0 {
                        digits.push(digit);
                    }
                }
                digits.truncate(digits.len() - limbs_trailing_zero_limbs(&digits));
                digits
            }

            /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
            /// significant, where the base is a power of two. The base-2 logarithm of the base is
            /// specified. The type of each digit is `$u`, and `log_base` must be no larger than the
            /// width of `$u`. If `self` is 0, the `Vec` is empty; otherwise, it begins with a
            /// nonzero digit.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$u`, or if `log_base` is zero.
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{Two, Zero};
            /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&Natural::from(123u32), 3),
            ///     vec![1, 7, 3]
            /// );
            /// ```
            fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<$t> {
                let mut digits = self.to_power_of_two_digits_asc(log_base);
                digits.reverse();
                digits
            }
        }

        impl Natural {
            pub fn $to_power_of_two_digits_asc_naive(&self, log_base: u64) -> Vec<$t> {
                assert_ne!(log_base, 0);
                if log_base > $t::WIDTH {
                    panic!(
                        "type {:?} is too small for a digit of width {}",
                        $t::NAME,
                        log_base
                    );
                }
                let mut digits = Vec::new();
                let mut n = self.clone();
                while n != 0 {
                    digits.push($t::exact_from((&n).mod_power_of_two(log_base)));
                    n >>= log_base;
                }
                digits
            }
        }
    };
}

power_of_two_digits_primitive!(u8, _to_power_of_two_digits_asc_u8_naive);
power_of_two_digits_primitive!(u16, _to_power_of_two_digits_asc_u16_naive);
power_of_two_digits_primitive!(u32, _to_power_of_two_digits_asc_u32_naive);
power_of_two_digits_primitive!(u64, _to_power_of_two_digits_asc_u64_naive);
power_of_two_digits_primitive!(u128, _to_power_of_two_digits_asc_u128_naive);
power_of_two_digits_primitive!(usize, _to_power_of_two_digits_asc_usize_naive);

impl PowerOfTwoDigits<Natural> for Natural {
    /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
    /// significant, where the base is a power of two. The base-2 logarithm of the base is
    /// specified. The type of each digit is `Natural`. If `self` is 0, the `Vec` is empty;
    /// otherwise, it ends with a nonzero digit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_asc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[3, 7, 1]"
    /// );
    /// ```
    fn to_power_of_two_digits_asc(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        if log_base <= Limb::WIDTH || self.limb_count() < 2 {
            return PowerOfTwoDigits::<Limb>::to_power_of_two_digits_asc(
                self,
                min(log_base, Limb::WIDTH),
            )
            .iter()
            .cloned()
            .map(Natural::from)
            .collect();
        }
        let limbs = match *self {
            Natural(Large(ref limbs)) => limbs,
            _ => unreachable!(),
        };
        let mut digits = Vec::new();
        if let Some(log_log_base) = log_base.checked_log_two() {
            assert!(log_log_base > Limb::LOG_WIDTH);
            digits.extend(
                limbs
                    .chunks(1 << (log_log_base - Limb::LOG_WIDTH))
                    .map(Natural::from_limbs_asc),
            );
        } else {
            let mut digit = Natural::ZERO;
            let mut remaining_digit_bits = log_base;
            for &limb in limbs {
                let mut limb = limb;
                let mut remaining_limb_bits = Limb::WIDTH;
                while remaining_limb_bits != 0 {
                    let digit_index = log_base - remaining_digit_bits;
                    if remaining_limb_bits <= remaining_digit_bits {
                        digit.assign_bits(
                            digit_index,
                            digit_index + remaining_limb_bits,
                            &Natural::from(limb),
                        );
                        remaining_digit_bits -= remaining_limb_bits;
                        remaining_limb_bits = 0;
                    } else {
                        digit.assign_bits(digit_index, log_base, &Natural::from(limb));
                        limb >>= remaining_digit_bits;
                        remaining_limb_bits -= remaining_digit_bits;
                        remaining_digit_bits = 0;
                    }
                    if remaining_digit_bits == 0 {
                        digits.push(digit);
                        digit = Natural::ZERO;
                        remaining_digit_bits = log_base;
                    }
                }
            }
            if digit != 0 {
                digits.push(digit);
            }
        }
        digits.truncate(digits.len() - limbs_trailing_zero_limbs(&digits));
        digits
    }

    /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
    /// significant, where the base is a power of two. The base-2 logarithm of the base is
    /// specified. The type of each digit is `Natural`. If `self` is 0, the `Vec` is empty;
    /// otherwise, it begins with a nonzero digit.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::logic::traits::PowerOfTwoDigits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOfTwoDigits::<Natural>::to_power_of_two_digits_desc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[1, 7, 3]"
    /// );
    /// ```
    fn to_power_of_two_digits_desc(&self, log_base: u64) -> Vec<Natural> {
        let mut digits = self.to_power_of_two_digits_asc(log_base);
        digits.reverse();
        digits
    }
}

impl Natural {
    pub fn _to_power_of_two_digits_asc_natural_naive(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        let mut digits = Vec::new();
        let mut n = self.clone();
        while n != 0 {
            digits.push((&n).mod_power_of_two(log_base));
            n >>= log_base;
        }
        digits
    }
}
