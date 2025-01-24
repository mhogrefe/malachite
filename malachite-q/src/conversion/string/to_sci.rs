// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::log_base::log_base_helper;
use crate::Rational;
use alloc::string::String;
use core::cmp::{max, Ordering::*};
use core::fmt::{Formatter, Write};
use malachite_base::num::arithmetic::traits::{
    Abs, CheckedLogBase2, DivExact, DivExactAssign, DivRound, DivisibleBy, Pow, Sign,
};
use malachite_base::num::conversion::string::options::{SciSizeOptions, ToSciOptions};
use malachite_base::num::conversion::string::to_sci::write_exponent;
use malachite_base::num::conversion::traits::{
    ExactFrom, IsInteger, RoundingFrom, ToSci, ToStringBase, WrappingFrom,
};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const BASE_PRIME_FACTORS: [[(u8, u8); 3]; 37] = [
    [(0, 0), (0, 0), (0, 0)],
    [(0, 0), (0, 0), (0, 0)],
    [(2, 1), (0, 0), (0, 0)],
    [(3, 1), (0, 0), (0, 0)],
    [(2, 2), (0, 0), (0, 0)],
    [(5, 1), (0, 0), (0, 0)],
    [(2, 1), (3, 1), (0, 0)],
    [(7, 1), (0, 0), (0, 0)],
    [(2, 3), (0, 0), (0, 0)],
    [(3, 2), (0, 0), (0, 0)],
    [(2, 1), (5, 1), (0, 0)],
    [(11, 1), (0, 0), (0, 0)],
    [(2, 2), (3, 1), (0, 0)],
    [(13, 1), (0, 0), (0, 0)],
    [(2, 1), (7, 1), (0, 0)],
    [(3, 1), (5, 1), (0, 0)],
    [(2, 4), (0, 0), (0, 0)],
    [(17, 1), (0, 0), (0, 0)],
    [(2, 1), (3, 2), (0, 0)],
    [(19, 1), (0, 0), (0, 0)],
    [(2, 2), (5, 1), (0, 0)],
    [(3, 1), (7, 1), (0, 0)],
    [(2, 1), (11, 1), (0, 0)],
    [(23, 1), (0, 0), (0, 0)],
    [(2, 3), (3, 1), (0, 0)],
    [(5, 2), (0, 0), (0, 0)],
    [(2, 1), (13, 1), (0, 0)],
    [(3, 3), (0, 0), (0, 0)],
    [(2, 2), (7, 1), (0, 0)],
    [(29, 1), (0, 0), (0, 0)],
    [(2, 1), (3, 1), (5, 1)],
    [(31, 1), (0, 0), (0, 0)],
    [(2, 5), (0, 0), (0, 0)],
    [(3, 1), (11, 1), (0, 0)],
    [(2, 1), (17, 1), (0, 0)],
    [(5, 1), (7, 1), (0, 0)],
    [(2, 2), (3, 2), (0, 0)],
];

impl Rational {
    /// When expanding a [`Rational`] in a small base $b$, determines how many digits after the
    /// decimal (or other-base) point are in the base-$b$ expansion.
    ///
    /// If the expansion is non-terminating, this method returns `None`. This happens iff the
    /// [`Rational`]'s denominator has prime factors not present in $b$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// // 3/8 is 0.375 in base 10.
    /// assert_eq!(
    ///     Rational::from_signeds(3, 8).length_after_point_in_small_base(10),
    ///     Some(3)
    /// );
    /// // 1/20 is 0.05 in base 10.
    /// assert_eq!(
    ///     Rational::from_signeds(1, 20).length_after_point_in_small_base(10),
    ///     Some(2)
    /// );
    /// // 1/7 is non-terminating in base 10.
    /// assert_eq!(
    ///     Rational::from_signeds(1, 7).length_after_point_in_small_base(10),
    ///     None
    /// );
    /// // 1/7 is 0.3 in base 21.
    /// assert_eq!(
    ///     Rational::from_signeds(1, 7).length_after_point_in_small_base(21),
    ///     Some(1)
    /// );
    /// ```
    pub fn length_after_point_in_small_base(&self, base: u8) -> Option<u64> {
        let d = self.denominator_ref();
        assert!((2..=36).contains(&base));
        if *d == 1u32 {
            return Some(0);
        }
        let mut temp = None;
        let mut length = 0;
        for (f, m) in BASE_PRIME_FACTORS[usize::wrapping_from(base)] {
            let count = if f == 0 {
                break;
            } else if f == 2 {
                let twos = d.trailing_zeros().unwrap();
                if twos != 0 {
                    temp = Some(d >> twos);
                }
                twos
            } else {
                let f = Natural::from(f);
                let mut count = 0;
                if let Some(temp) = temp.as_mut() {
                    while (&*temp).divisible_by(&f) {
                        temp.div_exact_assign(&f);
                        count += 1;
                    }
                } else if d.divisible_by(&f) {
                    count = 1;
                    let mut t = d.div_exact(&f);
                    while (&t).divisible_by(&f) {
                        t.div_exact_assign(&f);
                        count += 1;
                    }
                    temp = Some(t);
                }
                count
            };
            length = max(length, count.div_round(u64::from(m), Ceiling).0);
        }
        if let Some(temp) = temp {
            if temp == 1 {
                Some(length)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub_test! {floor_log_base_of_abs(x: &Rational, base: &Rational) -> i64 {
    if let Some(log_base) = base.checked_log_base_2() {
        match log_base.sign() {
            Equal => panic!("Cannot take base-1 logarithm"),
            Greater => x
                .floor_log_base_2_abs()
                .div_round(log_base, Floor).0,
            Less => {
                -(x.ceiling_log_base_2_abs()
                    .div_round(-log_base, Ceiling).0)
            }
        }
    } else {
        log_base_helper(x, base).0
    }
}}

fn fmt_zero(f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
    f.write_char('0')?;
    let scale = if options.get_include_trailing_zeros() {
        match options.get_size_options() {
            SciSizeOptions::Complete => None,
            SciSizeOptions::Scale(scale) => {
                if scale == 0 {
                    None
                } else {
                    Some(scale)
                }
            }
            SciSizeOptions::Precision(precision) => {
                if precision == 1 {
                    None
                } else {
                    Some(precision - 1)
                }
            }
        }
    } else {
        None
    };
    if let Some(scale) = scale {
        f.write_char('.')?;
        for _ in 0..scale {
            f.write_char('0')?;
        }
    }
    Ok(())
}

impl ToSci for Rational {
    /// Determines whether a [`Rational`] can be converted to a string using
    /// [`to_sci`](malachite_base::num::conversion::traits::ToSci::to_sci) and a particular set of
    /// options.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), s)`,
    /// where `s` depends on the size type specified in `options`.
    /// - If `options` has `scale` specified, then `s` is `options.scale`.
    /// - If `options` has `precision` specified, then `s` is `options.precision`.
    /// - If `options` has `size_complete` specified, then `s` is `self.denominator` (not the log of
    ///   the denominator!). This reflects the fact that setting `size_complete` might result in a
    ///   very long string.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_q::Rational;
    ///
    /// let mut options = ToSciOptions::default();
    /// assert!(Rational::from(123u8).fmt_sci_valid(options));
    /// assert!(Rational::from(u128::MAX).fmt_sci_valid(options));
    /// // u128::MAX has more than 16 significant digits
    /// options.set_rounding_mode(Exact);
    /// assert!(!Rational::from(u128::MAX).fmt_sci_valid(options));
    /// options.set_precision(50);
    /// assert!(Rational::from(u128::MAX).fmt_sci_valid(options));
    ///
    /// let mut options = ToSciOptions::default();
    /// options.set_size_complete();
    /// // 1/3 is non-terminating in base 10...
    /// assert!(!Rational::from_signeds(1, 3).fmt_sci_valid(options));
    /// options.set_size_complete();
    ///
    /// // ...but is terminating in base 36
    /// options.set_base(36);
    /// assert!(Rational::from_signeds(1, 3).fmt_sci_valid(options));
    /// ```
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
        if *self == 0 {
            return true;
        }
        if let SciSizeOptions::Complete = options.get_size_options() {
            return self
                .length_after_point_in_small_base(options.get_base())
                .is_some();
        }
        if options.get_rounding_mode() != Exact {
            return true;
        }
        let q_base = Rational::from(options.get_base());
        let scale = match options.get_size_options() {
            SciSizeOptions::Precision(precision) => {
                let log = floor_log_base_of_abs(self, &q_base);
                i64::exact_from(precision - 1) - log
            }
            SciSizeOptions::Scale(scale) => i64::exact_from(scale),
            _ => unreachable!(),
        };
        (self * q_base.pow(scale)).is_integer()
    }

    /// Converts a [`Rational` ]to a string using a specified base, possibly formatting the number
    /// using scientific notation.
    ///
    /// See [`ToSciOptions`] for details on the available options.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(), s)`,
    /// where `s` depends on the size type specified in `options`.
    /// - If `options` has `scale` specified, then `s` is `options.scale`.
    /// - If `options` has `precision` specified, then `s` is `options.precision`.
    /// - If `options` has `size_complete` specified, then `s` is `self.denominator` (not the log of
    ///   the denominator!). This reflects the fact that setting `size_complete` might result in a
    ///   very long string.
    ///
    /// # Panics
    /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the input
    /// must be rounded.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from_signeds(22, 7);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "3.142857142857143"
    /// );
    ///
    /// options.set_precision(3);
    /// assert_eq!(q.to_sci_with_options(options).to_string(), "3.14");
    ///
    /// options.set_rounding_mode(Ceiling);
    /// assert_eq!(q.to_sci_with_options(options).to_string(), "3.15");
    ///
    /// options = ToSciOptions::default();
    /// options.set_base(20);
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "3.2h2h2h2h2h2h2h3"
    /// );
    ///
    /// options.set_uppercase();
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "3.2H2H2H2H2H2H2H3"
    /// );
    ///
    /// options.set_base(2);
    /// options.set_rounding_mode(Floor);
    /// options.set_precision(19);
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "11.001001001001001"
    /// );
    ///
    /// options.set_include_trailing_zeros(true);
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "11.00100100100100100"
    /// );
    ///
    /// let q = Rational::from_unsigneds(936851431250u64, 1397u64);
    /// let mut options = ToSciOptions::default();
    /// options.set_precision(6);
    /// assert_eq!(q.to_sci_with_options(options).to_string(), "6.70617e8");
    ///
    /// options.set_e_uppercase();
    /// assert_eq!(q.to_sci_with_options(options).to_string(), "6.70617E8");
    ///
    /// options.set_force_exponent_plus_sign(true);
    /// assert_eq!(q.to_sci_with_options(options).to_string(), "6.70617E+8");
    ///
    /// let q = Rational::from_signeds(123i64, 45678909876i64);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "2.692708743135418e-9"
    /// );
    ///
    /// options.set_neg_exp_threshold(-10);
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "0.000000002692708743135418"
    /// );
    ///
    /// let q = Rational::power_of_2(-30i64);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "9.313225746154785e-10"
    /// );
    ///
    /// options.set_size_complete();
    /// assert_eq!(
    ///     q.to_sci_with_options(options).to_string(),
    ///     "9.31322574615478515625e-10"
    /// );
    /// ```
    fn fmt_sci(&self, f: &mut Formatter, options: ToSciOptions) -> core::fmt::Result {
        if *self == 0u32 {
            return fmt_zero(f, options);
        }
        if *self < 0u32 {
            f.write_char('-')?;
        }
        let base = options.get_base();
        let q_base = Rational::from(base);
        let mut trim_zeros = !options.get_include_trailing_zeros();
        let mut log = floor_log_base_of_abs(self, &q_base);
        // Here, precision 0 means that we're rounding down to zero
        let (mut scale, mut precision) = match options.get_size_options() {
            SciSizeOptions::Complete => {
                trim_zeros = false;
                let scale = self
                    .length_after_point_in_small_base(base)
                    .unwrap_or_else(|| {
                        panic!("{self} has a non-terminating expansion in base {base}")
                    });
                let precision = i64::exact_from(scale) + log + 1;
                assert!(precision > 0);
                (i64::exact_from(scale), precision)
            }
            SciSizeOptions::Scale(scale) => {
                (i64::exact_from(scale), i64::exact_from(scale) + log + 1)
            }
            SciSizeOptions::Precision(precision) => (
                i64::exact_from(precision - 1) - log,
                i64::exact_from(precision),
            ),
        };
        let n = Integer::rounding_from(self * q_base.pow(scale), options.get_rounding_mode())
            .0
            .abs();
        if precision <= 0 {
            // e.g. we're in base 10, self is 0.01 or 0.000001, but scale is 1
            if n == 0u32 {
                return fmt_zero(f, options);
            } else if n == 1u32 {
                precision = 1;
                log = -scale;
            } else {
                panic!("Bug: precision <= 0 must mean self.abs() rounds to 0 or 1");
            };
        }
        let mut cs = if options.get_lowercase() {
            n.to_string_base(base)
        } else {
            n.to_string_base_upper(base)
        }
        .into_bytes();
        let mut precision = usize::exact_from(precision);
        if cs.len() == precision + 1 {
            // We rounded up to a power of the base, so precision is greater than we expected. If
            // the options specify the precision, we need to adjust.
            log += 1;
            match options.get_size_options() {
                SciSizeOptions::Complete => panic!(),
                SciSizeOptions::Precision(_) => {
                    scale -= 1;
                    assert_eq!(cs.pop().unwrap(), b'0');
                }
                SciSizeOptions::Scale(_) => {
                    precision += 1;
                }
            }
        }
        assert_eq!(cs.len(), precision);
        if log <= options.get_neg_exp_threshold() || scale < 0 {
            assert_ne!(log, 0);
            // exponent
            if trim_zeros {
                let trailing_zeros = cs.iter().rev().take_while(|&&c| c == b'0').count();
                precision -= trailing_zeros;
                cs.truncate(precision);
            }
            if precision > 1 {
                cs.push(0);
                cs.copy_within(1..precision, 2);
                cs[1] = b'.';
            }
            f.write_str(&String::from_utf8(cs).unwrap())?;
            write_exponent(f, options, log)
        } else if scale == 0 {
            // no exponent or point
            f.write_str(&String::from_utf8(cs).unwrap())
        } else {
            // no exponent
            if trim_zeros {
                let trailing_zeros = cs
                    .iter()
                    .rev()
                    .take(usize::exact_from(scale))
                    .take_while(|&&c| c == b'0')
                    .count();
                precision -= trailing_zeros;
                cs.truncate(precision);
            }
            if log < 0 {
                f.write_char('0')?;
                f.write_char('.')?;
                for _ in 0..-log - 1 {
                    f.write_char('0')?;
                }
            } else {
                let digits_before = usize::exact_from(log) + 1;
                if precision > digits_before {
                    cs.push(0);
                    cs.copy_within(digits_before..precision, digits_before + 1);
                    cs[digits_before] = b'.';
                }
            }
            f.write_str(&String::from_utf8(cs).unwrap())
        }
    }
}
