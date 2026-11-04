// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::{ModPowerOf2Add, ModPowerOf2Mul};
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_extended_gcd::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Whether `p` and `q` are coprime modulo $2^k$. If they are not coprime modulo 2, they are not
// coprime modulo $2^k$ either. If they are, the extended GCD modulo 2 gives $s$ and $t$ with $u =
// sp + tq \equiv 1 \pmod 2$; then $u^{2^k} \equiv 1 \pmod{2^k}$, so $v = u^{2^k - 1}$ is an inverse
// of $u$, and $(sv)p + (tv)q \equiv 1 \pmod{2^k}$, which is checked.
pub fn mod_power_of_2_coprime_with_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    if pow == 0 {
        return true;
    }
    let low = |p: &UnsignedPolynomial<T>| {
        UnsignedPolynomial::from_coefficients_asc(
            p.coefficients_asc().iter().map(|&x| x & T::ONE).collect(),
        )
    };
    let (g, s, t) = mod_power_of_2_extended_gcd_naive(&low(p), &low(q), 1).unwrap();
    if g != UnsignedPolynomial::one() {
        return false;
    }
    let u = (&s)
        .mod_power_of_2_mul(p, pow)
        .mod_power_of_2_add((&t).mod_power_of_2_mul(q, pow), pow);
    // v = u * u^2 * u^4 * ... * u^(2^(k - 1)) = u^(2^k - 1).
    let mut v = UnsignedPolynomial::one();
    let mut power = u;
    for _ in 0..pow {
        v = v.mod_power_of_2_mul(&power, pow);
        power = (&power).mod_power_of_2_mul(&power, pow);
    }
    assert_eq!(
        s.mod_power_of_2_mul(&v, pow)
            .mod_power_of_2_mul(p, pow)
            .mod_power_of_2_add(
                t.mod_power_of_2_mul(&v, pow).mod_power_of_2_mul(q, pow),
                pow
            ),
        UnsignedPolynomial::one()
    );
    true
}
