// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::factorization::traits::Factor;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::modular::mod_gcd::mod_gcd_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;

// The polynomial `p`, reduced modulo `m`.
fn reduce<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(p.coefficients_asc().iter().map(|&x| x % m).collect())
}

// Whether `p` and `q` are coprime modulo `m`, by factoring `m`: they are exactly when their GCD
// modulo every prime factor of `m` is 1.
pub fn mod_coprime_with_naive<T: PrimitiveUnsigned + Factor>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> bool
where
    T::FACTORS: IntoIterator<Item = (T, u8)>,
{
    m.factor().into_iter().all(|(r, _)| {
        mod_gcd_naive(&reduce(p, r), &reduce(q, r), r).unwrap() == UnsignedPolynomial::one()
    })
}
