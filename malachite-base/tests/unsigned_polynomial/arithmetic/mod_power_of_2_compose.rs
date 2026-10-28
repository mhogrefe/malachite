// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::{
    ModPowerOf2Compose, ModPowerOf2ComposeAssign, ModPowerOf2Evaluate, Polynomial,
};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_compose::{
    mod_power_of_2_compose_divide_and_conquer, mod_power_of_2_compose_horner,
};

fn check<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let r = p.mod_power_of_2_compose(q, pow);
    assert!(r.is_valid());
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(p.mod_power_of_2_compose(q.clone(), pow), r);
    assert_eq!(p.clone().mod_power_of_2_compose(q, pow), r);
    assert_eq!(p.clone().mod_power_of_2_compose(q.clone(), pow), r);
    let mut s = p.clone();
    s.mod_power_of_2_compose_assign(q, pow);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_compose_assign(q.clone(), pow);
    assert_eq!(s, r);
    if *p != UnsignedPolynomial::ZERO && *q != UnsignedPolynomial::ZERO {
        // Both kernels, whatever the lengths, reduced afterwards.
        for kernel in [mod_power_of_2_compose_horner, mod_power_of_2_compose_divide_and_conquer] {
            let xs = kernel(p.coefficients_asc(), q.coefficients_asc(), pow);
            assert_eq!(
                UnsignedPolynomial::from_coefficients_asc(
                    xs.into_iter().map(|x| x.mod_power_of_2(pow)).collect(),
                ),
                r
            );
        }
    }
    r
}

fn test_mod_power_of_2_compose_helper<T: PrimitiveUnsigned>() {
    let test = |s, t, pow, out| {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::from_str(t).unwrap();
        let r = check(&p, &q, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(mod_power_of_2_compose_naive(&p, &q, pow), r);
    };
    test("0", "0", 0, "0");
    test("0", "2*x+1", 3, "0");
    test("5", "0", 3, "5");
    test("5", "3*x^2+2*x+1", 3, "5");
    test("3*x^2+2*x+1", "0", 4, "1");
    test("3*x^2+2*x+1", "0", 4, "1");
    test("3*x^2+2*x+1", "4", 4, "9");
    test("3*x^2+2*x+1", "5", 3, "6");
    test("x", "5*x^2+4*x+3", 3, "5*x^2+4*x+3");
    test("3*x^2+2*x+1", "x", 4, "3*x^2+2*x+1");
    test("3*x^2+2*x+1", "x+1", 4, "3*x^2+8*x+6");
    test("x^2+2*x+3", "2*x+5", 4, "4*x^2+8*x+6");
    test("x^2", "x+1", 1, "x^2+1");
    test("4*x^3+3*x^2+2*x+1", "x^2+2", 8, "4*x^6+27*x^4+62*x^2+49");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x+3",
        8,
        "5*x^4+64*x^3+53*x^2+156*x+35",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "255*x+3",
        8,
        "5*x^4+192*x^3+53*x^2+100*x+35",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "2*x+3",
        8,
        "80*x^4+212*x^2+56*x+35",
    );
    test("x^2+1", "2*x", 3, "4*x^2+1");
    test("x^3+1", "4*x", 4, "1");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2",
        8,
        "5*x^8+4*x^6+3*x^4+2*x^2+1",
    );
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+x+1",
        8,
        "5*x^8+20*x^7+54*x^6+92*x^5+122*x^4+114*x^3+85*x^2+40*x+15",
    );
    test(
        "x^4+1",
        "3*x^2+2*x+1",
        8,
        "81*x^8+216*x^7+68*x^6+56*x^5+214*x^4+104*x^3+36*x^2+8*x+2",
    );
    test(
        "x^5+x^3+x",
        "x^2+255*x+1",
        8,
        "x^10+251*x^9+15*x^8+226*x^7+46*x^6+202*x^5+51*x^4+219*x^3+22*x^2+247*x+3",
    );
    test(
        "7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "255*x^2+2",
        8,
        "7*x^12+166*x^10+229*x^8+132*x^6+3*x^4+2*x^2+1",
    );
    test(
        "x^8",
        "x^2+x+1",
        8,
        "x^16+8*x^15+36*x^14+112*x^13+10*x^12+248*x^11+16*x^10+248*x^9+83*x^8+248*x^7+16*x^6+248*x\
        ^5+10*x^4+112*x^3+36*x^2+8*x+1",
    );
    test(
        "x^9+1",
        "x^2+x",
        8,
        "x^18+9*x^17+36*x^16+84*x^15+126*x^14+126*x^13+84*x^12+36*x^11+9*x^10+x^9+1",
    );
    test(
        "9*x^10+2*x^9+4*x^8+7*x^7+x^6+5*x^3+2*x+3",
        "x^3+2*x^2+3*x+1",
        5,
        "9*x^30+20*x^29+2*x^28+24*x^27+21*x^26+4*x^25+3*x^24+16*x^23+26*x^22+31*x^21+2*x^20+31*x^1\
        9+2*x^18+23*x^17+16*x^16+16*x^15+3*x^14+24*x^13+6*x^12+31*x^11+5*x^10+21*x^9+29*x^8+16*x^\
        7+4*x^6+29*x^5+26*x^4+22*x^3+22*x^2+28*x+1",
    );
    test(
        "17*x^16+16*x^15+15*x^14+14*x^13+13*x^12+12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*\
        x^4+4*x^3+3*x^2+2*x+1",
        "255*x^2+2*x+1",
        8,
        "17*x^32+224*x^31+192*x^30+64*x^29+55*x^28+252*x^27+148*x^26+160*x^25+84*x^24+224*x^23+192\
        *x^22+64*x^21+236*x^20+144*x^19+208*x^18+64*x^17+222*x^16+32*x^15+192*x^14+192*x^13+66*x^\
        12+232*x^11+152*x^10+128*x^9+116*x^8+224*x^7+128*x^6+192*x^5+156*x^4+144*x^3+16*x^2+192*x\
        +153",
    );
    test(
        "x^11+x^10+x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x+1",
        "2*x^2+2*x",
        4,
        "8*x^6+8*x^5+12*x^4+6*x^2+2*x+1",
    );
    test("3*x^2+7", "4*x+2", 3, "3");
}

#[test]
fn test_mod_power_of_2_compose() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_compose_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_fail_1() {
    // A coefficient is not reduced.
    UnsignedPolynomial::<u8>::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_compose(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_fail_2() {
    // A coefficient of the inner polynomial is not reduced.
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_compose(UnsignedPolynomial::from_str("8*x").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_fail_3() {
    // The modulus is too large for the coefficient type.
    UnsignedPolynomial::<u8>::ZERO.mod_power_of_2_compose(UnsignedPolynomial::ZERO, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_assign_fail() {
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_compose_assign(UnsignedPolynomial::from_str("x+1").unwrap(), 3);
}

fn mod_power_of_2_compose_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let r = check(&p, &q, pow);
            assert_eq!(mod_power_of_2_compose_naive(&p, &q, pow), r);
            // The degree is at most the product of the degrees.
            if p.len() > 1 && q.len() > 1 {
                assert!(r.len() <= (p.len() - 1) * (q.len() - 1) + 1);
            }
            // Evaluating p(q) at a point is evaluating p at q's value there.
            for x in [T::ZERO, T::ONE, T::exact_from(3u8)] {
                let x = x.mod_power_of_2(pow);
                assert_eq!(
                    (&r).mod_power_of_2_evaluate(x, pow),
                    (&p).mod_power_of_2_evaluate((&q).mod_power_of_2_evaluate(x, pow), pow)
                );
            }
            // Composition is associative.
            assert_eq!(
                (&r).mod_power_of_2_compose(&q, pow),
                (&p).mod_power_of_2_compose((&q).mod_power_of_2_compose(&q, pow), pow)
            );
            if pow > 0 {
                // x is an identity on both sides.
                let x = UnsignedPolynomial::from_coefficients_asc(vec![T::ZERO, T::ONE]);
                assert_eq!((&p).mod_power_of_2_compose(&x, pow), p);
                assert_eq!((&x).mod_power_of_2_compose(&p, pow), p);
            }
        },
    );
}

#[test]
fn mod_power_of_2_compose_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_compose_properties_helper);
}
