// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::test_util::gaussian_integer::arithmetic::mul::gaussian_integer_mul_naive;

// A reference implementation of Gaussian-integer squaring: the naive four-product multiplication
// with both operands equal. `x * x` would not do here: the multiplication operator detects aliased
// operands and delegates to the squaring algorithm under test.
pub fn gaussian_integer_square_naive(x: &GaussianInteger) -> GaussianInteger {
    gaussian_integer_mul_naive(x, x)
}
