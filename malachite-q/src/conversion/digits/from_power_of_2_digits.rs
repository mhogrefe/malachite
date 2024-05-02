// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rational_sequences::RationalSequence;
use malachite_nz::natural::Natural;

impl Rational {
    /// Converts base-$2^k$ digits to a [`Rational`]. The inputs are taken by value.
    ///
    /// The input consists of the digits of the integer portion of the [`Rational`] and the digits
    /// of the fractional portion. The integer-portion digits are ordered from least- to
    /// most-significant, and the fractional-portion digits from most- to least.
    ///
    /// The fractional-portion digits may end in infinitely many zeros or $(2^k-1)$s; these are
    /// handled correctly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(before_point.len(),
    /// after_point.component_len())`, and $m$ is `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::Rational;
    ///
    /// let before_point = vec_from_str("[1, 1]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[0]").unwrap(),
    ///     vec_from_str("[0, 0, 1]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_power_of_2_digits(1, before_point, after_point).to_string(),
    ///     "43/14"
    /// );
    ///
    /// // 21.34565656..._32
    /// let before_point = vec_from_str("[1, 2]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[3, 4]").unwrap(),
    ///     vec_from_str("[5, 6]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_power_of_2_digits(5, before_point, after_point).to_string(),
    ///     "34096673/523776"
    /// );
    /// ```
    pub fn from_power_of_2_digits(
        log_base: u64,
        before_point: Vec<Natural>,
        after_point: RationalSequence<Natural>,
    ) -> Rational {
        let (non_repeating, repeating) = after_point.into_vecs();
        let r_len = u64::exact_from(repeating.len());
        let nr_len = u64::exact_from(non_repeating.len());
        let nr =
            Natural::from_power_of_2_digits_asc(log_base, non_repeating.into_iter().rev()).unwrap();
        let r = Natural::from_power_of_2_digits_asc(log_base, repeating.into_iter().rev()).unwrap();
        let floor = Rational::from(
            Natural::from_power_of_2_digits_asc(log_base, before_point.into_iter()).unwrap(),
        );
        floor
            + if r == 0u32 {
                Rational::from(nr) >> (log_base * nr_len)
            } else {
                (Rational::from_naturals(r, Natural::low_mask(log_base * r_len))
                    + Rational::from(nr))
                    >> (log_base * nr_len)
            }
    }

    /// Converts base-$2^k$ digits to a [`Rational`]. The inputs are taken by reference.
    ///
    /// The input consists of the digits of the integer portion of the [`Rational`] and the digits
    /// of the fractional portion. The integer-portion digits are ordered from least- to
    /// most-significant, and the fractional-portion digits from most- to least.
    ///
    /// The fractional-portion digits may end in infinitely many zeros or $(2^k-1)$s; these are
    /// handled correctly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(before_point.len(),
    /// after_point.component_len())`, and $m$ is `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::Rational;
    ///
    /// let before_point = vec_from_str("[1, 1]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[0]").unwrap(),
    ///     vec_from_str("[0, 0, 1]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_power_of_2_digits_ref(1, &before_point, &after_point).to_string(),
    ///     "43/14"
    /// );
    ///
    /// // 21.34565656..._32
    /// let before_point = vec_from_str("[1, 2]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[3, 4]").unwrap(),
    ///     vec_from_str("[5, 6]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_power_of_2_digits_ref(5, &before_point, &after_point).to_string(),
    ///     "34096673/523776"
    /// );
    /// ```
    pub fn from_power_of_2_digits_ref(
        log_base: u64,
        before_point: &[Natural],
        after_point: &RationalSequence<Natural>,
    ) -> Rational {
        let (non_repeating, repeating) = after_point.to_vecs();
        let r_len = u64::exact_from(repeating.len());
        let nr_len = u64::exact_from(non_repeating.len());
        let nr =
            Natural::from_power_of_2_digits_asc(log_base, non_repeating.into_iter().rev()).unwrap();
        let r = Natural::from_power_of_2_digits_asc(log_base, repeating.into_iter().rev()).unwrap();
        let floor = Rational::from(
            Natural::from_power_of_2_digits_asc(log_base, before_point.iter().cloned()).unwrap(),
        );
        floor
            + if r == 0u32 {
                Rational::from(nr) >> (log_base * nr_len)
            } else {
                (Rational::from_naturals(r, Natural::low_mask(log_base * r_len))
                    + Rational::from(nr))
                    >> (log_base * nr_len)
            }
    }
}
