// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::PrimitivePart;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::resultant::{
    resultant_euclidean, resultant_modular,
};
use malachite_nz::integer_polynomial::arithmetic::resultant_extended_gcd::resultant_extended_gcd;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_7,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::resultant::resultant_sylvester;

// The resultant by the subresultant algorithm and the modular algorithm, against the determinant of
// the Sylvester matrix.
#[test]
fn test_resultant() {
    let test = |p, q, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let (xs, ys) = if p.len() >= q.len() {
            (p.coefficients_asc(), q.coefficients_asc())
        } else {
            (q.coefficients_asc(), p.coefficients_asc())
        };
        let r = resultant_sylvester(xs, ys);
        assert_eq!(r.to_string(), out);
        assert_eq!(resultant_euclidean(xs, ys), r);
        assert_eq!(resultant_modular(xs, ys), r);
    };
    // - a constant second polynomial
    // - len2 == 1, the first
    // - len2 == 1, the second
    test("x^2+1", "3", "9");
    // - both lengths even
    // - d != e
    // - both_even(len1, len2)
    // - len2 <= 1
    // - negative
    // - curr_bits < bound
    test("x+1", "x+2", "1");
    // - a zero resultant
    // - len1 == 0
    test("x^2-1", "x-1", "0");
    // - lengths of different parities
    // - !(negative)
    test("x^2+1", "x+1", "2");
    // - a prime dividing a leading coefficient, in the modular algorithm
    // - l.rem_limb_ref(p) == 0
    test("9223372036854775837*x+1", "x+2", "18446744073709551673");
    // - contents other than 1
    // - a_content != 1u32
    // - b_content != 1u32
    test("6*x^2+3", "4*x+2", "72");
    // - a degree difference of 2
    test("x^4+1", "x^2+x+1", "1");
    // - a negative resultant
    test("x^2-2", "x+1", "-1");
}

// The resultant of primitive polynomials and the cofactors that combine them into it.
#[test]
fn test_resultant_extended_gcd() {
    let test = |p, q, out: (&str, &str, &str)| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let (r, s, t) = resultant_extended_gcd(&p, &q);
        if r != 0u32 {
            assert_eq!(&s * &p + &t * &q, IntegerPolynomial::from(r.clone()));
        }
        assert_eq!(
            (r.to_string(), s.to_string(), t.to_string()),
            (out.0.to_string(), out.1.to_string(), out.2.to_string())
        );
    };
    // - the first is 0
    // - p.coefficients.len() < q.coefficients.len()
    // - q.coefficients.is_empty()
    test("0", "x+1", ("0", "0", "0"));
    // - the first shorter than the second
    // - !stabilised
    // - first
    // - stabilised, the second
    // - product.significant_bits() > bound
    // - !(len2 > 144 || len2_cubed.saturating_mul(bits) > 6000000)
    test("x+1", "x^2+1", ("2", "-x+1", "1"));
    // - a zero resultant
    // - r == 0u32
    test("x^2-1", "x-1", ("0", "0", "0"));
    // - a prime dividing a leading coefficient
    // - reduce(lead1, p) == 0 || reduce(lead2, p) == 0 || r_mod == 0
    // - stabilised, the first
    // - t1.last() == Some(&0)
    // - !(t1.len() == 1 && t1[0] == r_mod)
    // - !(first)
    test(
        "9223372036854775837*x+1",
        "x+2",
        ("18446744073709551673", "-1", "9223372036854775837"),
    );
    // - one prime
    test("x^2+1", "x+1", ("2", "1", "-x+1"));
    // - cofactors too large for one prime
    // - t1.len() == 1 && t1[0] == r_mod
    test(
        "x^2+1180591620717411303424",
        "x+3",
        ("1180591620717411303433", "1", "-x+3"),
    );
}

// The resultant of polynomials with large coefficients, which the dispatcher sends to the modular
// algorithm, and the cofactors that combine them into it.
#[test]
fn test_resultant_extended_gcd_algorithms() {
    let p = IntegerPolynomial::from_str(
        "x^29+170141183460469231731687303715884105727*x^28+340282366920938463463374607431768211457\
        *x+3",
    )
    .unwrap();
    let q = IntegerPolynomial::from_str(
        "x^29+340282366920938463463374607431768211457*x^7+170141183460469231731687303715884105729",
    )
    .unwrap();
    // - len2 > 144 || len2_cubed.saturating_mul(bits) > 6000000
    let (r, s, t) = resultant_extended_gcd(&p, &q);
    assert_eq!(&s * &p + &t * &q, IntegerPolynomial::from(r.clone()));
    assert_eq!(
        r,
        resultant_sylvester(p.coefficients_asc(), q.coefficients_asc())
    );
}

// The resultant of nonzero polynomials, the first at least as long, by the subresultant algorithm
// and the modular algorithm, against the determinant of the Sylvester matrix.
#[test]
fn resultant_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let (xs, ys) = if p.len() >= q.len() {
            (p.coefficients_asc(), q.coefficients_asc())
        } else {
            (q.coefficients_asc(), p.coefficients_asc())
        };
        if ys.is_empty() {
            return;
        }
        let r = resultant_sylvester(xs, ys);
        assert_eq!(resultant_euclidean(xs, ys), r);
        assert_eq!(resultant_modular(xs, ys), r);
    });
}

// The resultant of the primitive parts and the cofactors that combine them into it.
#[test]
fn resultant_extended_gcd_properties() {
    integer_polynomial_pair_gen_var_7().test_properties(|(p, q)| {
        let (p, q) = (p.primitive_part(), q.primitive_part());
        let (r, s, t) = resultant_extended_gcd(&p, &q);
        if p == IntegerPolynomial::ZERO || q == IntegerPolynomial::ZERO || r == 0u32 {
            assert_eq!(s, IntegerPolynomial::ZERO);
            assert_eq!(t, IntegerPolynomial::ZERO);
            if p != IntegerPolynomial::ZERO && q != IntegerPolynomial::ZERO {
                let (xs, ys) = if p.len() >= q.len() {
                    (p.coefficients_asc(), q.coefficients_asc())
                } else {
                    (q.coefficients_asc(), p.coefficients_asc())
                };
                assert_eq!(resultant_sylvester(xs, ys), Integer::ZERO);
            }
            return;
        }
        assert_eq!(&s * &p + &t * &q, IntegerPolynomial::from(r.clone()));
        if q.len() > 1 {
            assert!(s.len() < q.len());
        }
        if p.len() > 1 {
            assert!(t.len() < p.len());
        }
        let (xs, ys) = if p.len() >= q.len() {
            (p.coefficients_asc(), q.coefficients_asc())
        } else {
            (q.coefficients_asc(), p.coefficients_asc())
        };
        assert_eq!(resultant_sylvester(xs, ys), r);
    });
}
