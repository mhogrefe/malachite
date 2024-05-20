// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, Pow};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{Digits, ExactFrom};
use malachite_base::rational_sequences::RationalSequence;
use malachite_nz::natural::Natural;

impl Rational {
    /// Converts base-$b$ digits to a [`Rational`]. The inputs are taken by value.
    ///
    /// The input consists of the digits of the integer portion of the [`Rational`] and the digits
    /// of the fractional portion. The integer-portion digits are ordered from least- to
    /// most-significant, and the fractional-portion digits from most- to least.
    ///
    /// The fractional-portion digits may end in infinitely many zeros or $(b-1)$s; these are
    /// handled correctly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm)^2 \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(before_point.len(),
    /// after_point.component_len())`, and $m$ is `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let before_point = vec_from_str("[3]").unwrap();
    /// let after_point =
    ///     RationalSequence::from_vecs(Vec::new(), vec_from_str("[1, 4, 2, 8, 5, 7]").unwrap());
    /// assert_eq!(
    ///     Rational::from_digits(&Natural::from(10u32), before_point, after_point).to_string(),
    ///     "22/7"
    /// );
    ///
    /// // 21.34565656...
    /// let before_point = vec_from_str("[1, 2]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[3, 4]").unwrap(),
    ///     vec_from_str("[5, 6]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_digits(&Natural::from(10u32), before_point, after_point).to_string(),
    ///     "105661/4950"
    /// );
    /// ```
    pub fn from_digits(
        base: &Natural,
        before_point: Vec<Natural>,
        after_point: RationalSequence<Natural>,
    ) -> Rational {
        if let Some(log_base) = base.checked_log_base_2() {
            return Rational::from_power_of_2_digits(log_base, before_point, after_point);
        }
        let (non_repeating, repeating) = after_point.into_vecs();
        let r_len = u64::exact_from(repeating.len());
        let nr_len = u64::exact_from(non_repeating.len());
        let nr = Natural::from_digits_asc(base, non_repeating.into_iter().rev()).unwrap();
        let r = Natural::from_digits_asc(base, repeating.into_iter().rev()).unwrap();
        let floor =
            Rational::from(Natural::from_digits_asc(base, before_point.into_iter()).unwrap());
        floor
            + if r == 0u32 {
                Rational::from_naturals(nr, base.pow(nr_len))
            } else {
                (Rational::from_naturals(r, base.pow(r_len) - Natural::ONE) + Rational::from(nr))
                    / Rational::from(base.pow(nr_len))
            }
    }

    /// Converts base-$b$ digits to a [`Rational`]. The inputs are taken by reference.
    ///
    /// The input consists of the digits of the integer portion of the [`Rational`] and the digits
    /// of the fractional portion. The integer-portion digits are ordered from least- to
    /// most-significant, and the fractional-portion digits from most- to least.
    ///
    /// The fractional-portion digits may end in infinitely many zeros or $(b-1)$s; these are
    /// handled correctly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm)^2 \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(before_point.len(),
    /// after_point.component_len())`, and $m$ is `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let before_point = vec_from_str("[3]").unwrap();
    /// let after_point =
    ///     RationalSequence::from_vecs(Vec::new(), vec_from_str("[1, 4, 2, 8, 5, 7]").unwrap());
    /// assert_eq!(
    ///     Rational::from_digits_ref(&Natural::from(10u32), &before_point, &after_point)
    ///         .to_string(),
    ///     "22/7"
    /// );
    ///
    /// // 21.34565656...
    /// let before_point = vec_from_str("[1, 2]").unwrap();
    /// let after_point = RationalSequence::from_vecs(
    ///     vec_from_str("[3, 4]").unwrap(),
    ///     vec_from_str("[5, 6]").unwrap(),
    /// );
    /// assert_eq!(
    ///     Rational::from_digits_ref(&Natural::from(10u32), &before_point, &after_point)
    ///         .to_string(),
    ///     "105661/4950"
    /// );
    /// ```
    pub fn from_digits_ref(
        base: &Natural,
        before_point: &[Natural],
        after_point: &RationalSequence<Natural>,
    ) -> Rational {
        if let Some(log_base) = base.checked_log_base_2() {
            return Rational::from_power_of_2_digits_ref(log_base, before_point, after_point);
        }
        let (non_repeating, repeating) = after_point.to_vecs();
        let r_len = u64::exact_from(repeating.len());
        let nr_len = u64::exact_from(non_repeating.len());
        let nr = Natural::from_digits_asc(base, non_repeating.into_iter().rev()).unwrap();
        let r = Natural::from_digits_asc(base, repeating.into_iter().rev()).unwrap();
        let floor =
            Rational::from(Natural::from_digits_asc(base, before_point.iter().cloned()).unwrap());
        floor
            + if r == 0u32 {
                Rational::from_naturals(nr, base.pow(nr_len))
            } else {
                (Rational::from_naturals(r, base.pow(r_len) - Natural::ONE) + Rational::from(nr))
                    / Rational::from(base.pow(nr_len))
            }
    }
}
