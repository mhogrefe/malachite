// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::ReciprocalAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

pub fn from_continued_fraction_alt(floor: Integer, xs: Vec<Natural>) -> Rational {
    if xs.is_empty() {
        Rational::from(floor)
    } else {
        let mut x = Rational::ZERO;
        let mut first = true;
        for n in xs.into_iter().rev() {
            if first {
                first = false;
            } else {
                x.reciprocal_assign();
            }
            x += Rational::from(n);
        }
        if !first {
            x.reciprocal_assign();
        }
        x + Rational::from(floor)
    }
}
