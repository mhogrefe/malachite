// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use alloc::string::ToString;
use core::fmt::{Formatter, Result, Write};
use malachite_base::strings::typst::ToTypst;

impl ToTypst for BigExponentFloat {
    /// Writes a [`BigExponentFloat`] as a Typst math-mode fragment.
    ///
    /// This is as [`Float`](crate::Float)s are written. A NaN becomes `"NaN"` and the infinities
    /// become `infinity` and `-infinity`. A finite [`BigExponentFloat`] is written as
    /// [`Display`](core::fmt::Display) writes it, with the exponent, if there is one, lifted into a
    /// real power of ten: `1.3e30` becomes `1.3 times 10^(30)`. The exponent may be arbitrarily
    /// large.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_base::num::conversion::traits::FromStringBase;
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_typst_string(), "\"NaN\"");
    /// assert_eq!(BigExponentFloat::INFINITY.to_typst_string(), "infinity");
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.to_typst_string(),
    ///     "-infinity"
    /// );
    /// assert_eq!(BigExponentFloat::ZERO.to_typst_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_typst_string(), "1.0");
    /// assert_eq!(BigExponentFloat::from(1.5).to_typst_string(), "1.5");
    /// assert_eq!(
    ///     BigExponentFloat::from_string_base(2, "1.0E+100")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "1.3 times 10^(30)"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from_string_base(2, "1.0E-100000000000000000000")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "4.2 times 10^(-30102999566398119522)"
    /// );
    /// ```
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        if self.is_nan() {
            return f.write_str("\"NaN\"");
        } else if self.is_infinite() {
            return f.write_str(if self.is_sign_positive() {
                "infinity"
            } else {
                "-infinity"
            });
        }
        let s = self.to_string();
        let Some(e_index) = s.find('e') else {
            return f.write_str(&s);
        };
        let (mantissa, exponent) = s.split_at(e_index);
        let exponent = &exponent[1..];
        f.write_str(mantissa)?;
        f.write_str(" times 10^")?;
        f.write_char('(')?;
        f.write_str(exponent)?;
        f.write_char(')')
    }
}
