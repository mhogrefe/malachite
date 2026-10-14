// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModDiv, ModIsReduced, ModMul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_div::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_div() {
    let test = |s, c, m, out: Option<&str>| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let r = (&v).mod_div(c, m);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.clone().mod_div(c, m), r);
        assert_eq!(unsigned_vector_mod_div_naive(&v, c, m), r);
    };
    test("()", 0, 1, Some("()"));
    test("()", 3, 7, Some("()"));
    test("(0)", 0, 1, Some("(0)"));
    test("(1, 2, 3)", 3, 7, Some("(5, 3, 1)"));
    test("(2, 4, 0)", 2, 6, Some("(1, 2, 0)"));
    test("(2, 3)", 2, 6, None);
    test("(0, 0)", 0, 5, Some("(0, 0)"));
    test("(0, 1)", 0, 5, None);
    test("(4, 2)", 4, 8, None);
    test("(254, 1)", 2, 255, Some("(127, 128)"));
}

#[test]
fn test_mod_div_u64() {
    let v = UnsignedVector::<u64>::from_str("(1, 2, 18446744073709551614)").unwrap();
    assert_eq!(
        v.mod_div(2, u64::MAX).unwrap().to_string(),
        "(9223372036854775808, 1, 9223372036854775807)"
    );
}

#[test]
#[should_panic]
fn mod_div_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 7)").unwrap()).mod_div(1, 7);
}

#[test]
#[should_panic]
fn mod_div_fail_2() {
    // The divisor is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 2)").unwrap()).mod_div(7, 7);
}

#[test]
#[should_panic]
fn mod_div_fail_3() {
    // The modulus is zero.
    (&UnsignedVector::<u8>::from_str("()").unwrap()).mod_div(0, 0);
}

#[test]
#[should_panic]
fn mod_div_fail_4() {
    UnsignedVector::<u8>::from_str("(1, 7)")
        .unwrap()
        .mod_div(1, 7);
}

fn mod_div_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(v, c, m)| {
        let r = (&v).mod_div(c, m);
        assert_eq!(v.clone().mod_div(c, m), r);
        assert_eq!(unsigned_vector_mod_div_naive(&v, c, m), r);
        // There are quotients exactly when gcd(c, m) divides every element.
        let g = c.gcd(m);
        assert_eq!(r.is_some(), v.elements.iter().all(|&x| x.divisible_by(g)));
        if let Some(r) = r {
            // The quotients are reduced, there is one for each element, and multiplying them by c
            // gives back the elements.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(r.dimension(), v.dimension());
            assert_eq!((&r).mod_mul(c, m), v);
        }
        // Multiplying by c and dividing by it again gives back the vector when c is invertible.
        if g == T::ONE {
            assert_eq!((&v).mod_mul(c, m).mod_div(c, m), Some(v));
        }
    });
}

#[test]
fn mod_div_properties() {
    apply_fn_to_unsigneds!(mod_div_properties_helper);
}
