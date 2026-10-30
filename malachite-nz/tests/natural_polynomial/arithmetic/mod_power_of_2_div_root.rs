// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2Mul, ModPowerOf2Neg,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_root::*;

// Checks the quotient and remainder of `p` by $x - c$ in every way of calling them, against the
// naive version; checks that the remainder is the value of `p` at `c` and that $(x - c)q + r$ is
// `p`; and returns them.
fn verify(p: &NaturalPolynomial, c: &Natural, pow: u64) -> (NaturalPolynomial, Natural) {
    let (q, r) = p.mod_power_of_2_div_root(c, pow);
    assert!(q.is_valid());
    assert_eq!(
        p.clone().mod_power_of_2_div_root(c.clone(), pow),
        (q.clone(), r.clone())
    );

    let mut s = p.clone();
    assert_eq!(s.mod_power_of_2_div_root_assign(c, pow), r);
    assert_eq!(s, q);
    assert_eq!(
        mod_power_of_2_div_root_naive(p, c, pow),
        (q.clone(), r.clone())
    );
    assert_eq!(p.mod_power_of_2_evaluate(c, pow), r);
    let neg = c.mod_power_of_2_neg(pow);
    let divisor =
        NaturalPolynomial::from_coefficients_asc(vec![neg, Natural::ONE.mod_power_of_2(pow)]);
    let constant = NaturalPolynomial::from_coefficients_asc(vec![r.clone()]);
    assert_eq!(
        divisor
            .mod_power_of_2_mul(&q, pow)
            .mod_power_of_2_add(constant, pow),
        *p
    );
    (q, r)
}

#[test]
fn test_mod_power_of_2_div_root() {
    let test = |p, c, pow: u64, out: &str, out_r: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let c = Natural::from_str(c).unwrap();

        let (q, r) = verify(&p, &c, pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(r.to_string(), out_r);
    };
    // - the polynomial is 0
    test("0", "3", 3, "0", "0");
    // - a constant
    test("5", "3", 3, "0", "5");
    // - c is 0
    test("x^2+3*x+2", "0", 200, "x+3", "2");
    // - a root
    test(
        "x^2+3*x+2",
        "1606938044258990275541962092341162602522202993782792835301375",
        200,
        "x+2",
        "0",
    );
    // - not a root
    test("x^3+2*x+1", "3", 200, "x^2+3*x+11", "34");
    // - an exponent of one limb
    test("x^3+2*x+1", "3", 8, "x^2+3*x+11", "34");
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_root(Natural::ONE, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_fail_2() {
    // c is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_root(Natural::from(8u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_power_of_2_div_root_assign(Natural::from(8u32), 3);
}

#[test]
fn mod_power_of_2_div_root_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, c, pow)| {
        verify(&p, &c, pow);
    });

    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, _, pow)| {
        // Dividing by x shifts the coefficients down.
        let (q, r) = verify(&p, &Natural::ZERO, pow);
        assert_eq!(q, (&p).div_power_of_x(1));
        assert_eq!(r, *p.coefficient(0));
    });
}
