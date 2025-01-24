// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::CheckedLogBase2;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;

impl EqAbs<Rational> for Float {
    #[inline]
    fn eq_abs(&self, other: &Rational) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                exponent,
                significand,
                ..
            }) => {
                *other != 0
                    && if let Some(log_d) = other.denominator_ref().checked_log_base_2() {
                        let n = other.numerator_ref();
                        i64::from(*exponent)
                            == i64::exact_from(n.significant_bits()) - i64::exact_from(log_d)
                            && significand.cmp_normalized(n) == Equal
                    } else {
                        false
                    }
            }
            _ => false,
        }
    }
}

impl EqAbs<Float> for Rational {
    #[inline]
    fn eq_abs(&self, other: &Float) -> bool {
        other.eq_abs(self)
    }
}
