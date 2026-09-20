// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::float::NiceFloat;
use crate::strings::latex::ToLatex;
use alloc::string::ToString;
use core::fmt::{Display, Formatter, Result, Write};

macro_rules! impl_to_latex {
    ($t:ident) => {
        impl ToLatex for $t {
            /// Writes a primitive integer as a LaTeX math-mode fragment.
            ///
            /// The fragment is identical to the [`Display`] output. LaTeX typesets decimal digits,
            /// and a leading `-` before them, correctly in math mode, so nothing needs escaping or
            /// marking up.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::latex#fmt_latex).
            #[inline]
            fn fmt_latex(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }
    };
}
apply_to_primitive_ints!(impl_to_latex);

macro_rules! impl_to_latex_primitive_float {
    ($t:ident) => {
        impl ToLatex for $t {
            /// Writes a primitive float as a LaTeX math-mode fragment.
            ///
            /// The infinities become `\infty` and `-\infty`, and `NaN` becomes `\text{NaN}`, which
            /// typesets it upright rather than as a product of three italic variables. The zeros
            /// keep their signs, as `0.0` and `-0.0`.
            ///
            /// A finite value starts from its [`NiceFloat`](crate::num::float::NiceFloat)
            /// representation, the shortest string that round-trips. If that representation uses an
            /// exponent, it is rewritten in the LaTeX form: `1.0e-45` becomes `1.0 \times
            /// 10^{-45}`. The exponent is wrapped in braces unless it is a single digit, which a
            /// superscript takes on its own; in practice the shortest representation never has a
            /// single-digit exponent, since it only resorts to one for values at least $10^{13}$ or
            /// below $10^{-5}$. A representation without an exponent is written out unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::latex#fmt_latex).
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
                let s = NiceFloat(*self).to_string();
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
    };
}
apply_to_primitive_floats!(impl_to_latex_primitive_float);
