// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModEntrywiseInverse, ModPowerOf2EntrywiseInverse, ModPowerOf2EntrywiseMul, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_power_of_2_entrywise_inverse::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_power_of_2_entrywise_inverse() {
    let test = |s, pow, out: Option<&str>| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let r = (&v).mod_power_of_2_entrywise_inverse(pow);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.clone().mod_power_of_2_entrywise_inverse(pow), r);
        assert_eq!(
            unsigned_vector_mod_power_of_2_entrywise_inverse_naive(&v, pow),
            r
        );
    };
    test("()", 0, Some("()"));
    test("(0)", 0, Some("(0)"));
    test("(0, 0)", 0, Some("(0, 0)"));
    test("()", 3, Some("()"));
    test("(1, 3, 5, 7)", 4, Some("(1, 11, 13, 7)"));
    test("(1, 2, 3)", 4, None);
    test("(0)", 1, None);
    test("(1)", 1, Some("(1)"));
    test("(255, 3)", 8, Some("(255, 171)"));
    test("(254, 3)", 8, None);
}

#[test]
fn test_mod_power_of_2_entrywise_inverse_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551615, 3, 5)").unwrap();
    assert_eq!(
        v.mod_power_of_2_entrywise_inverse(64).unwrap().to_string(),
        "(18446744073709551615, 12297829382473034411, 14757395258967641293)"
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_inverse_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 8)").unwrap()).mod_power_of_2_entrywise_inverse(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_inverse_fail_2() {
    // pow is greater than the width of the type.
    (&UnsignedVector::<u8>::from_str("()").unwrap()).mod_power_of_2_entrywise_inverse(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_inverse_fail_3() {
    UnsignedVector::<u8>::from_str("(1, 8)")
        .unwrap()
        .mod_power_of_2_entrywise_inverse(3);
}

fn check_inverses<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, r: &UnsignedVector<T>, pow: u64) {
    // The inverses are reduced, there is one for each element, and each times its element is 1.
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(r.dimension(), v.dimension());
    let ones = UnsignedVector {
        elements: vec![T::ONE.mod_power_of_2(pow); v.elements.len()],
    };
    assert_eq!(v.mod_power_of_2_entrywise_mul(r, pow), ones);
    // Inverting again gives back the original vector.
    assert_eq!(r.mod_power_of_2_entrywise_inverse(pow).as_ref(), Some(v));
    // For a power of 2 below the width of `T`, this is `mod_entrywise_inverse`.
    if pow < T::WIDTH {
        assert_eq!(
            v.mod_entrywise_inverse(T::power_of_2(pow)).as_ref(),
            Some(r)
        );
    }
}

fn mod_power_of_2_entrywise_inverse_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let r = (&v).mod_power_of_2_entrywise_inverse(pow);
        assert_eq!(v.clone().mod_power_of_2_entrywise_inverse(pow), r);
        assert_eq!(
            unsigned_vector_mod_power_of_2_entrywise_inverse_naive(&v, pow),
            r
        );
        // There are inverses exactly when the modulus is 1 or every element is odd.
        assert_eq!(r.is_some(), pow == 0 || v.elements.iter().all(|&x| x.odd()));
        if let Some(r) = r {
            check_inverses(&v, &r, pow);
        } else if pow < T::WIDTH {
            assert_eq!((&v).mod_entrywise_inverse(T::power_of_2(pow)), None);
        }

        // Keeping only the odd elements always gives inverses.
        let u = unsigned_vector_retain_odd(&v);
        let r = (&u).mod_power_of_2_entrywise_inverse(pow).unwrap();
        check_inverses(&u, &r, pow);
    });
}

#[test]
fn mod_power_of_2_entrywise_inverse_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_entrywise_inverse_properties_helper);
}
