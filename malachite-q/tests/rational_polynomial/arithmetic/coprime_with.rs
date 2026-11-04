// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::number_theory::traits::{CoprimeWith, Gcd};
use malachite_base::polynomial::Polynomial;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen_var_6, rational_polynomial_triple_gen,
};

// Checks the coprimality of `p` and `q` in every way of calling it, symmetry, and agreement with
// the GCD; and returns it.
fn verify(p: &RationalPolynomial, q: &RationalPolynomial) -> bool {
    let coprime = p.coprime_with(q);
    assert_eq!(p.clone().coprime_with(q.clone()), coprime);
    assert_eq!(p.clone().coprime_with(q), coprime);
    assert_eq!(p.coprime_with(q.clone()), coprime);
    assert_eq!(q.coprime_with(p), coprime);
    assert_eq!(p.gcd(q) == RationalPolynomial::one(), coprime);
    coprime
}

#[test]
fn test_coprime_with() {
    let test = |p, q, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let q = RationalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q), out);
    };
    // - both 0
    // - q.len() == 0
    test("0", "0", false);
    // - a nonzero constant and 0
    test("1/2", "0", true);
    test("x+1", "0", false);
    // - nonzero constants
    // - q.len() == 1
    test("1/2", "-3", true);
    test("x^2+1", "2/3", true);
    // - a common factor
    // - q.len() > 1
    test("1/2*x^2-1/2", "3*x^2+6*x+3", false);
    // - coprime
    test("2", "x", true);
    test("1/2*x^2+1", "x+1/3", true);
}

#[test]
fn coprime_with_properties() {
    rational_polynomial_pair_gen_var_6().test_properties(|(p, q)| {
        verify(&p, &q);
        assert_eq!(
            (&p).coprime_with(RationalPolynomial::ZERO),
            (&p).coprime_with(&p)
        );
        assert!((&p).coprime_with(RationalPolynomial::one()));
    });

    rational_polynomial_triple_gen().test_properties(|(a, b, c)| {
        assert_eq!(
            (&a * &b).coprime_with(&c),
            (&a).coprime_with(&c) && (&b).coprime_with(&c)
        );
    });
}
