// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::{min, Ordering::*};
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, DivRound, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits, WrappingFrom};
use malachite_base::num::iterators::iterator_to_bit_chunks;
use malachite_base::num::logic::traits::{BitBlockAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_trailing_zeros;

impl Natural {
    pub_test! {to_power_of_2_digits_asc_naive<
        T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
    >(
        &self,
        log_base: u64,
    ) -> Vec<T> {
        assert_ne!(log_base, 0);
            assert!(log_base <= T::WIDTH,
                "type {:?} is too small for a digit of width {}",
                T::NAME,
                log_base
            );
        let digit_len = self
            .significant_bits()
            .div_round(log_base, Ceiling).0;
        let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
        let mut previous_index = 0;
        for _ in 0..digit_len {
            let index = previous_index + log_base;
            digits.push(T::exact_from(&self.get_bits(previous_index, index)));
            previous_index = index;
        }
        digits
    }}

    pub_test! {from_power_of_2_digits_asc_naive<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural>
    where
        Natural: From<T>,
    {
        assert_ne!(log_base, 0);
            assert!(log_base <= T::WIDTH,
                "type {:?} is too small for a digit of width {}",
                T::NAME,
                log_base
            );
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
    }}
}

fn to_power_of_2_digits_asc_nz<T: PrimitiveUnsigned>(x: &Natural, log_base: u64) -> Vec<T>
where
    Limb: PowerOf2Digits<T>,
{
    assert_ne!(log_base, 0);
    assert!(
        log_base <= T::WIDTH,
        "type {:?} is too small for a digit of width {}",
        T::NAME,
        log_base
    );
    let limbs = match *x {
        Natural(Small(ref small)) => {
            return PowerOf2Digits::<T>::to_power_of_2_digits_asc(small, min(log_base, Limb::WIDTH))
        }
        Natural(Large(ref limbs)) => limbs,
    };
    let mut digits = iterator_to_bit_chunks(limbs.iter().copied(), Limb::WIDTH, log_base)
        .map(Option::unwrap)
        .collect_vec();
    digits.truncate(digits.len() - slice_trailing_zeros(&digits));
    digits
}

fn to_power_of_2_digits_desc_nz<T>(x: &Natural, log_base: u64) -> Vec<T>
where
    Natural: PowerOf2Digits<T>,
{
    let mut digits = x.to_power_of_2_digits_asc(log_base);
    digits.reverse();
    digits
}

fn from_power_of_2_digits_asc_nz<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
    log_base: u64,
    digits: I,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
{
    assert_ne!(log_base, 0);
    assert!(
        log_base <= T::WIDTH,
        "type {:?} is too small for a digit of width {}",
        T::NAME,
        log_base
    );
    let mut limbs = Vec::new();
    for digit in iterator_to_bit_chunks(digits, log_base, Limb::WIDTH) {
        limbs.push(digit?);
    }
    Some(Natural::from_owned_limbs_asc(limbs))
}

fn from_power_of_2_digits_desc_nz<T: PrimitiveUnsigned, I: Iterator<Item = T>>(
    log_base: u64,
    digits: I,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
{
    assert_ne!(log_base, 0);
    assert!(
        log_base <= T::WIDTH,
        "type {:?} is too small for a digit of width {}",
        T::NAME,
        log_base
    );
    let digits = digits.collect_vec();
    let mut limbs = Vec::new();
    for digit in iterator_to_bit_chunks(digits.iter().copied().rev(), log_base, Limb::WIDTH) {
        limbs.push(digit?);
    }
    Some(Natural::from_owned_limbs_asc(limbs))
}

macro_rules! power_of_2_digits_unsigned {
    (
        $t: ident
    ) => {
        impl PowerOf2Digits<$t> for Natural {
            /// Returns a [`Vec`] containing the base-$2^k$ digits of a [`Natural`] in ascending
            /// order: least- to most-significant.
            ///
            /// The base-2 logarithm of the base is specified. Each digit has primitive integer
            /// type, and `log_base` must be no larger than the width of that type. If the
            /// [`Natural`] is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
            ///
            /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$ or
            /// $d_{n-1} \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{n-1}2^{ki}d_i = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of the digit type, or if `log_base`
            /// is zero.
            ///
            /// # Examples
            /// See [here](super::power_of_2_digits#to_power_of_2_digits_asc).
            #[inline]
            fn to_power_of_2_digits_asc(&self, log_base: u64) -> Vec<$t> {
                to_power_of_2_digits_asc_nz(self, log_base)
            }

            /// Returns a [`Vec`] containing the base-$2^k$ digits of a [`Natural`] in descending
            /// order: most- to least-significant.
            ///
            /// The base-2 logarithm of the base is specified. Each digit has primitive integer
            /// type, and `log_base` must be no larger than the width of that type. If the
            /// [`Natural`] is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
            ///
            /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$ or $d_0
            /// \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `log_base` is greater than the width of the digit type, or if `log_base`
            /// is zero.
            ///
            /// # Examples
            /// See [here](super::power_of_2_digits#to_power_of_2_digits_desc).
            #[inline]
            fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<$t> {
                to_power_of_2_digits_desc_nz(self, log_base)
            }

            /// Converts an iterator of base-$2^k$ digits into a [`Natural`].
            ///
            /// The base-2 logarithm of the base is specified. The input digits are in ascending
            /// order: least- to most-significant. Each digit has primitive integer type, and
            /// `log_base` must be no larger than the width of that type.
            ///
            /// If some digit is greater than $2^k$, `None` is returned.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{ki}d_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Panics
            /// Panics if `log_base` is zero or greater than the width of the digit type.
            ///
            /// # Examples
            /// See [here](super::power_of_2_digits#from_power_of_2_digits_asc).
            #[inline]
            fn from_power_of_2_digits_asc<I: Iterator<Item = $t>>(
                log_base: u64,
                digits: I,
            ) -> Option<Natural> {
                from_power_of_2_digits_asc_nz(log_base, digits)
            }

            /// Converts an iterator of base-$2^k$ digits into a [`Natural`].
            ///
            /// The base-2 logarithm of the base is specified. The input digits are in descending
            /// order: most- to least-significant. Each digit has primitive integer type, and
            /// `log_base` must be no larger than the width of that type.
            ///
            /// If some digit is greater than $2^k$, `None` is returned.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Panics
            /// Panics if `log_base` is zero or greater than the width of the digit type.
            ///
            /// # Examples
            /// See [here](super::power_of_2_digits#from_power_of_2_digits_desc).
            #[inline]
            fn from_power_of_2_digits_desc<I: Iterator<Item = $t>>(
                log_base: u64,
                digits: I,
            ) -> Option<Natural> {
                from_power_of_2_digits_desc_nz(log_base, digits)
            }
        }
    };
}
apply_to_unsigneds!(power_of_2_digits_unsigned);

impl PowerOf2Digits<Natural> for Natural {
    /// Returns a [`Vec`] containing the base-$2^k$ digits of a [`Natural`] in ascending order:
    /// least- to most-significant.
    ///
    /// The base-2 logarithm of the base is specified. The type of each digit is [`Natural`]. If the
    /// [`Natural`] is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
    ///
    /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$ or $d_{n-1} \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{n-1}2^{ki}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::ZERO, 6)
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::TWO, 6).to_debug_string(),
    ///     "[2]"
    /// );
    ///
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_asc(&Natural::from(123u32), 3)
    ///         .to_debug_string(),
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
            .copied()
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

    /// Returns a [`Vec`] containing the base-$2^k$ digits of a [`Natural`] in descending order:
    /// most- to least-significant.
    ///
    /// The base-2 logarithm of the base is specified. The type of each digit is [`Natural`]. If the
    /// [`Natural`] is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
    ///
    /// $f(x, k) = (d_i)_ {i=0}^{n-1}$, where $0 \leq d_i < 2^k$ for all $i$, $n=0$ or $d_0 \neq 0$,
    /// and
    ///
    /// $$
    /// \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::ZERO, 6)
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::TWO, 6)
    ///         .to_debug_string(),
    ///     "[2]"
    /// );
    ///
    /// // 123_10 = 173_8
    /// assert_eq!(
    ///     PowerOf2Digits::<Natural>::to_power_of_2_digits_desc(&Natural::from(123u32), 3)
    ///         .to_debug_string(),
    ///     "[1, 7, 3]"
    /// );
    /// ```
    fn to_power_of_2_digits_desc(&self, log_base: u64) -> Vec<Natural> {
        let mut digits = self.to_power_of_2_digits_asc(log_base);
        digits.reverse();
        digits
    }

    /// Converts an iterator of base-$2^k$ digits into a [`Natural`].
    ///
    /// The base-2 logarithm of the base is specified. The input digits are in ascending order:
    /// least- to most-significant. The type of each digit is [`Natural`].
    ///
    /// If some digit is greater than $2^k$, `None` is returned.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{ki}d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `digits.count()`, and $m$ is `log_base`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_asc(6, digits.iter().cloned()).to_debug_string(),
    ///     "Some(0)"
    /// );
    ///
    /// let digits = &[Natural::TWO, Natural::ZERO];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_asc(6, digits.iter().cloned()).to_debug_string(),
    ///     "Some(2)"
    /// );
    ///
    /// let digits = &[Natural::from(3u32), Natural::from(7u32), Natural::ONE];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_asc(3, digits.iter().cloned()).to_debug_string(),
    ///     "Some(123)"
    /// );
    ///
    /// let digits = &[Natural::from(100u32)];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_asc(3, digits.iter().cloned()).to_debug_string(),
    ///     "None"
    /// );
    /// ```
    fn from_power_of_2_digits_asc<I: Iterator<Item = Natural>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural> {
        assert_ne!(log_base, 0);
        if let Some(log_log_base) = log_base.checked_log_base_2() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Equal => {
                    for digit in digits {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        limbs.push(Limb::wrapping_from(&digit));
                    }
                }
                Less => {
                    for chunk in &digits.chunks(usize::wrapping_from(Limb::WIDTH >> log_log_base)) {
                        let mut limb = 0;
                        let mut offset = 0;
                        for digit in chunk {
                            if digit.significant_bits() > log_base {
                                return None;
                            }
                            limb |= Limb::wrapping_from(&digit) << offset;
                            offset += log_base;
                        }
                        limbs.push(limb);
                    }
                }
                Greater => {
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        offset += chunk_size;
                        limbs.extend(digit.limbs());
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

    /// Converts an iterator of base-$2^k$ digits into a [`Natural`].
    ///
    /// The base-2 logarithm of the base is specified. The input digits are in descending order:
    /// most- to least-significant. The type of each digit is [`Natural`].
    ///
    /// If some digit is greater than $2^k$, `None` is returned.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{n-1}, k) = \sum_{i=0}^{n-1}2^{k (n-i-1)}d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `digits.count()`, and $m$ is `log_base`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_base::num::conversion::traits::PowerOf2Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// let digits = &[Natural::ZERO, Natural::ZERO, Natural::ZERO];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_desc(6, digits.iter().cloned()).to_debug_string(),
    ///     "Some(0)"
    /// );
    ///
    /// let digits = &[Natural::ZERO, Natural::TWO];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_desc(6, digits.iter().cloned()).to_debug_string(),
    ///     "Some(2)"
    /// );
    ///
    /// let digits = &[Natural::ONE, Natural::from(7u32), Natural::from(3u32)];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_desc(3, digits.iter().cloned()).to_debug_string(),
    ///     "Some(123)"
    /// );
    ///
    /// let digits = &[Natural::from(100u32)];
    /// assert_eq!(
    ///     Natural::from_power_of_2_digits_desc(3, digits.iter().cloned()).to_debug_string(),
    ///     "None"
    /// );
    /// ```
    fn from_power_of_2_digits_desc<I: Iterator<Item = Natural>>(
        log_base: u64,
        digits: I,
    ) -> Option<Natural> {
        assert_ne!(log_base, 0);
        if let Some(log_log_base) = log_base.checked_log_base_2() {
            let mut limbs = Vec::new();
            match log_log_base.cmp(&Limb::LOG_WIDTH) {
                Equal => {
                    for digit in digits {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        limbs.push(Limb::wrapping_from(&digit));
                    }
                    limbs.reverse();
                }
                Less => {
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
                Greater => {
                    let digits = digits.collect_vec();
                    let mut offset = 0;
                    let chunk_size = usize::wrapping_from(log_base >> Limb::LOG_WIDTH);
                    for digit in digits.iter().rev() {
                        if digit.significant_bits() > log_base {
                            return None;
                        }
                        offset += chunk_size;
                        limbs.extend(digit.limbs());
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
    pub_test! {to_power_of_2_digits_asc_natural_naive(&self, log_base: u64) -> Vec<Natural> {
        assert_ne!(log_base, 0);
        let digit_len = self
            .significant_bits()
            .div_round(log_base, Ceiling).0;
        let mut digits = Vec::with_capacity(usize::exact_from(digit_len));
        let mut previous_index = 0;
        for _ in 0..digit_len {
            let index = previous_index + log_base;
            digits.push(self.get_bits(previous_index, index));
            previous_index = index;
        }
        digits
    }}

    pub_test! {from_power_of_2_digits_asc_natural_naive<I: Iterator<Item = Natural>>(
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
    }}
}
