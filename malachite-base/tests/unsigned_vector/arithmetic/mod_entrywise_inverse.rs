// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModEntrywiseInverse, ModEntrywiseMul, ModIsReduced};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_entrywise_inverse::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_entrywise_inverse() {
    let test = |s, m, out: Option<&str>| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let r = (&v).mod_entrywise_inverse(m);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.clone().mod_entrywise_inverse(m), r);
        assert_eq!(unsigned_vector_mod_entrywise_inverse_naive(&v, m), r);
    };
    test("()", 1, Some("()"));
    test("()", 5, Some("()"));
    test("(0)", 1, Some("(0)"));
    test("(0, 0)", 1, Some("(0, 0)"));
    test("(1, 2, 3, 4)", 7, Some("(1, 4, 5, 2)"));
    test("(1, 2, 3)", 6, None);
    test("(0, 1)", 7, None);
    test("(5)", 6, Some("(5)"));
    test("(254, 253)", 255, Some("(254, 127)"));
    test("(3, 4)", 255, None);
}

#[test]
fn test_mod_entrywise_inverse_u64() {
    let v = UnsignedVector::<u64>::from_str("(2, 18446744073709551614, 7)").unwrap();
    assert_eq!(
        v.mod_entrywise_inverse(u64::MAX).unwrap().to_string(),
        "(9223372036854775808, 18446744073709551614, 15811494920322472813)"
    );
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 7)").unwrap()).mod_entrywise_inverse(7);
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_2() {
    // The modulus is zero.
    (&UnsignedVector::<u8>::from_str("()").unwrap()).mod_entrywise_inverse(0);
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_3() {
    UnsignedVector::<u8>::from_str("(1, 7)")
        .unwrap()
        .mod_entrywise_inverse(7);
}

fn mod_entrywise_inverse_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let r = (&v).mod_entrywise_inverse(m);
        assert_eq!(v.clone().mod_entrywise_inverse(m), r);
        assert_eq!(unsigned_vector_mod_entrywise_inverse_naive(&v, m), r);
        // There are inverses exactly when every element is coprime to `m`.
        assert_eq!(r.is_some(), v.elements.iter().all(|&x| x.coprime_with(m)));
        if let Some(r) = r {
            check_inverses(&v, &r, m);
        }

        // Keeping only the invertible elements always gives inverses.
        let u = unsigned_vector_retain_units(&v, m);
        let r = (&u).mod_entrywise_inverse(m).unwrap();
        check_inverses(&u, &r, m);
    });
}

fn check_inverses<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, r: &UnsignedVector<T>, m: T) {
    // The inverses are reduced, there is one for each element, and each times its element is 1.
    assert!(r.mod_is_reduced(&m));
    assert_eq!(r.dimension(), v.dimension());
    let ones = UnsignedVector {
        elements: vec![T::ONE % m; v.elements.len()],
    };
    assert_eq!(v.mod_entrywise_mul(r, m), ones);
    // Inverting again gives back the original vector.
    assert_eq!(r.mod_entrywise_inverse(m).as_ref(), Some(v));
}

#[test]
fn mod_entrywise_inverse_properties() {
    apply_fn_to_unsigneds!(mod_entrywise_inverse_properties_helper);
}
