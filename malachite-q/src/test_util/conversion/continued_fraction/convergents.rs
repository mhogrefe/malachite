// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::conversion::continued_fraction::to_continued_fraction::RationalContinuedFraction;
use crate::conversion::traits::ContinuedFraction;
use crate::Rational;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[derive(Clone, Debug)]
pub struct ConvergentsAlt {
    first: bool,
    floor: Integer,
    xs: Vec<Natural>,
    cf: RationalContinuedFraction,
}

impl Iterator for ConvergentsAlt {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        if self.first {
            self.first = false;
            Some(Rational::from(&self.floor))
        } else if let Some(n) = self.cf.next() {
            self.xs.push(n);
            Some(Rational::from_continued_fraction_ref(
                &self.floor,
                self.xs.iter(),
            ))
        } else {
            self.xs.clear();
            None
        }
    }
}

pub fn convergents_alt(x: Rational) -> ConvergentsAlt {
    let (floor, cf) = x.continued_fraction();
    ConvergentsAlt {
        first: true,
        floor,
        xs: Vec::new(),
        cf,
    }
}
