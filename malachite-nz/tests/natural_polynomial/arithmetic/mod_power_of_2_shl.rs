// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModPowerOf2Evaluate, Polynomial};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_unsigned_triple_gen_var_1,
    natural_unsigned_unsigned_triple_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_shl::*;

fn test_mod_power_of_2_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial>,
    u64: ExactFrom<T>,
{
    let test = |s, bits: u8, pow, out| {
        let bits = T::from(bits);
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = (&p).mod_power_of_2_shl(bits, pow);
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_shl(bits, pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_shl_assign(bits, pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_power_of_2_shl_naive(&p, bits, pow), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, 0, "0");
    test("0", 5, 3, "0");
    // Shifting by 0 changes nothing.
    test("5*x^2+x+3", 0, 3, "5*x^2+x+3");
    // Every coefficient is shifted and reduced.
    test("5*x^2+x+3", 1, 3, "2*x^2+2*x+6");
    test("5*x^2+x+3", 2, 3, "4*x^2+4*x+4");
    // Shifting by at least pow gives zero.
    test("5*x^2+x+3", 3, 3, "0");
    test("5*x^2+x+3", 10, 3, "0");
    // The degree can drop.
    test("4*x^2+3", 1, 3, "6");
    test("4*x^2+2*x", 2, 3, "0");
    test(
        "x+1",
        99,
        100,
        "633825300114114700748351602688*x+633825300114114700748351602688",
    );
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_shl_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_1() {
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_shl(1u8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_2() {
    (&NaturalPolynomial::from_str("8*x+1").unwrap()).mod_power_of_2_shl(1u8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_3() {
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    p.mod_power_of_2_shl_assign(1u8, 3);
}

fn mod_power_of_2_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalPolynomial: ModPowerOf2Shl<T, Output = NaturalPolynomial> + ModPowerOf2ShlAssign<T>,
    for<'a> &'a NaturalPolynomial:
        ModPowerOf2Shl<T, Output = NaturalPolynomial> + Shl<T, Output = NaturalPolynomial>,
    Natural: ModPowerOf2Shl<T, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, bits, pow)| {
            let q = (&p).mod_power_of_2_shl(bits, pow);
            assert!(q.is_valid());
            assert!(q.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(p.clone().mod_power_of_2_shl(bits, pow), q);
            let mut r = p.clone();
            r.mod_power_of_2_shl_assign(bits, pow);
            assert!(r.is_valid());
            assert_eq!(r, q);
            assert_eq!(mod_power_of_2_shl_naive(&p, bits, pow), q);

            // It is the plain shift, reduced.
            assert_eq!((&p << bits).mod_power_of_2(pow), q);
            // Shifting by the same amount as a u64 gives the same result.
            assert_eq!(
                <&NaturalPolynomial as ModPowerOf2Shl<u64>>::mod_power_of_2_shl(
                    &p,
                    u64::exact_from(bits),
                    pow
                ),
                q
            );
            // The degree can only drop, and shifting by at least pow gives zero.
            assert!(q.len() <= p.len());
            if u64::exact_from(bits) >= pow {
                assert_eq!(q, NaturalPolynomial::ZERO);
            }
            // It commutes with negation.
            assert_eq!(
                (&p).mod_power_of_2_neg(pow).mod_power_of_2_shl(bits, pow),
                (&q).mod_power_of_2_neg(pow)
            );
            // Evaluation: q(3) = p(3) shifted, modulo 2^pow.
            let x = Natural::from(3u32).mod_power_of_2(pow);
            assert_eq!(
                (&q).mod_power_of_2_evaluate(&x, pow),
                (&p).mod_power_of_2_evaluate(&x, pow)
                    .mod_power_of_2_shl(bits, pow)
            );
        },
    );

    natural_unsigned_unsigned_triple_gen_var_6::<T>().test_properties(|(x, bits, pow)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            NaturalPolynomial::from(x.clone()).mod_power_of_2_shl(bits, pow),
            NaturalPolynomial::from(x.mod_power_of_2_shl(bits, pow))
        );
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_shl_properties_helper);
}
