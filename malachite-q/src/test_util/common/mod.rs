// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::integer::Integer;
use num::{BigInt, BigRational};

impl From<&BigRational> for Rational {
    fn from(n: &BigRational) -> Self {
        Self::from_integers(Integer::from(n.numer()), Integer::from(n.denom()))
    }
}

impl From<&Rational> for BigRational {
    fn from(n: &Rational) -> Self {
        let mut q = Self::new_raw(
            BigInt::from(n.numerator_ref()),
            BigInt::from(n.denominator_ref()),
        );
        if *n < 0 {
            q = -q;
        }
        q
    }
}

impl From<&rug::Rational> for Rational {
    fn from(n: &rug::Rational) -> Self {
        Self::from_integers(Integer::from(n.numer()), Integer::from(n.denom()))
    }
}

impl From<&Rational> for rug::Rational {
    fn from(n: &Rational) -> Self {
        let mut q = Self::from((
            rug::Integer::from(n.numerator_ref()),
            rug::Integer::from(n.denominator_ref()),
        ));
        if *n < 0 {
            q = -q;
        }
        q
    }
}
