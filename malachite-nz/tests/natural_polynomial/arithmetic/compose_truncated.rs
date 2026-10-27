// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Compose, ComposeTruncated, ComposeTruncatedAssign, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_3,
    natural_polynomial_unsigned_pair_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::compose_truncated::*;
use std::panic::catch_unwind;

// Checks the composition as power series of `p` and `q`, truncated to `len` coefficients, in every
// way of calling it, against the naive version, the truncation of the full composition, and the
// composition of the corresponding `IntegerPolynomial`s, and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, len: u64) -> NaturalPolynomial {
    let r = p.compose_truncated(q, len);
    assert!(r.is_valid());
    assert_eq!(p.clone().compose_truncated(q.clone(), len), r);
    assert_eq!(p.clone().compose_truncated(q, len), r);
    assert_eq!(p.compose_truncated(q.clone(), len), r);
    let mut s = p.clone();
    s.compose_truncated_assign(q, len);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.compose_truncated_assign(q.clone(), len);
    assert_eq!(s, r);
    assert_eq!(natural_polynomial_compose_truncated_naive(p, q, len), r);
    assert_eq!(p.compose(q).truncate(len), r);
    assert_eq!(
        IntegerPolynomial::from(p).compose_truncated(IntegerPolynomial::from(q), len),
        IntegerPolynomial::from(&r)
    );
    r
}

#[test]
fn test_compose_truncated() {
    let test = |p, q, len, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, len).to_string(), out);
    };
    // - the length is 0
    test("3*x^2+2*x+1", "x", 0, "0");
    // - the outer polynomial is 0
    test("0", "x", 5, "0");
    // - the inner polynomial is 0: the constant coefficient
    test("3*x^2+2*x+1", "0", 5, "1");
    // - a constant outer polynomial
    test("7", "x^2+3*x", 5, "7");
    // - length 1: the constant coefficient
    test("6*x^2+5*x+4", "x^2+x", 1, "4");
    // - Horner's rule, with the whole composition shorter than the length
    test("3*x^2+2*x+1", "x^2+x", 10, "3*x^4+6*x^3+5*x^2+2*x+1");
    // - Horner's rule, truncating
    test("4*x^3+3*x^2+2*x+1", "x^2+x", 5, "15*x^4+10*x^3+5*x^2+2*x+1");
    // - the Brent-Kung algorithm
    test(
        "3*x^29+14*x^28+8*x^27+3*x^26+2*x^25+14*x^24+3*x^23+2*x^22+10*x^21+6*x^20+3*x^19+2*x^18+14\
        *x^17+6*x^16+2*x^15+2*x^14+6*x^13+7*x^12+6*x^11+15*x^10+8*x^9+2*x^8+3*x^7+10*x^6+10*x^5+5*\
        x^4+12*x^3+16*x^2+6*x+5",
        "8*x^19+3*x^18+13*x^17+11*x^16+x^15+15*x^14+x^13+x^12+12*x^10+4*x^9+12*x^8+15*x^7+7*x^6+7*\
        x^5+16*x^4+12*x^3+x^2+16*x",
        17,
        "115075582775966145506*x^16+2745765783511032892*x^15+186659908452898693*x^14+2964452653140\
        4104*x^13+2176541109932816*x^12+119759984464576*x^11+16864025023454*x^10+559688253444*x^9+\
        9771596973*x^8+909623850*x^7+172207414*x^6+10687466*x^5+343152*x^4+49736*x^3+4102*x^2+96*x\
        +5",
    );
    // - a monomial inner polynomial
    test(
        "254*x^14+14*x^13+143*x^12+x^11+256*x^10+74*x^9+121*x^8+243*x^7+149*x^6+58*x^5+98*x^4+123*\
        x^3+101*x^2+166*x+129",
        "3*x^2",
        13,
        "108621*x^12+14094*x^10+7938*x^8+3321*x^6+909*x^4+498*x^2+129",
    );
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "254*x^14+14*x^13+143*x^12+x^11+256*x^10+74*x^9+121*x^8+243*x^7+149*x^6+58*x^5+98*x^4+123*\
        x^3+101*x^2+166*x+129",
        "x^5+2*x^4",
        12,
        "101*x^10+404*x^9+404*x^8+166*x^5+332*x^4+129",
    );
}

#[test]
fn compose_truncated_fail() {
    // - the inner polynomial has a nonzero constant coefficient
    let p = NaturalPolynomial::from_str("x+1").unwrap();
    let q = NaturalPolynomial::from_str("x+1").unwrap();
    assert_panic!((&p).compose_truncated(&q, 3));
    let q = NaturalPolynomial::from_str("x+1").unwrap();
    assert_panic!(p.clone().compose_truncated(q, 3));
    // - even when the length is 0
    let q = NaturalPolynomial::from_str("x+1").unwrap();
    assert_panic!((&p).compose_truncated(&q, 0));
}

#[test]
#[should_panic]
fn compose_truncated_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.compose_truncated_assign(&NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn compose_truncated_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_3().test_properties(
        |(p, q, len)| {
            let r = verify(&p, &q, len);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).compose_truncated(&q, len),
                (&p).compose_truncated((&q).compose_truncated(&q, len), len)
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(p.truncate(len).compose_truncated(q.truncate(len), len), r);
        },
    );

    natural_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        // Composing with x truncates.
        assert_eq!(verify(&p, &NaturalPolynomial::x(), len), p.truncate(len));
        // Composing with 0 gives the constant coefficient.
        assert_eq!(
            verify(&p, &NaturalPolynomial::ZERO, len),
            NaturalPolynomial::from(p.coefficient(0).clone()).truncate(len)
        );
        // Composing x with a polynomial truncates it.
        if p != NaturalPolynomial::ZERO && *p.coefficient(0) == 0u32 {
            assert_eq!(verify(&NaturalPolynomial::x(), &p, len), p.truncate(len));
        }
    });
}
