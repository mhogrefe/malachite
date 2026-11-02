// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::modular::mod_div_rem::mod_div_rem_naive;
use crate::test_util::natural_polynomial::modular::mod_mul::mod_mul_naive;

// The product of `a` and `b` modulo `f`, with coefficients modulo `m`: the schoolbook product,
// reduced by schoolbook division. `f` must be nonzero with a leading coefficient invertible modulo
// `m`, unless `m` is 1.
pub fn mod_mul_mod_polynomial_naive(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    mod_div_rem_naive(&mod_mul_naive(a, b, m), f, m).unwrap().1
}
