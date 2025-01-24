// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::traits::SimplestRationalInInterval;
use crate::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
use crate::Rational;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;

impl Rational {
    /// Converts a primitive float to the simplest [`Rational`] that rounds to that value.
    ///
    /// To be more specific: Suppose the floating-point input is $x$. If $x$ is an integer, its
    /// [`Rational`] equivalent is returned. Otherwise, this function finds $a$ and $b$, which are
    /// the floating point predecessor and successor of $x$, and finds the simplest [`Rational`] in
    /// the open interval $(\frac{x + a}{2}, \frac{x + b}{2})$. "Simplicity" refers to low
    /// complexity. See [`Rational::cmp_complexity`] for a definition of complexity.
    ///
    /// For example, `0.1f32` is converted to $1/10$ rather than to the exact value of the float,
    /// which is $13421773/134217728$. If you want the exact value, use `Rational::from` instead.
    ///
    /// If the floating point value cannot be NaN or infinite, and error is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.sci_exponent()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::conversion::from_primitive_float::RationalFromPrimitiveFloatError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(0.0).to_debug_string(),
    ///     "Ok(0)"
    /// );
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(1.5).to_debug_string(),
    ///     "Ok(3/2)"
    /// );
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(-1.5).to_debug_string(),
    ///     "Ok(-3/2)"
    /// );
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(0.1f32).to_debug_string(),
    ///     "Ok(1/10)"
    /// );
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(0.33333334f32).to_debug_string(),
    ///     "Ok(1/3)"
    /// );
    /// assert_eq!(
    ///     Rational::try_from_float_simplest(f32::NAN),
    ///     Err(RationalFromPrimitiveFloatError)
    /// );
    /// ```
    pub fn try_from_float_simplest<T: PrimitiveFloat>(
        x: T,
    ) -> Result<Rational, RationalFromPrimitiveFloatError>
    where
        Rational: TryFrom<T, Error = RationalFromPrimitiveFloatError>,
    {
        let q = Rational::try_from(x)?;
        Ok(if *q.denominator_ref() <= 2u32 {
            q
        } else {
            let succ_q = Rational::exact_from(x.next_higher());
            let pred_q = Rational::exact_from(x.next_lower());
            let x = (pred_q + &q) >> 1;
            let y = (succ_q + q) >> 1;
            Rational::simplest_rational_in_open_interval(&x, &y)
        })
    }
}
