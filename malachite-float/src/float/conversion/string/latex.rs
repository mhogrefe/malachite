// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{ComparableFloat, ComparableFloatRef, Float};
use alloc::string::ToString;
use core::fmt::{Formatter, Result, Write};
use malachite_base::strings::latex::ToLatex;

impl ToLatex for Float {
    /// Writes a [`Float`] as a LaTeX math-mode fragment.
    ///
    /// This is as the primitive floats are written. A NaN becomes `\text{NaN}` and the infinities
    /// become `\infty` and `-\infty`. A finite [`Float`] is written as [`Display`] writes it, with
    /// the exponent, if there is one, lifted into a real power of ten: `1.3e30` becomes ``1.3
    /// \times 10^{30}``.
    ///
    /// As with [`Display`], the digit count is determined by the [`Float`]'s precision rather than
    /// by its value, and the two zeros are kept apart.
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
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
    /// };
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_latex_string(), r"\text{NaN}");
    /// assert_eq!(Float::INFINITY.to_latex_string(), r"\infty");
    /// assert_eq!(Float::NEGATIVE_INFINITY.to_latex_string(), r"-\infty");
    /// assert_eq!(Float::ZERO.to_latex_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.to_latex_string(), "-0.0");
    /// assert_eq!(Float::ONE.to_latex_string(), "1.0");
    /// assert_eq!(Float::from(1.5).to_latex_string(), "1.5");
    /// assert_eq!(
    ///     Float::power_of_2(100u64).to_latex_string(),
    ///     r"1.3 \times 10^{30}"
    /// );
    /// assert_eq!(
    ///     Float::power_of_2(-100i64).to_latex_string(),
    ///     r"7.9 \times 10^{-31}"
    /// );
    /// ```
    ///
    /// | value                       | fragment             | renders as           |
    /// |-----------------------------|----------------------|----------------------|
    /// | `Float::NAN`                | `\text{NaN}`         | $\text{NaN}$         |
    /// | `Float::INFINITY`           | `\infty`             | $\infty$             |
    /// | `Float::ONE`                | `1.0`                | $1.0$                |
    /// | `Float::power_of_2(100u64)` | `1.3 \times 10^{30}` | $1.3 \times 10^{30}$ |
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

impl ToLatex for ComparableFloat {
    /// Writes a [`ComparableFloat`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`Float`]'s own: the wrapper exists to give an equality and an
    /// ordering that tell more [`Float`]s apart than the usual ones do, and does not change what
    /// the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`Float`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(Float::ONE).to_latex_string(), "1.0");
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}

impl ToLatex for ComparableFloatRef<'_> {
    /// Writes a [`ComparableFloatRef`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the wrapped [`Float`]'s own: the wrapper exists to give an equality and an
    /// ordering that tell more [`Float`]s apart than the usual ones do, and does not change what
    /// the value is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for [`Float`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_float::{ComparableFloatRef, Float};
    ///
    /// let x = Float::ONE;
    /// assert_eq!(ComparableFloatRef(&x).to_latex_string(), "1.0");
    /// ```
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.0.fmt_latex(f)
    }
}
