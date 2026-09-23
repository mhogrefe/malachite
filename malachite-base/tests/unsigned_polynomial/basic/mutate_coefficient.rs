// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_pair_gen_var_1;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mutate_coefficient() {
    let test = |s, index, c: u32, out| {
        let mut p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        let ret = p.mutate_coefficient(index, |x| {
            *x = u64::from(c);
            "done"
        });
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(ret, "done");
    };
    test("x^2+3*x+2", 1, 4, "x^2+4*x+2");
    test("x^2+3*x+2", 0, 0, "x^2+3*x");
    // Clearing the leading coefficient lowers the degree, and clearing the only one empties the
    // polynomial.
    test("x^2+3*x+2", 2, 0, "3*x+2");
    test("5", 0, 0, "0");
    // The polynomial grows to reach an index it did not have.
    test("0", 3, 1, "x^3");
    test("x", 5, 2, "2*x^5+x");
    // Growing and then writing zero leaves no trace.
    test("x", 5, 0, "x");
    test("0", 100, 0, "0");
}

#[test]
fn test_mutate_coefficient_returns_closure_value() {
    let mut p = UnsignedPolynomial::<u64>::from_str("x+1").unwrap();
    assert_eq!(p.mutate_coefficient(0, |x| *x), 1);
    assert_eq!(p.to_string(), "x+1");
}

#[test]
fn mutate_coefficient_properties() {
    unsigned_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        // Writing back what was already there changes nothing.
        let mut q = p.clone();
        let old = q.mutate_coefficient(i, |x| *x);
        assert!(q.is_valid());
        assert_eq!(q, p);
        assert_eq!(old, p.coefficient(i));

        // Writing a value puts it there, and the degree is at least the index unless the value is
        // zero.
        let mut q = p.clone();
        q.mutate_coefficient(i, |x| *x = 1);
        assert!(q.is_valid());
        assert_eq!(q.coefficient(i), 1);
        assert!(q.degree().unwrap() >= i);
        // Every other coefficient is untouched.
        for j in 0..25 {
            if j != i {
                assert_eq!(q.coefficient(j), p.coefficient(j));
            }
        }

        // Writing zero and writing back the old value returns the polynomial to itself.
        let mut q = p.clone();
        let old = q.mutate_coefficient(i, |x| {
            let old = *x;
            *x = 0;
            old
        });
        assert!(q.is_valid());
        assert_eq!(q.coefficient(i), 0);
        q.mutate_coefficient(i, |x| *x = old);
        assert_eq!(q, p);
    });
}
