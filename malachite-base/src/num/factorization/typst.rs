// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::factorization::factor::Factors;
use crate::strings::typst::ToTypst;
use core::fmt::{Display, Formatter, Result, Write};

impl<T: PrimitiveUnsigned, const N: usize> ToTypst for Factors<T, N> {
    /// Writes a [`Factors`] as a Typst math-mode fragment.
    ///
    /// The fragment is the factorization written out as a product of prime powers, so the
    /// factorization of 90 becomes `2 times 3^2 times 5`. An exponent of 1 is left off, as it is
    /// when a factorization is written by hand.
    ///
    /// The factorization of 1 has no factors at all, and becomes `1`: the empty product, which is
    /// what it multiplies out to.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of distinct prime
    /// factors.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::Factor;
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(1u32.factor().to_typst().to_string(), "1");
    /// assert_eq!(2u32.factor().to_typst().to_string(), "2");
    /// assert_eq!(4u32.factor().to_typst().to_string(), "2^2");
    /// assert_eq!(90u32.factor().to_typst().to_string(), "2 times 3^2 times 5");
    /// assert_eq!(1024u32.factor().to_typst().to_string(), "2^(10)");
    /// ```
    ///
    /// | value              | fragment              |
    /// |--------------------|-----------------------|
    /// | `1u32.factor()`    | `1`                   |
    /// | `2u32.factor()`    | `2`                   |
    /// | `4u32.factor()`    | `2^2`                 |
    /// | `90u32.factor()`   | `2 times 3^2 times 5` |
    /// | `1024u32.factor()` | `2^(10)`              |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        let mut any = false;
        for (factor, exponent) in self.clone() {
            if any {
                f.write_str(" times ")?;
            }
            any = true;
            Display::fmt(&factor, f)?;
            if exponent != 1 {
                f.write_char('^')?;
                // A lone digit needs no grouping, and an exponent of 1 is never written.
                if exponent < 10 {
                    Display::fmt(&exponent, f)?;
                } else {
                    f.write_char('(')?;
                    Display::fmt(&exponent, f)?;
                    f.write_char(')')?;
                }
            }
        }
        if any {
            Ok(())
        } else {
            // The factorization of 1 is the empty product.
            f.write_char('1')
        }
    }
}
