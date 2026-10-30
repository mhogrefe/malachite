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
use malachite_base::polynomial::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::pseudo_div_rem::*;

// Checks the pseudo-quotient, pseudo-remainder, and exponent of `p` by `q` in every way of calling
// them, against the naive version and each algorithm; checks that $\ell^d p = qs + r$ with $\deg r
// < \deg q$, that $d$ is the least such exponent, and that the classical pseudo-division is the
// same up to a power of $\ell$; and returns them.
fn verify(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    let (s, r, d) = p.pseudo_div_rem(q);
    assert!(s.is_valid());
    assert!(r.is_valid());
    assert_eq!(
        p.clone().pseudo_div_rem(q.clone()),
        (s.clone(), r.clone(), d)
    );
    assert_eq!(p.clone().pseudo_div_rem(q), (s.clone(), r.clone(), d));
    assert_eq!(p.pseudo_div_rem(q.clone()), (s.clone(), r.clone(), d));
    let mut t = p.clone();
    assert_eq!(t.pseudo_div_assign_rem(q), (r.clone(), d));
    assert_eq!(t, s);
    let mut t = p.clone();
    assert_eq!(t.pseudo_div_assign_rem(q.clone()), (r.clone(), d));
    assert_eq!(t, s);
    assert_eq!(p.pseudo_div(q), (s.clone(), d));
    assert_eq!(p.clone().pseudo_div(q.clone()), (s.clone(), d));
    let mut t = p.clone();
    assert_eq!(t.pseudo_div_assign(q), d);
    assert_eq!(t, s);
    assert_eq!(p.pseudo_rem(q), (r.clone(), d));
    assert_eq!(p.clone().pseudo_rem(q.clone()), (r.clone(), d));
    let mut t = p.clone();
    assert_eq!(t.pseudo_rem_assign(q), d);
    assert_eq!(t, r);
    assert_eq!(pseudo_div_rem_naive(p, q), (s.clone(), r.clone(), d));
    assert_eq!(
        pseudo_div_rem_basecase_wrapper(p, q),
        (s.clone(), r.clone(), d)
    );
    assert_eq!(
        pseudo_div_rem_divide_and_conquer_wrapper(p, q),
        (s.clone(), r.clone(), d)
    );
    let lead = q.coefficients_asc().last().unwrap();
    let scale = IntegerPolynomial::from(lead.pow(d));
    assert_eq!(&scale * p, q * &s + &r);
    assert!(r.len() < q.len());
    if d != 0 {
        assert!(
            !s.coefficients_asc()
                .iter()
                .chain(r.coefficients_asc())
                .all(|x| x.divisible_by(lead))
        );
    }
    if p.len() >= q.len() {
        let e = p.len() - q.len() + 1;
        let f = IntegerPolynomial::from(lead.pow(e - d));
        assert_eq!(pseudo_div_rem_cohen(p, q), (&f * &s, &f * &r));
    }
    (s, r, d)
}

#[test]
fn test_pseudo_div_rem() {
    let test = |p, q, out_s, out_r, out_d| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let (s, r, d) = verify(&p, &q);
        assert_eq!(s.to_string(), out_s);
        assert_eq!(r.to_string(), out_r);
        assert_eq!(d, out_d);
    };
    // - the dividend is 0
    test("0", "x+1", "0", "0", 0);
    // - the dividend is shorter than the divisor
    test("3*x+5", "2*x^2+1", "0", "3*x+5", 0);
    // - a monic divisor
    test("x^3+2*x+1", "x+1", "x^2-x+3", "-2", 0);
    // - a divisor whose leading coefficient divides
    test("6*x^2+3*x+9", "3*x+1", "6*x+1", "26", 1);
    // - a nonunit leading coefficient
    test("x^3+2*x+1", "2*x+1", "4*x^2-2*x+9", "-1", 3);
    // - a negative leading coefficient
    test("x^2+1", "-3*x+2", "-3*x-2", "13", 2);
    // - a constant divisor
    test("5*x^2+3", "2", "5*x^2+3", "0", 1);
    // - a leading coefficient of -1
    test("x^2+3*x+2", "-x+5", "-x-8", "42", 0);
    // - several factors of the leading coefficient
    test("x^4+x+1", "2*x^2+x+1", "4*x^2-2*x-1", "11*x+9", 3);
    // - the remainder is 0
    test("6*x^2+5*x+1", "2*x+1", "3*x+1", "0", 0);
}

// Every algorithm, at lengths that reach each case of divide-and-conquer, against the naive
// version.
#[test]
fn test_pseudo_div_rem_algorithms() {
    let coefficients = |n: usize, seed: i64| -> Vec<Integer> {
        (0..n)
            .map(|i| {
                let i = i64::try_from(i).unwrap();
                Integer::from((i * i * 7 + i * seed + 3) % 101 - 50)
            })
            .collect()
    };
    for &(p_len, q_len) in &[(20, 17), (40, 30), (55, 30), (200, 30), (130, 20), (300, 100)] {
        for lead in [1, 2, -3, 6] {
            let p = IntegerPolynomial::from_coefficients_asc(coefficients(p_len, 1));
            let mut ys = coefficients(q_len, 2);
            *ys.last_mut().unwrap() = Integer::from(lead);
            let q = IntegerPolynomial::from_coefficients_asc(ys);
            verify(&p, &q);
            // - an exact division, whose least exponent is 0
            let pq = &p * &q;
            assert_eq!(verify(&pq, &q), (p, IntegerPolynomial::ZERO, 0));
        }
    }
}

#[test]
#[should_panic]
fn pseudo_div_rem_fail() {
    IntegerPolynomial::from_str("x+1")
        .unwrap()
        .pseudo_div_rem(IntegerPolynomial::ZERO);
}

#[test]
#[should_panic]
fn pseudo_div_assign_rem_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.pseudo_div_assign_rem(IntegerPolynomial::ZERO);
}

#[test]
fn pseudo_div_rem_properties() {
    integer_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        verify(&p, &q);
    });

    integer_polynomial_pair_gen_var_3().test_properties(|(p, q)| {
        // Dividing a multiple of the divisor leaves no remainder.
        let pq = &p * &q;
        let (s, r, d) = verify(&pq, &q);
        assert_eq!(r, IntegerPolynomial::ZERO);
        assert_eq!(d, 0);
        assert_eq!(s, p);
    });
}
