// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, DivExact, DivExactAssign, DivisibleBy};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen_var_1,
    integer_polynomial_integer_pair_gen_var_3,
};

#[test]
fn test_div_exact() {
    let test = |s, c, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        // All four combinations of value and reference, and in place with both.
        let q = (&p).div_exact(&c);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).div_exact(c.clone()), q);
        assert_eq!(p.clone().div_exact(&c), q);
        assert_eq!(p.clone().div_exact(c.clone()), q);
        let mut r = p.clone();
        r.div_exact_assign(&c);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p;
        r.div_exact_assign(c);
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("0", "-7", "0");
    // Dividing by 1 leaves the polynomial alone, and dividing by -1 negates it.
    test("x^2-4*x-5", "1", "x^2-4*x-5");
    test("x^2-4*x-5", "-1", "-x^2+4*x+5");
    test("6*x^2-3*x+9", "3", "2*x^2-x+3");
    test("6*x^2-3*x+9", "-3", "-2*x^2+x-3");
    // An interior zero stays.
    test("10*x^3-20", "10", "x^3-2");
    test("-12", "-4", "3");
    // Coefficients and divisor of many limbs.
    test(
        "1000000000000000000000000*x^2-2000000000000000000000000",
        "-1000000000000000000000000",
        "-x^2+2",
    );
    test(
        "1524157875323883675019051998750190521*x+1234567890123456789",
        "1234567890123456789",
        "1234567890123456789*x+1",
    );
}

#[test]
#[should_panic]
fn div_exact_fail() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .div_exact(Integer::ZERO);
}

#[test]
#[should_panic]
fn div_exact_val_ref_fail() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .div_exact(&Integer::ZERO);
}

#[test]
#[should_panic]
fn div_exact_ref_val_fail() {
    let _ = (&IntegerPolynomial::from_str("x").unwrap()).div_exact(Integer::ZERO);
}

#[test]
#[should_panic]
fn div_exact_ref_ref_fail() {
    let _ = (&IntegerPolynomial::from_str("x").unwrap()).div_exact(&Integer::ZERO);
}

#[test]
#[should_panic]
fn div_exact_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x").unwrap();
    p.div_exact_assign(Integer::ZERO);
}

#[test]
#[should_panic]
fn div_exact_assign_ref_fail() {
    let mut p = IntegerPolynomial::from_str("x").unwrap();
    p.div_exact_assign(&Integer::ZERO);
}

#[test]
fn div_exact_properties() {
    integer_polynomial_integer_pair_gen_var_3().test_properties(|(p, c)| {
        let q = (&p).div_exact(&c);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!((&p).div_exact(c.clone()), q);
        assert_eq!(p.clone().div_exact(&c), q);
        assert_eq!(p.clone().div_exact(c.clone()), q);
        let mut r = p.clone();
        r.div_exact_assign(&c);
        assert_eq!(r, q);
        let mut r = p.clone();
        r.div_exact_assign(c.clone());
        assert_eq!(r, q);

        // The degree is unchanged.
        assert_eq!(q.degree(), p.degree());
        // Dividing by the negated divisor negates the quotient.
        assert_eq!((&p).div_exact(-&c), -&q);
        // Coefficient by coefficient, this is the `Integer` operation.
        for (x, y) in p.coefficients_asc().iter().zip(q.coefficients_asc()) {
            assert_eq!(*y, x.div_exact(&c));
        }
        // The divisor divides the content, and the quotient's content is the content divided by the
        // divisor.
        let content = (&p).content();
        assert!((&content).divisible_by(c.unsigned_abs_ref()));
        assert_eq!(q.content(), content.div_exact(c.unsigned_abs_ref()));
    });

    integer_polynomial_integer_pair_gen_var_1().test_properties(|(p, c)| {
        // Undoing a multiplication by a nonzero `Integer` recovers the polynomial.
        let multiple = IntegerPolynomial::from_coefficients_asc(
            p.coefficients_asc()
                .iter()
                .map(|x| x * &c)
                .collect::<Vec<_>>(),
        );
        assert_eq!(multiple.div_exact(&c), p);
    });

    integer_polynomial_gen().test_properties(|p| {
        // Dividing by 1 changes nothing, and dividing by -1 negates.
        assert_eq!((&p).div_exact(Integer::ONE), p);
        assert_eq!((&p).div_exact(Integer::NEGATIVE_ONE), -&p);
        // A nonzero polynomial divided by its content has content 1.
        if p != IntegerPolynomial::ZERO {
            let content = Integer::from((&p).content());
            assert_eq!(p.div_exact(content).content(), 1u32);
        }
    });
}
