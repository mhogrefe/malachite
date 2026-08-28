// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::fmt::{Display, Formatter, Result, Write};

impl Display for GaussianInteger {
    // Purely real values display like their real part, and 0 is "0". Otherwise the imaginary unit
    // is written as "i", "-i", or with an integer coefficient, and a nonzero real part precedes it
    // with a joining sign: "1+i", "1-i", "2+3i", "2-3i".
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.imaginary == 0 {
            return Display::fmt(&self.real, f);
        }
        if self.real != 0 {
            Display::fmt(&self.real, f)?;
            if self.imaginary > 0 {
                f.write_char('+')?;
            }
        }
        if self.imaginary == 1 {
            f.write_char('i')
        } else if self.imaginary == -1 {
            f.write_str("-i")
        } else {
            Display::fmt(&self.imaginary, f)?;
            f.write_char('i')
        }
    }
}
