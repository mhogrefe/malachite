use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, DivRound, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ExactFrom, PowerOf2Digits, WrappingFrom,
};
use malachite_base::num::iterators::iterator_to_bit_chunks;
use malachite_base::num::logic::traits::{BitBlockAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_trailing_zeros;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::{min, Ordering};

impl Natural {
    pub fn _to_power_of_2_digits_asc_naive<T: CheckedFrom<Natural> + PrimitiveUnsigned>(
        &self,
        log_base: u64,
    ) -> Vec<T> {
        assert_ne!(log_base, 0);
        if log_base > T::WIDTH {
            panic!(
                "type {:?} is too small for a digit of width {}",
                T::NAME,
                log_base
            );
        }
        let digit_len = self
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling);
        let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
        let mut previous_index = 0;
        for _ in 0..digit_len {
            let index = previous_index + log_base;
            digits.push(T::exact_from(self.get_bits(previous_index, index)));
            previous_index = index;
        }
        digits
    }

    pub fn _from_power_of_2_digits_asc_naive<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural>
    where
        Natural: From<T>,
    {
        assert_ne!(log_base, 0);
        if log_base > T::WIDTH {
            panic!(
                "type {:?} is too small for a digit of width {}",
                T::NAME,
                log_base
            );
        }
        let mut n = Natural::ZERO;
        let mut previous_index = 0;
        for digit in digits {
            if digit.significant_bits() > log_base {
                return None;
            }
            let index = previous_index + log_base;
            n.assign_bits(previous_index, index, &Natural::from(digit));
            previous_index = index;
        }
        Some(n)
    }
}

fn _to_power_of_2_digits_asc<T: PrimitiveUnsigned>(x: &Natural, log_base: u64) -> Vec<T>
where
    Limb: PowerOf2Digits<T>,
{
    assert_ne!(log_base, 0);
    if log_base > T::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            T::NAME,
            log_base
        );
    }
    let limbs = match *x {
        Natural(Small(ref small)) => {
            return PowerOf2Digits::<T>::to_power_of_2_digits_asc(small, min(log_base, Limb::WIDTH))
        }
        Natural(Large(ref limbs)) => limbs,
    };
    let mut digits = iterator_to_bit_chunks(limbs.iter().cloned(), Limb::WIDTH, log_base)
        .map(Option::unwrap)
        .collect_vec();
    digits.truncate(digits.len() - slice_trailing_zeros(&digits));
    digits
}

fn _to_power_of_2_digits_desc<T>(x: &Natural, log_base: u64) -> Vec<T>
where
    Natural: PowerOf2Digits<T>,
{
    let mut digits = x.to_power_of_2_digits_asc(log_base);
    digits.reverse();
    digits
}

fn _from_power_of_2_digits_asc<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
    log_base: u64,
    digits: I,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
{
    assert_ne!(log_base, 0);
    if log_base > T::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            T::NAME,
            log_base
        );
    }
    let mut limbs = Vec::new();
    for digit in iterator_to_bit_chunks(digits, log_base, Limb::WIDTH) {
        limbs.push(digit?);
    }
    Some(Natural::from_owned_limbs_asc(limbs))
}

fn _from_power_of_2_digits_desc<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
    log_base: u64,
    digits: I,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
{
    assert_ne!(log_base, 0);
    if log_base > T::WIDTH {
        panic!(
            "type {:?} is too small for a digit of width {}",
            T::NAME,
            log_base
        );
    }
    let digits = digits.collect_vec();
    let mut limbs = Vec::new();
    for digit in iterator_to_bit_chunks(digits.iter().cloned().rev(), log_base, Limb::WIDTH) {
        limbs.push(digit?);
    }
    Some(Natural::from_owned_limbs_asc(limbs))
}

macro_rules! power_of_2_digits_unsigned {
    (
        $t: ident
    ) => {
        impl PowerOf2Digits<$t> for Natural {
            /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
            /// significant, where the base is a power of 2. The base-2 logarithm of the base is
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
            /// use malachite_base::num::conversion::traits::PowerOf2Digits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOf2Digits::<u64>::to_power_of_2_digits_asc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOf2Digits::<u16>::to_power_of_2_digits_asc(&Natural::from(123u32), 3),
            ///     vec![3, 7, 1]
            /// );
            /// ```
            #[inline]
            fn to_power_of_2_digits_asc(&self, log_base: u64) -> Vec<$t> {
                _to_power_of_2_digits_asc(self, log_base)
            }

            /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
            /// significant, where the base is a power of 2. The base-2 logarithm of the base is
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
            /// use malachite_base::num::conversion::traits::PowerOf2Digits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&Natural::ZERO, 6),
            ///     Vec::<u64>::new()
            /// );
            /// assert_eq!(
            ///     PowerOf2Digits::<u64>::to_power_of_2_digits_desc(&Natural::TWO, 6),
            ///     vec![2]
            /// );
            /// // 123_10 = 173_8
            /// assert_eq!(
            ///     PowerOf2Digits::<u16>::to_power_of_2_digits_desc(&Natural::from(123u32), 3),
            ///     vec![1, 7, 3]
            /// );
            /// ```
            #[inline]
            fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<$t> {
                _to_power_of_2_digits_desc(self, log_base)
            }

            /// Converts an iterator of digits into a `Natural`, where the base is a power of 2.
            /// The base-2 logarithm of the base is specified. The input digits are in ascending
            /// order: least- to most-significant. The type of each digit is `$t`, and `log_base`
            /// must be no larger than the width of `$t`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `digits.count()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$t`, if `log_base` is zero, or if
            /// some digit is greater than 2<sup>`log_base`.</sup>
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::PowerOf2Digits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_asc(6, [0u64, 0, 0].iter().cloned()).unwrap(),
            ///     0
            /// );
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_asc(6, [2u64, 0].iter().cloned()).unwrap(),
            ///     2
            /// );
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_asc(3, [3u16, 7, 1].iter().cloned()).unwrap(),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_power_of_2_digits_asc<I: Iterator<Item = $t>>(
                log_base: u64,
                digits: I,
            ) -> Option<Natural> {
                _from_power_of_2_digits_asc(log_base, digits)
            }

            /// Converts an iterator of digits into a `Natural`, where the base is a power of 2.
            /// The base-2 logarithm of the base is specified. The input digits are in descending
            /// order: most- to least-significant. The type of each digit is `$t`, and `log_base`
            /// must be no larger than the width of `$t`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `digits.count()`
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of `$t`, if `log_base` is zero, or if
            /// some digit is greater than 2<sup>`log_base`.</sup>
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::conversion::traits::PowerOf2Digits;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_desc(6, [0u64, 0, 0].iter().cloned()).unwrap(),
            ///     0
            /// );
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_desc(6, [0u64, 2].iter().cloned()).unwrap(),
            ///     2
            /// );
            /// assert_eq!(
            ///     Natural::from_power_of_2_digits_desc(3, [1u16, 7, 3].iter().cloned())
            ///         .unwrap(),
            ///     123
            /// );
            /// ```
            #[inline]
            fn from_power_of_2_digits_desc<I: Iterator<Item = $t>>(
                log_base: u64,
                digits: I,
            ) -> Option<Natural> {
                _from_power_of_2_digits_desc(log_base, digits)
            }
        }
    };
}
apply_to_unsigneds!(power_of_2_digits_unsigned);

impl PowerOf2Digits<Natural> for Natural {
    /// Returns a `Vec` containing the digits of `self` in ascending order: least- to most-
    /// significant, where the base is a power of 2. The base-2 logarithm of the base is
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
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[3, 7, 1]"
    /// );
    /// ```
    fn to_power_of_2_digits_asc(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        if log_base <= Limb::WIDTH || self.limb_count() < 2 {
            return PowerOf2Digits::<Limb>::to_power_of_2_digits_asc(
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
        if let Some(log_log_base) = log_base.checked_log_base_2() {
            assert!(log_log_base > Limb::LOG_WIDTH);
            digits.extend(
                limbs
                    .chunks(usize::power_of_2(log_log_base - Limb::LOG_WIDTH))
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
        digits.truncate(digits.len() - slice_trailing_zeros(&digits));
        digits
    }

    /// Returns a `Vec` containing the digits of `self` in descending order: most- to least-
    /// significant, where the base is a power of 2. The base-2 logarithm of the base is
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
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::ZERO, 6)
    ///     ),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::TWO, 6)
    ///     ),
    ///     "[2]"
    /// );
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     format!(
    ///         "{:?}",
    ///         PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::from(123u32), 3)
    ///     ),
    ///     "[1, 7, 3]"
    /// );
    /// ```
    fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<Natural> {
        let mut digits = self.to_power_of_2_digits_asc(log_base);
        digits.reverse();
        digits
    }

    /// Converts an iterator of digits into a `Natural`, where the base is a power of 2. The
    /// base-2 logarithm of the base is specified. The input digits are in ascending order: least-
    /// to most-significant. The type of each digit is `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `digits.count()` * `log_base`
    ///
    /// # Panics
    /// Panics if `log_base` is zero or if some digit is greater than 2<sup>`log_base`.</sup>
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_2_digits_asc(6, digits.iter().cloned()).unwrap(), 0);
    ///
    /// let digits = &[Natural::TWO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_2_digits_asc(6, digits.iter().cloned()).unwrap(), 2);
    ///
    /// let digits = &[Natural::from(3u32), Natural::from(7u32), Natural::ONE];
    /// assert_eq!(Natural::from_power_of_2_digits_asc(3, digits.iter().cloned()).unwrap(), 123);
    /// ```
    fn from_power_of_2_digits_asc<I: Iterator<Item = Natural>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural> {
        assert_ne!(log_base, 0);
        if let Some(log_log_base) = log_base.checked_log_base_2() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Ordering::Equal => {
                    for digit in digits {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        limbs.push(Limb::wrapping_from(digit));
                    }
                }
                Ordering::Less => {
                    for chunk in &digits.chunks(usize::wrapping_from(Limb::WIDTH >> log_log_base)) {
                        let mut limb = 0;
                        let mut offset = 0;
                        for digit in chunk {
                            if digit.significant_bits() > log_base {
                                return None;
                            }
                            limb |= Limb::wrapping_from(digit) << offset;
                            offset += log_base;
                        }
                        limbs.push(limb);
                    }
                }
                Ordering::Greater => {
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits {
                        offset += chunk_size;
                        for limb in digit.limbs() {
                            if limb.significant_bits() > log_base {
                                return None;
                            }
                            limbs.push(limb);
                        }
                        limbs.resize(offset, 0);
                    }
                }
            }
            Some(Natural::from_owned_limbs_asc(limbs))
        } else {
            let mut n = Natural::ZERO;
            let mut previous_index = 0;
            for digit in digits {
                if digit.significant_bits() > log_base {
                    return None;
                }
                let index = previous_index + log_base;
                n.assign_bits(previous_index, index, &digit);
                previous_index = index;
            }
            Some(n)
        }
    }

    /// Converts an iterator of digits into a `Natural`, where the base is a power of 2. The
    /// base-2 logarithm of the base is specified. The input digits are in descending order: least-
    /// to most-significant. The type of each digit is `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `digits.count()` * `log_base`
    ///
    /// # Panics
    /// Panics if `log_base` is zero or if some digit is greater than 2<sup>`log_base`.</sup>
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(Natural::from_power_of_2_digits_desc(6, digits.iter().cloned()).unwrap(), 0);
    ///
    /// let digits = &[Natural::ZERO, Natural::TWO];
    /// assert_eq!(Natural::from_power_of_2_digits_desc(6, digits.iter().cloned()).unwrap(), 2);
    ///
    /// let digits = &[Natural::ONE, Natural::from(7u32), Natural::from(3u32)];
    /// assert_eq!(Natural::from_power_of_2_digits_desc(3, digits.iter().cloned()).unwrap(), 123);
    /// ```
    fn from_power_of_2_digits_desc<I: Iterator<Item = Natural>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural> {
        assert_ne!(log_base, 0);
        if let Some(log_log_base) = log_base.checked_log_base_2() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Ordering::Equal => {
                    for digit in digits {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        limbs.push(Limb::wrapping_from(digit));
                    }
                    limbs.reverse();
                }
                Ordering::Less => {
                    let digits = digits.collect_vec();
                    for chunk in digits.rchunks(usize::wrapping_from(Limb::WIDTH >> log_log_base)) {
                        let mut limb = 0;
                        let mut offset = 0;
                        for digit in chunk.iter().rev() {
                            if digit.significant_bits() > log_base {
                                return None;
                            }
                            limb |= Limb::wrapping_from(digit) << offset;
                            offset += log_base;
                        }
                        limbs.push(limb);
                    }
                }
                Ordering::Greater => {
                    let digits = digits.collect_vec();
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits.iter().rev() {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        offset += chunk_size;
                        for limb in digit.limbs() {
                            limbs.push(limb);
                        }
                        limbs.resize(offset, 0);
                    }
                }
            }
            Some(Natural::from_owned_limbs_asc(limbs))
        } else {
            let digits = digits.collect_vec();
            let mut n = Natural::ZERO;
            let mut previous_index = 0;
            for digit in digits.iter().rev() {
                if digit.significant_bits() > log_base {
                    return None;
                }
                let index = previous_index + log_base;
                n.assign_bits(previous_index, index, digit);
                previous_index = index;
            }
            Some(n)
        }
    }
}

impl Natural {
    pub fn _to_power_of_2_digits_asc_natural_naive(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        let digit_len = self
            .significant_bits()
            .div_round(log_base, RoundingMode::Ceiling);
        let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
        let mut previous_index = 0;
        for _ in 0..digit_len {
            let index = previous_index + log_base;
            digits.push(self.get_bits(previous_index, index));
            previous_index = index;
        }
        digits
    }

    pub fn _from_power_of_2_digits_asc_natural_naive<I: Iterator<Item = Natural>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural> {
        assert_ne!(log_base, 0);
        let mut n = Natural::ZERO;
        let mut previous_index = 0;
        for digit in digits {
            if digit.significant_bits() > log_base {
                return None;
            }
            let index = previous_index + log_base;
            n.assign_bits(previous_index, index, &digit);
            previous_index = index;
        }
        Some(n)
    }
}
