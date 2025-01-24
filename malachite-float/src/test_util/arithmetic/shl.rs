// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use std::ops::Shl;

pub fn shl_naive<T: PrimitiveInt>(x: Float, bits: T) -> Float
where
    Rational: Shl<T, Output = Rational>,
{
    if x.is_normal() {
        let prec = x.significant_bits();
        Float::from_rational_prec(Rational::exact_from(x) << bits, prec).0
    } else {
        x
    }
}
