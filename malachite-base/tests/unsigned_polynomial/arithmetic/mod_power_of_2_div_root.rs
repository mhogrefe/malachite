// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2Mul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::*;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_root::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the quotient and remainder of `p` by $x - c$ in every way of calling them, against the
// naive version; checks that the remainder is the value of `p` at `c` and that $(x - c)q + r$ is
// `p`; and returns them.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    pow: u64,
) -> (UnsignedPolynomial<T>, T) {
    let (q, r) = p.mod_power_of_2_div_root(c, pow);
    assert!(q.is_valid());
    assert_eq!(p.clone().mod_power_of_2_div_root(c, pow), (q.clone(), r));
    let mut s = p.clone();
    assert_eq!(s.mod_power_of_2_div_root_assign(c, pow), r);
    assert_eq!(s, q);
    assert_eq!(mod_power_of_2_div_root_naive(p, c, pow), (q.clone(), r));
    assert_eq!(p.mod_power_of_2_evaluate(c, pow), r);
    let divisor = UnsignedPolynomial::from_coefficients_asc(vec![
        c.wrapping_neg().mod_power_of_2(pow),
        T::ONE.mod_power_of_2(pow),
    ]);
    let constant = UnsignedPolynomial::from_coefficients_asc(vec![r]);
    assert_eq!(
        divisor
            .mod_power_of_2_mul(&q, pow)
            .mod_power_of_2_add(constant, pow),
        *p
    );
    (q, r)
}

fn test_mod_power_of_2_div_root_helper<T: PrimitiveUnsigned>() {
    let test = |p, c: u8, pow: u64, out: &str, out_r: u8| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let (q, r) = verify(&p, T::from(c), pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(r, T::from(out_r));
    };
    // - modulo 1
    test("0", 0, 0, "0", 0);
    // - the polynomial is 0
    test("0", 3, 3, "0", 0);
    // - a constant
    test("5", 3, 3, "0", 5);
    // - c is 0
    test("x^2+3*x+2", 0, 3, "x+3", 2);
    // - a root
    test("x^2+3*x+2", 7, 3, "x+2", 0);
    // - not a root
    test("x^3+2*x+1", 3, 3, "x^2+3*x+3", 2);
    // - the full width
    test("250*x^3+3*x+1", 249, 8, "250*x^2+42*x+221", 246);
}

#[test]
fn test_mod_power_of_2_div_root() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_div_root_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_root(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_fail_2() {
    // c is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_root(8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_root_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    p.mod_power_of_2_div_root_assign(8, 3);
}

fn mod_power_of_2_div_root_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, c, pow)| {
        verify(&p, c, pow);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        // Dividing by x shifts the coefficients down.
        let (q, r) = verify(&p, T::ZERO, pow);
        assert_eq!(q, (&p).div_power_of_x(1));
        assert_eq!(r, p.coefficient(0));
    });
}

#[test]
fn mod_power_of_2_div_root_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_root_properties_helper);
}
