// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{bool_gen, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_gen;

#[test]
fn test_from_integer() {
    let test = |s, out| {
        let p = IntegerPolynomial::from(Integer::from_str(s).unwrap());
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("123", "123");
    test("1000000000000000000000", "1000000000000000000000");
}

#[test]
fn test_from_primitive() {
    assert_eq!(IntegerPolynomial::from(0u32).to_string(), "0");
    assert_eq!(IntegerPolynomial::from(123u8).to_string(), "123");
    assert_eq!(IntegerPolynomial::from(123u16).to_string(), "123");
    assert_eq!(IntegerPolynomial::from(123u64).to_string(), "123");
    assert_eq!(IntegerPolynomial::from(123u128).to_string(), "123");
    assert_eq!(IntegerPolynomial::from(123usize).to_string(), "123");
    assert_eq!(IntegerPolynomial::from(false).to_string(), "0");
    assert_eq!(IntegerPolynomial::from(true).to_string(), "1");
}

#[test]
fn from_integer_properties() {
    integer_gen().test_properties(|x| {
        let p = IntegerPolynomial::from(x.clone());
        assert!(p.is_valid());
        // A number becomes the constant polynomial with that value: degree 0, or no degree when the
        // number is zero.
        if x == 0u32 {
            assert_eq!(p, IntegerPolynomial::ZERO);
            assert_eq!(p.degree(), None);
        } else {
            assert_eq!(p.degree(), Some(0));
        }
        assert_eq!(*p.coefficient(0), x);
        assert_eq!(p.to_string(), x.to_string());
    });

    unsigned_gen::<u64>().test_properties(|x| {
        assert_eq!(
            IntegerPolynomial::from(x),
            IntegerPolynomial::from(Integer::from(x))
        );
    });

    bool_gen().test_properties(|b| {
        assert_eq!(
            IntegerPolynomial::from(b),
            IntegerPolynomial::from(Integer::from(b))
        );
    });
}
