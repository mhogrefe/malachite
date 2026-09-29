// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModNeg, ModPowerOf2Shl, ModShl, ModShlAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{ModEvaluate, Polynomial};
use malachite_base::test_util::generators::{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_4, unsigned_triple_gen_var_18,
};
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_shl::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

fn test_mod_shl_helper<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>()
where
    UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>> + ModShlAssign<U, T>,
    for<'a> &'a UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>,
{
    let test = |s, bits: u8, m: u8, out| {
        let bits = U::from(bits);
        let m = T::from(m);
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_shl(bits, m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_shl(bits, m), q);
        let mut r = p.clone();
        r.mod_shl_assign(bits, m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(mod_shl_naive(&p, bits, m), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, 1, "0");
    test("0", 5, 10, "0");
    // Shifting by 0 changes nothing.
    test("3*x^2+x+7", 0, 10, "3*x^2+x+7");
    // Every coefficient is multiplied by 2^bits mod m.
    test("3*x^2+x+7", 1, 10, "6*x^2+2*x+4");
    test("3*x^2+x+7", 3, 10, "4*x^2+8*x+6");
    test("3*x^2+x+7", 4, 11, "4*x^2+5*x+2");
    test("3*x^2+x+7", 204, 11, "4*x^2+5*x+2");
    // 2 has order 10 modulo 11.
    test("3*x^2+x+7", 200, 11, "3*x^2+x+7");
    test("254*x+1", 1, 255, "253*x+2");
    // m need not be odd, so coefficients can vanish and the degree can drop.
    test("3*x^2+5", 2, 12, "8");
    test("3*x+6", 2, 12, "0");
    test("x+1", 1, 2, "0");
}

#[test]
fn test_mod_shl() {
    apply_fn_to_unsigneds_and_unsigneds!(test_mod_shl_helper);
}

#[test]
#[should_panic]
fn mod_shl_fail_1() {
    UnsignedPolynomial::<u8>::from_str("10*x+1")
        .unwrap()
        .mod_shl(1u8, 10);
}

#[test]
#[should_panic]
fn mod_shl_fail_2() {
    (&UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap()).mod_shl(1u8, 10);
}

#[test]
#[should_panic]
fn mod_shl_fail_3() {
    let mut p = UnsignedPolynomial::<u8>::from_str("10*x+1").unwrap();
    p.mod_shl_assign(1u8, 10);
}

#[test]
#[should_panic]
fn mod_shl_fail_4() {
    UnsignedPolynomial::<u8>::ZERO.mod_shl(1u8, 0);
}

fn mod_shl_properties_helper<
    T: PrimitiveUnsigned + ModShl<U, T, Output = T> + ModShl<u64, T, Output = T>,
    U: PrimitiveUnsigned,
>()
where
    UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>> + ModShlAssign<U, T>,
    for<'a> &'a UnsignedPolynomial<T>: ModShl<U, T, Output = UnsignedPolynomial<T>>
        + ModPowerOf2Shl<U, Output = UnsignedPolynomial<T>>,
{
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_4::<T, U>().test_properties(
        |(p, bits, m)| {
            let q = (&p).mod_shl(bits, m);
            assert!(q.is_valid());
            assert!(q.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(p.clone().mod_shl(bits, m), q);
            let mut r = p.clone();
            r.mod_shl_assign(bits, m);
            assert!(r.is_valid());
            assert_eq!(r, q);
            assert_eq!(mod_shl_naive(&p, bits, m), q);

            // Shifting by the same amount as a u64 gives the same result.
            assert_eq!(
                <&UnsignedPolynomial<T> as ModShl<u64, T>>::mod_shl(&p, bits.exact_into(), m),
                q
            );
            // The degree can only drop.
            assert!(q.len() <= p.len());
            // A power-of-2 modulus gives the same result as a shift modulo that power of 2.
            if m.is_power_of_2() {
                assert_eq!((&p).mod_power_of_2_shl(bits, m.trailing_zeros()), q);
            }
            // It commutes with negation.
            assert_eq!((&p).mod_neg(m).mod_shl(bits, m), (&q).mod_neg(m));
            // Evaluation: q(3) = p(3) shifted, modulo m.
            let x = T::exact_from(3u8) % m;
            assert_eq!(
                (&q).mod_evaluate(x, m),
                (&p).mod_evaluate(x, m).mod_shl(bits, m)
            );
        },
    );

    unsigned_triple_gen_var_18::<T, U>().test_properties(|(x, bits, m)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            UnsignedPolynomial::from(x).mod_shl(bits, m),
            UnsignedPolynomial::from(x.mod_shl(bits, m))
        );
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(mod_shl_properties_helper);
}
