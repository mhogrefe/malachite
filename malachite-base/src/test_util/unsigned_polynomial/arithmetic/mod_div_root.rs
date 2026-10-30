// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_div_rem::mod_div_rem_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Divides `p` by  - c$ modulo m by naive long division.
pub fn mod_div_root_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    m: T,
) -> (UnsignedPolynomial<T>, T) {
    let divisor = UnsignedPolynomial::from_coefficients_asc(vec![
        if c == T::ZERO { T::ZERO } else { m - c },
        T::ONE,
    ]);
    let (q, r) = mod_div_rem_naive(p, &divisor, m).unwrap();
    (q, r.coefficient(0))
}
