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
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{CheckedModPowerOf2Gcd, ModPowerOf2CoprimeWith};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_coprime_with::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

// Checks the coprimality of `p` and `q` modulo $2^k$ in every way of calling it, symmetry,
// agreement with the naive version, and agreement with the checked GCD where that succeeds; and
// returns it.
fn verify<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    let coprime = p.mod_power_of_2_coprime_with(q, pow);
    assert_eq!(
        p.clone().mod_power_of_2_coprime_with(q.clone(), pow),
        coprime
    );
    assert_eq!(p.clone().mod_power_of_2_coprime_with(q, pow), coprime);
    assert_eq!(p.mod_power_of_2_coprime_with(q.clone(), pow), coprime);
    assert_eq!(q.mod_power_of_2_coprime_with(p, pow), coprime);
    assert_eq!(mod_power_of_2_coprime_with_naive(p, q, pow), coprime);
    if pow != 0
        && let Some(g) = p.checked_mod_power_of_2_gcd(q, pow)
    {
        assert_eq!(g == UnsignedPolynomial::one(), coprime);
    }
    coprime
}

fn test_mod_power_of_2_coprime_with_helper<T: PrimitiveUnsigned>() {
    let test = |p, q, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(p).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, pow), out);
    };
    // - pow == 0
    test("0", "0", 0, true);
    // - both 0
    // - pow != 0
    test("0", "0", 3, false);
    // - an odd constant and 0
    test("3", "0", 3, true);
    // - an even constant and 0
    // - xs.last() == Some(&0)
    test("2", "0", 3, false);
    // - a unit that is not a constant: 1 + 2x, whose square is 1 modulo 8
    test("2*x+1", "0", 3, true);
    // - a common factor modulo 2
    test("x", "0", 1, false);
    test("x+1", "x+3", 3, false);
    test("2*x^2+x+1", "4*x+2", 3, false);
    // - coprime modulo 2
    test("x^2+x+1", "x", 2, true);
    test("4*x^2+x", "2*x+1", 3, true);
    // - pow == 8
    test("255*x+1", "254", 8, false);
    test("255*x+1", "253", 8, true);
}

#[test]
fn test_mod_power_of_2_coprime_with() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_coprime_with_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_coprime_with_fail_1() {
    UnsignedPolynomial::<u8>::from_str("x+8")
        .unwrap()
        .mod_power_of_2_coprime_with(UnsignedPolynomial::from_str("x").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_coprime_with_fail_2() {
    UnsignedPolynomial::<u8>::from_str("x")
        .unwrap()
        .mod_power_of_2_coprime_with(UnsignedPolynomial::from_str("x").unwrap(), 9);
}

fn mod_power_of_2_coprime_with_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, _, pow)| {
            assert_eq!(
                (&p).mod_power_of_2_coprime_with(UnsignedPolynomial::ZERO, pow),
                (&p).mod_power_of_2_coprime_with(&p, pow)
            );
            if pow != 0 {
                assert!((&p).mod_power_of_2_coprime_with(UnsignedPolynomial::one(), pow));
            }
        },
    );
}

#[test]
fn mod_power_of_2_coprime_with_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_coprime_with_properties_helper);
}
