// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::logic::traits::SignificantBits;

// This differs from the `precision` function provided by `PrimitiveFloat`. That function returns
// the smallest precision necessary to represent the float, whereas this function returns the
// maximum precision of any float in the same binade. If the float is non-finite or zero, 1 is
// returned.
pub fn alt_precision<T: PrimitiveFloat>(x: T) -> u64 {
    if x.is_finite() && x != T::ZERO {
        let (mantissa, exponent) = x.raw_mantissa_and_exponent();
        if exponent == 0 {
            mantissa.significant_bits()
        } else {
            T::MANTISSA_WIDTH + 1
        }
    } else {
        1
    }
}
