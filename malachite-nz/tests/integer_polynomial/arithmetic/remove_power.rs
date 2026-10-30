// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

// Checks the removal of the largest power of `q` from `p` in every way of calling it, against the
// naive version; checks that `q` to the power times the result is `p`, and that `q` does not divide
// the result unless it is 0; and returns it.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial) -> (IntegerPolynomial, u64) {
    let (s, e) = p.remove_power(q);
    assert!(s.is_valid());
    assert_eq!(p.clone().remove_power(q.clone()), (s.clone(), e));
    assert_eq!(p.clone().remove_power(q), (s.clone(), e));
    assert_eq!(p.remove_power(q.clone()), (s.clone(), e));
    let mut t = p.clone();
    assert_eq!(t.remove_power_assign(q), e);
    assert_eq!(t, s);
    let mut t = p.clone();
    assert_eq!(t.remove_power_assign(q.clone()), e);
    assert_eq!(t, s);
    assert_eq!(remove_power_naive(p, q), (s.clone(), e));
    assert_eq!(q.pow(e) * &s, *p);
    if s != IntegerPolynomial::ZERO {
        assert!(!(&s).divisible_by(q));
    }
    (s, e)
}

#[test]
fn test_remove_power() {
    let test = |p, q, out: &str, out_e| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let (s, e) = verify(&p, &q);
        assert_eq!(s.to_string(), out);
        assert_eq!(e, out_e);
    };
    // - the dividend is 0
    test("0", "x+1", "0", 0);
    // - not divisible
    test("x^2+1", "x+1", "x^2+1", 0);
    // - divisible several times
    test("2*x^4+9*x^3+15*x^2+11*x+3", "x+1", "2*x+3", 3);
    // - a non-monic factor
    test("4*x^3+24*x^2+21*x+5", "2*x+1", "x+5", 2);
    // - a constant factor
    test("12*x^2+8", "2", "3*x^2+2", 2);
    // - a negative constant factor
    test("12*x^2+8", "-2", "3*x^2+2", 2);
    // - a constant factor and a dividend that vanishes at 1
    test("4*x-4", "2", "x-1", 2);
    // - a factor that vanishes at 1
    test("x^3+x^2-5*x+3", "x-1", "x+3", 2);
    // - a factor whose value at 1 is 1
    test("x^3+x^2-x+2", "x^2-x+1", "x+2", 1);
    // - the dividend is shorter
    test("x+1", "x^2+1", "x+1", 0);
}

#[test]
#[should_panic]
fn remove_power_fail_1() {
    // The divisor is 0.
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .remove_power(IntegerPolynomial::ZERO);
}

#[test]
#[should_panic]
fn remove_power_fail_2() {
    // The divisor is a unit.
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .remove_power(IntegerPolynomial::from_str("-1").unwrap());
}

#[test]
#[should_panic]
fn remove_power_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.remove_power_assign(IntegerPolynomial::from_str("1").unwrap());
}

#[test]
fn remove_power_properties() {
    integer_polynomial_pair_gen_var_5().test_properties(|(p, q)| {
        let e = verify(&p, &q).1;
        assert!(p == IntegerPolynomial::ZERO || e != 0);
        // Multiplying by the factor adds 1 to the exponent.
        if p != IntegerPolynomial::ZERO {
            let pq = &p * &q;
            assert_eq!(verify(&pq, &q).1, e + 1);
        }
    });
}
