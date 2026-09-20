// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::float::NiceFloat;
use crate::strings::typst::ToTypst;
use alloc::string::ToString;
use core::fmt::{Display, Formatter, Result, Write};

macro_rules! impl_to_typst {
    ($t:ident) => {
        impl ToTypst for $t {
            /// Writes a primitive integer as a Typst math-mode fragment.
            ///
            /// The fragment is the integer's decimal digits, preceded by a minus sign if it is
            /// negative, which is what Typst math mode already writes a number as.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::typst#fmt_typst).
            #[inline]
            fn fmt_typst(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }
    };
}
apply_to_primitive_ints!(impl_to_typst);

macro_rules! impl_to_typst_primitive_float {
    ($t:ident) => {
        impl ToTypst for $t {
            /// Writes a primitive float as a Typst math-mode fragment.
            ///
            /// A NaN becomes `"NaN"` and the infinities become `infinity` and `-infinity`. A finite
            /// float is written as its [`NiceFloat`] representation, with the exponent, if there is
            /// one, lifted into a real power of ten: `1.23e-3` becomes `1.23 times 10^(-3)`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::typst#fmt_typst).
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
                let s = NiceFloat(*self).to_string();
                let Some(e_index) = s.find('e') else {
                    return f.write_str(&s);
                };
                let (mantissa, exponent) = s.split_at(e_index);
                let exponent = &exponent[1..];
                f.write_str(mantissa)?;
                f.write_str(" times 10^(")?;
                f.write_str(exponent)?;
                f.write_char(')')
            }
        }
    };
}
apply_to_primitive_floats!(impl_to_typst_primitive_float);
