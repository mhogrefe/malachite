// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModPowerOf2Lcm, ModPowerOf2Rem};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_lcm::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the LCM of `p` and `q` modulo $2^k$ in every way of calling it, against the naive version;
// checks that it is 0 or monic, and a multiple of both; and returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let l = p.checked_mod_power_of_2_lcm(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_lcm(q.clone(), pow), l);
    assert_eq!(p.clone().checked_mod_power_of_2_lcm(q, pow), l);
    assert_eq!(p.checked_mod_power_of_2_lcm(q.clone(), pow), l);
    assert_eq!(mod_power_of_2_lcm_naive(p, q, pow), l);
    if pow == 1 {
        assert!(l.is_some());
        assert_eq!(q.checked_mod_power_of_2_lcm(p, pow), l);
    }
    if let Some(l) = &l {
        assert!(l.is_valid());
        assert!(l.mod_power_of_2_is_reduced(pow));
        if *l != UnsignedPolynomial::ZERO {
            assert!(l.is_monic());
            assert_eq!(l.mod_power_of_2_rem(p, pow), UnsignedPolynomial::ZERO);
            assert_eq!(l.mod_power_of_2_rem(q, pow), UnsignedPolynomial::ZERO);
        }
    }
    l
}

fn test_checked_mod_power_of_2_lcm_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|g| g.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - modulo 1
    // - pow == 0
    test("0", "0", 0, Some("0"));
    // - the first is 0
    test("0", "3*x+5", 3, Some("0"));
    // - odd constants
    test("3", "5", 3, Some("1"));
    // - an even leading coefficient
    test("2*x+1", "x+1", 3, None);
    // - modulo 2
    test("x^2+x", "x^2+1", 1, Some("x^3+x"));
    // - a nontrivial GCD
    test("x^3+x^2+3*x+3", "x^2+3*x+2", 3, Some("x^4+3*x^3+5*x^2+x+6"));
    // - an even leading coefficient met along the way
    test("x^2+1", "x+3", 2, None);
}

#[test]
fn test_checked_mod_power_of_2_lcm() {
    apply_fn_to_unsigneds!(test_checked_mod_power_of_2_lcm_helper);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_lcm_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_lcm(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_lcm_fail_2() {
    // The power is too large.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .checked_mod_power_of_2_lcm(UnsignedPolynomial::from_str("x+1").unwrap(), 9);
}

fn checked_mod_power_of_2_lcm_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );
}

#[test]
fn checked_mod_power_of_2_lcm_properties() {
    apply_fn_to_unsigneds!(checked_mod_power_of_2_lcm_properties_helper);
}
