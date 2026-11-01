// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::foer_sequences::FoerSequence;
use malachite_base::num::arithmetic::traits::{Abs, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::Digits;
use malachite_nz::natural::Natural;
use std::collections::BTreeMap;

// Finds the repeating part by remembering every remainder until one recurs.
pub fn to_digits_naive(x: &Rational, base: &Natural) -> (Vec<Natural>, FoerSequence<Natural>) {
    let x = x.abs();
    let floor = (&x).floor();
    let mut remainder = x - Rational::from(&floor);
    let before_point = floor.unsigned_abs().to_digits_asc(base);
    let mut state_map = BTreeMap::new();
    let mut digits = Vec::new();
    let base = Rational::from(base);
    for i in 0.. {
        if remainder == 0u32 {
            return (before_point, FoerSequence::from_vec(digits));
        }
        if let Some(previous_i) = state_map.insert(remainder.clone(), i) {
            let repeating = digits.drain(previous_i..).collect();
            return (before_point, FoerSequence::from_vecs(digits, repeating));
        }
        remainder *= &base;
        let floor = (&remainder).floor().unsigned_abs();
        digits.push(floor.clone());
        remainder -= Rational::from(floor);
    }
    unreachable!()
}
