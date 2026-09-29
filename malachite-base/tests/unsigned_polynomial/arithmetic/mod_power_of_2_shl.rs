// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactInto;
use malachite_base::polynomial::{ModPowerOf2Evaluate, Polynomial};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_3, unsigned_triple_gen_var_17,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_shl::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_power_of_2_shl_helper<T: PrimitiveUnsigned, U: PrimitiveUnsigned>()
where
    UnsignedPolynomial<T>:
        ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    let test = |s, bits: u8, pow, out| {
        let bits = U::from(bits);
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
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
    test("255*x+1", 8, 8, "0");
    test("255*x+1", 200, 8, "0");
    // The degree can drop.
    test("4*x^2+3", 1, 3, "6");
    test("x^2+128*x+1", 1, 8, "2*x^2+2");
    test("128*x^2+64", 1, 8, "128");
    test("255*x+1", 7, 8, "128*x+128");
}

#[test]
fn test_mod_power_of_2_shl() {
    apply_fn_to_unsigneds_and_unsigneds!(test_mod_power_of_2_shl_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_1() {
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_shl(1u8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap()).mod_power_of_2_shl(1u8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_shl_assign(1u8, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_shl_fail_4() {
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_shl(1u8, 9);
}

fn mod_power_of_2_shl_properties_helper<
    T: PrimitiveUnsigned + ModPowerOf2Shl<U, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    UnsignedPolynomial<T>:
        ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>> + ModPowerOf2ShlAssign<U>,
    for<'a> &'a UnsignedPolynomial<T>: ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_3::<T, U>().test_properties(
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

            // Every coefficient is shifted as a primitive integer.
            for i in 0..p.len() {
                assert_eq!(
                    q.coefficient(i),
                    p.coefficient(i).mod_power_of_2_shl(bits, pow)
                );
            }
            // Shifting by the same amount as a u64 gives the same result.
            assert_eq!(
                <&UnsignedPolynomial<T> as ModPowerOf2Shl<u64>>::mod_power_of_2_shl(
                    &p,
                    bits.exact_into(),
                    pow
                ),
                q
            );
            // The degree can only drop, and shifting by at least pow gives zero.
            assert!(q.len() <= p.len());
            if ExactInto::<u64>::exact_into(bits) >= pow {
                assert_eq!(q, UnsignedPolynomial::ZERO);
            }
            // It commutes with negation.
            assert_eq!(
                (&p).mod_power_of_2_neg(pow).mod_power_of_2_shl(bits, pow),
                (&q).mod_power_of_2_neg(pow)
            );
            // Evaluation: q(x) = p(x) shifted, modulo 2^pow.
            let x = T::exact_from(3u8).mod_power_of_2(pow);
            assert_eq!(
                (&q).mod_power_of_2_evaluate(x, pow),
                (&p).mod_power_of_2_evaluate(x, pow)
                    .mod_power_of_2_shl(bits, pow)
            );
        },
    );

    unsigned_triple_gen_var_17::<T, U>().test_properties(|(x, bits, pow)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            UnsignedPolynomial::from(x).mod_power_of_2_shl(bits, pow),
            UnsignedPolynomial::from(x.mod_power_of_2_shl(bits, pow))
        );
    });
}

#[test]
fn mod_power_of_2_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_power_of_2_shl_properties_helper);
}
