// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::traits::SimplestRationalInInterval;
use crate::Rational;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::conversion::string::from_sci_string::preprocess_sci_string;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::FromSciString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::string::from_sci_string::FromSciStringHelper;

impl FromSciString for Rational {
    /// Converts a string, possibly in scientfic notation, to a [`Rational`].
    ///
    /// Use [`FromSciStringOptions`] to specify the base (from 2 to 36, inclusive). The rounding
    /// mode option is ignored.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, an ambiguity arises where it may not be clear whether `'e'` is a digit or an
    /// exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after the
    /// exponent indicator when the base is 15 or greater.
    ///
    /// The exponent itself is always parsed using base 10.
    ///
    /// Decimal (or other-base) points are allowed.
    ///
    /// If the string is unparseable, `None` is returned.
    ///
    /// This function is very literal; given `"0.333"`, it will return $333/1000$ rather than $1/3$.
    /// If you'd prefer that it return $1/3$, consider using
    /// [`from_sci_string_simplest`](Rational::from_sci_string_simplest) instead. However, that
    /// function has its quirks too: given `"0.1"`, it will not return $1/10$ (see its documentation
    /// for an explanation of this behavior). This function _does_ return $1/10$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m^n n \log m (\log n + \log\log m))$
    ///
    /// $M(n, m) = O(m^n n \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `s.len()`, and $m$ is `options.base`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::num::conversion::traits::FromSciString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_sci_string("123").unwrap(), 123);
    /// assert_eq!(
    ///     Rational::from_sci_string("0.1").unwrap().to_string(),
    ///     "1/10"
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string("0.10").unwrap().to_string(),
    ///     "1/10"
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string("0.333").unwrap().to_string(),
    ///     "333/1000"
    /// );
    /// assert_eq!(Rational::from_sci_string("1.2e5").unwrap(), 120000);
    /// assert_eq!(
    ///     Rational::from_sci_string("1.2e-5").unwrap().to_string(),
    ///     "3/250000"
    /// );
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     Rational::from_sci_string_with_options("ff", options).unwrap(),
    ///     255
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string_with_options("ffE+5", options).unwrap(),
    ///     267386880
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string_with_options("ffE-5", options)
    ///         .unwrap()
    ///         .to_string(),
    ///     "255/1048576"
    /// );
    /// ```
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Rational> {
        let (s, exponent) = preprocess_sci_string(s, options)?;
        let x = Rational::from(Integer::parse_int(&s, options.get_base())?);
        Some(x * Rational::from(options.get_base()).pow(exponent))
    }
}

impl Rational {
    /// Converts a string, possibly in scientfic notation, to a [`Rational`]. This function finds
    /// the simplest [`Rational`] which rounds to the target string according to the precision
    /// implied by the string.
    ///
    /// Use [`FromSciStringOptions`] to specify the base (from 2 to 36, inclusive). The rounding
    /// mode option is ignored.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, an ambiguity arises where it may not be clear whether `'e'` is a digit or an
    /// exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after the
    /// exponent indicator when the base is 15 or greater.
    ///
    /// The exponent itself is always parsed using base 10.
    ///
    /// Decimal (or other-base) points are allowed.
    ///
    /// If the string is unparseable, `None` is returned.
    ///
    /// Here's a more precise description of the function's behavior. Suppose we are using base $b$,
    /// and the literal value of the string (as parsed by
    /// [`from_sci_string`](Rational::from_sci_string)) is $q$, and the implied scale is $s$
    /// (meaning $s$ digits are provided after the point; if the string is `"123.456"`, then $s$ is
    /// 3). Then this function computes $\varepsilon = b^{-s}/2$ and finds the simplest [`Rational`]
    /// in the closed interval $[q - \varepsilon, q + \varepsilon]$. The simplest [`Rational`] is
    /// the one with minimal denominator; if there are multiple such [`Rational`]s, the one with the
    /// smallest absolute numerator is chosen.
    ///
    /// The following discussion assumes base 10.
    ///
    /// This method allows the function to convert `"0.333"` to $1/3$, since $1/3$ is the simplest
    /// [`Rational`] in the interval $[0.3325, 0.3335]$. But note that if the scale of the input is
    /// low, some unexpected behavior may occur. For example, `"0.1"` will be converted into $1/7$
    /// rather than $1/10$, since $1/7$ is the simplest [`Rational`] in $[0.05, 0.15]$. If you'd
    /// prefer that result be $1/10$, you have a few options:
    /// - Use [`from_sci_string_with_options`](Rational::from_sci_string_with_options) instead. This
    ///   function interprets its input literally; it converts `"0.333"` to $333/1000$.
    /// - Increase the scale of the input; `"0.10"` is converted to $1/10$.
    /// - Use [`from_sci_string_with_options`](Rational::from_sci_string_with_options), and round
    ///   the result manually using functions like
    ///   [`round_to_multiple`](malachite_base::num::arithmetic::traits::RoundToMultiple::round_to_multiple)
    ///   and
    ///   [`simplest_rational_in_closed_interval`](Rational::simplest_rational_in_closed_interval).
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m^n n \log m (\log n + \log\log m))$
    ///
    /// $M(n, m) = O(m^n n \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `s.len()`, and $m$ is `options.base`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_q::Rational;
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest_with_options("ff", options).unwrap(),
    ///     255
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest_with_options("ffE+5", options).unwrap(),
    ///     267386880
    /// );
    /// // 1/4105 is 0.000ff705..._16
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest_with_options("ffE-5", options)
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/4105"
    /// );
    /// ```
    pub fn from_sci_string_simplest_with_options(
        s: &str,
        options: FromSciStringOptions,
    ) -> Option<Rational> {
        let (s, exponent) = preprocess_sci_string(s, options)?;
        let x = Rational::from(Integer::parse_int(&s, options.get_base())?);
        let p = Rational::from(options.get_base()).pow(exponent);
        let q = x * &p;
        if exponent >= 0 {
            Some(q)
        } else {
            let epsilon = p >> 1;
            Some(Rational::simplest_rational_in_closed_interval(
                &(&q - &epsilon),
                &(q + epsilon),
            ))
        }
    }

    /// Converts a string, possibly in scientfic notation, to a [`Rational`]. This function finds
    /// the simplest [`Rational`] which rounds to the target string according to the precision
    /// implied by the string.
    ///
    /// The string is parsed using base 10. To use other bases, try
    /// [`from_sci_string_simplest_with_options`](Rational::from_sci_string_simplest_with_options)
    /// instead.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`.
    ///
    /// The exponent itself is also parsed using base 10.
    ///
    /// Decimal points are allowed.
    ///
    /// If the string is unparseable, `None` is returned.
    ///
    /// Here's a more precise description of the function's behavior. Suppose that the literal value
    /// of the string (as parsed by [`from_sci_string`](Rational::from_sci_string)) is $q$, and the
    /// implied scale is $s$ (meaning $s$ digits are provided after the point; if the string is
    /// `"123.456"`, then $s$ is 3). Then this function computes $\varepsilon = 10^{-s}/2$ and finds
    /// the simplest [`Rational`] in the closed interval $[q - \varepsilon, q + \varepsilon]$. The
    /// simplest [`Rational`] is the one with minimal denominator; if there are multiple such
    /// [`Rational`]s, the one with the smallest absolute numerator is chosen.
    ///
    /// This method allows the function to convert `"0.333"` to $1/3$, since $1/3$ is the simplest
    /// [`Rational`] in the interval $[0.3325, 0.3335]$. But note that if the scale of the input is
    /// low, some unexpected behavior may occur. For example, `"0.1"` will be converted into $1/7$
    /// rather than $1/10$, since $1/7$ is the simplest [`Rational`] in $[0.05, 0.15]$. If you'd
    /// prefer that result be $1/10$, you have a few options:
    /// - Use [`from_sci_string`](Rational::from_sci_string) instead. This function interprets its
    ///   input literally; it converts `"0.333"` to $333/1000$.
    /// - Increase the scale of the input; `"0.10"` is converted to $1/10$.
    /// - Use [`from_sci_string`](Rational::from_sci_string), and round the result manually using
    ///   functions like
    ///   [`round_to_multiple`](malachite_base::num::arithmetic::traits::RoundToMultiple::round_to_multiple)
    ///   and
    ///   [`simplest_rational_in_closed_interval`](Rational::simplest_rational_in_closed_interval).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(10^n n \log n)$
    ///
    /// $M(n) = O(10^n n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_sci_string_simplest("123").unwrap(), 123);
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest("0.1")
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/7"
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest("0.10")
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/10"
    /// );
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest("0.333")
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// assert_eq!(Rational::from_sci_string_simplest("1.2e5").unwrap(), 120000);
    /// assert_eq!(
    ///     Rational::from_sci_string_simplest("1.2e-5")
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/80000"
    /// );
    /// ```
    #[inline]
    pub fn from_sci_string_simplest(s: &str) -> Option<Rational> {
        Rational::from_sci_string_simplest_with_options(s, FromSciStringOptions::default())
    }
}
