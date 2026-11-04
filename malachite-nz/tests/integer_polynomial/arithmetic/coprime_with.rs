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
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen_var_7, integer_polynomial_triple_gen,
};

// Checks the coprimality of `p` and `q` in every way of calling it, symmetry, and agreement with
// the GCD; and returns it.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial) -> bool {
    let coprime = p.coprime_with(q);
    assert_eq!(p.clone().coprime_with(q.clone()), coprime);
    assert_eq!(p.clone().coprime_with(q), coprime);
    assert_eq!(p.coprime_with(q.clone()), coprime);
    assert_eq!(q.coprime_with(p), coprime);
    assert_eq!(p.gcd(q) == IntegerPolynomial::one(), coprime);
    coprime
}

#[test]
fn test_coprime_with() {
    let test = |p, q, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q), out);
    };
    // - both 0
    // - !p.content().coprime_with(q.content())
    test("0", "0", false);
    // - units and 0
    // - p.content().coprime_with(q.content())
    test("1", "0", true);
    test("-1", "0", true);
    // - a non-unit constant and 0
    test("2", "0", false);
    test("x+1", "0", false);
    // - contents that are not coprime
    test("2*x+2", "4", false);
    test("6*x+6", "4*x^2-4", false);
    // - coprime contents and a common factor
    test("x^2-1", "x+1", false);
    test("3*x^2-3", "2*x+2", false);
    // - coprime: the GCD is 1, although 2 and x generate a proper ideal
    test("2", "x", true);
    test("x^2+1", "x+1", true);
    test("2*x+1", "3*x+1", true);
    test(
        "100000000000000000000*x^6-100000000000000000000",
        "100000000000000000000*x^5+100000000000000000000*x^4-x-1",
        false,
    );
    test(
        "100000000000000000000*x^6-99999999999999999999",
        "100000000000000000000*x^5+100000000000000000000*x^4-x-1",
        true,
    );
}

#[test]
fn coprime_with_properties() {
    integer_polynomial_pair_gen_var_7().test_properties(|(p, q)| {
        verify(&p, &q);
        assert_eq!(
            (&p).coprime_with(IntegerPolynomial::ZERO),
            (&p).coprime_with(&p)
        );
        assert!((&p).coprime_with(IntegerPolynomial::one()));
    });

    integer_polynomial_triple_gen().test_properties(|(a, b, c)| {
        assert_eq!(
            (&a * &b).coprime_with(&c),
            (&a).coprime_with(&c) && (&b).coprime_with(&c)
        );
    });
}
