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
use malachite_base::strings::latex::ToLatex;

impl ToLatex for BigExponentFloat {
    /// Writes a [`BigExponentFloat`] as a LaTeX math-mode fragment.
    ///
    /// This is as [`Float`](crate::Float)s are written. A NaN becomes `\text{NaN}` and the
    /// infinities become `\infty` and `-\infty`. A finite [`BigExponentFloat`] is written as
    /// [`Display`](core::fmt::Display) writes it, with the exponent, if there is one, lifted into a
    /// real power of ten: `1.3e30` becomes ``1.3 \times 10^{30}``. The exponent may be arbitrarily
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
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.to_latex_string(), r"\text{NaN}");
    /// assert_eq!(BigExponentFloat::INFINITY.to_latex_string(), r"\infty");
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.to_latex_string(),
    ///     r"-\infty"
    /// );
    /// assert_eq!(BigExponentFloat::ZERO.to_latex_string(), "0.0");
    /// assert_eq!(BigExponentFloat::ONE.to_latex_string(), "1.0");
    /// assert_eq!(BigExponentFloat::from(1.5).to_latex_string(), "1.5");
    /// assert_eq!(
    ///     BigExponentFloat::from_string_base(2, "1.0E+100")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "1.3 \\times 10^{30}"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::from_string_base(2, "1.0E+100000000000000000000")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "2.4 \\times 10^{30102999566398119521}"
    /// );
    /// ```
    ///
    /// | value                       | fragment             | renders as           |
    /// |-----------------------------|----------------------|----------------------|
    /// | `BigExponentFloat::NAN`     | `\text{NaN}`         | $\text{NaN}$         |
    /// | `BigExponentFloat::INFINITY`| `\infty`             | $\infty$             |
    /// | `BigExponentFloat::ONE`     | `1.0`                | $1.0$                |
    /// | $2^{100}$                   | `1.3 \times 10^{30}` | $1.3 \times 10^{30}$ |
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        if self.is_nan() {
            return f.write_str("\\text{NaN}");
        } else if self.is_infinite() {
            return f.write_str(if self.is_sign_positive() {
                "\\infty"
            } else {
                "-\\infty"
            });
        }
        let s = self.to_string();
        let Some(e_index) = s.find('e') else {
            return f.write_str(&s);
        };
        let (mantissa, exponent) = s.split_at(e_index);
        let exponent = &exponent[1..];
        f.write_str(mantissa)?;
        f.write_str(" \\times 10^")?;
        if exponent.len() == 1 {
            // A lone digit needs no braces, and a lone digit is necessarily positive.
            f.write_str(exponent)
        } else {
            f.write_char('{')?;
            f.write_str(exponent)?;
            f.write_char('}')
        }
    }
}
