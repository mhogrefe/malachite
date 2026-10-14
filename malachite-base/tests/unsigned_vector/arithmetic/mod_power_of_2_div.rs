// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModDiv, ModPowerOf2Div, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_1;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_power_of_2_div::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_power_of_2_div() {
    let test = |s, c, pow, out: Option<&str>| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let r = (&v).mod_power_of_2_div(c, pow);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.clone().mod_power_of_2_div(c, pow), r);
        assert_eq!(unsigned_vector_mod_power_of_2_div_naive(&v, c, pow), r);
    };
    test("()", 0, 0, Some("()"));
    test("()", 3, 4, Some("()"));
    test("(0)", 0, 0, Some("(0)"));
    test("(0, 0)", 0, 4, Some("(0, 0)"));
    test("(0, 1)", 0, 4, None);
    test("(1, 2, 3)", 3, 4, Some("(11, 6, 1)"));
    test("(2, 4, 0)", 2, 4, Some("(1, 2, 0)"));
    test("(2, 3)", 2, 4, None);
    test("(4, 8, 12)", 4, 4, Some("(1, 2, 3)"));
    test("(255, 254)", 255, 8, Some("(1, 2)"));
}

#[test]
fn test_mod_power_of_2_div_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551615, 2, 6)").unwrap();
    assert_eq!(
        v.mod_power_of_2_div(3, 64).unwrap().to_string(),
        "(6148914691236517205, 6148914691236517206, 2)"
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 8)").unwrap()).mod_power_of_2_div(1, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_2() {
    // The divisor is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 2)").unwrap()).mod_power_of_2_div(8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_3() {
    // pow is greater than the width of the type.
    (&UnsignedVector::<u8>::from_str("()").unwrap()).mod_power_of_2_div(0, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_fail_4() {
    UnsignedVector::<u8>::from_str("(1, 8)")
        .unwrap()
        .mod_power_of_2_div(1, 3);
}

fn mod_power_of_2_div_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(v, c, pow)| {
        let r = (&v).mod_power_of_2_div(c, pow);
        assert_eq!(v.clone().mod_power_of_2_div(c, pow), r);
        assert_eq!(unsigned_vector_mod_power_of_2_div_naive(&v, c, pow), r);
        // There are quotients exactly when every element has one.
        assert_eq!(
            r.is_some(),
            v.elements
                .iter()
                .all(|&x| x.mod_power_of_2_div(c, pow).is_some())
        );
        if let Some(r) = &r {
            // The quotients are reduced, there is one for each element, and multiplying them by c
            // gives back the elements.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.dimension(), v.dimension());
            assert_eq!(r.mod_power_of_2_mul(c, pow), v);
        }
        // Modulo a power of 2 below the width of `T`, quotients exist exactly when they do for
        // mod_div, and for an odd divisor the two agree.
        if pow < T::WIDTH {
            let d = (&v).mod_div(c, T::power_of_2(pow));
            assert_eq!(d.is_some(), r.is_some());
            if c.odd() {
                assert_eq!(d, r);
            }
        }
    });
}

#[test]
fn mod_power_of_2_div_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_div_properties_helper);
}
