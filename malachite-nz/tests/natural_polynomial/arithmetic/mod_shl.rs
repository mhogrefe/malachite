// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Shl;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModNeg, ModShl, ModShlAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{ModEvaluate, Polynomial};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_triple_gen_var_6, natural_polynomial_unsigned_natural_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_shl::*;

fn test_mod_shl_helper<T: PrimitiveUnsigned>()
where
    NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>
        + ModShlAssign<T, Natural>
        + for<'a> ModShl<T, &'a Natural, Output = NaturalPolynomial>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a, 'b> &'a NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>
        + ModShl<T, &'b Natural, Output = NaturalPolynomial>,
    u64: ExactFrom<T>,
{
    let test = |s, bits: u8, m, out| {
        let bits = T::from(bits);
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_shl(bits, &m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_shl(bits, m.clone()), q);
        assert_eq!(p.clone().mod_shl(bits, &m), q);
        assert_eq!(p.clone().mod_shl(bits, m.clone()), q);
        let mut r = p.clone();
        r.mod_shl_assign(bits, &m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_shl_assign(bits, m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_shl_naive(&p, bits, &m), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "1", "0");
    test("0", 5, "10", "0");
    // Shifting by 0 changes nothing.
    test("3*x^2+x+7", 0, "10", "3*x^2+x+7");
    // Every coefficient is multiplied by 2^bits mod m.
    test("3*x^2+x+7", 1, "10", "6*x^2+2*x+4");
    test("3*x^2+x+7", 3, "10", "4*x^2+8*x+6");
    test("3*x^2+x+7", 4, "11", "4*x^2+5*x+2");
    // m need not be odd, so coefficients can vanish and the degree can drop.
    test("3*x^2+5", 2, "12", "8");
    test("3*x+6", 2, "12", "0");
    test(
        "x+1",
        100,
        "1000000000000000000000000000000",
        "267650600228229401496703205376*x+267650600228229401496703205376",
    );
}

#[test]
fn test_mod_shl() {
    apply_fn_to_unsigneds!(test_mod_shl_helper);
}

#[test]
#[should_panic]
fn mod_shl_fail_1() {
    NaturalPolynomial::from_str("10*x+1")
        .unwrap()
        .mod_shl(1u8, Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_shl_fail_2() {
    (&NaturalPolynomial::from_str("10*x+1").unwrap()).mod_shl(1u8, &Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_shl_fail_3() {
    let mut p = NaturalPolynomial::from_str("10*x+1").unwrap();
    p.mod_shl_assign(1u8, &Natural::from(10u32));
}

#[test]
#[should_panic]
fn mod_shl_fail_4() {
    NaturalPolynomial::ZERO.mod_shl(1u8, Natural::ZERO);
}

fn mod_shl_properties_helper<T: PrimitiveUnsigned>()
where
    NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>
        + ModShlAssign<T, Natural>
        + for<'a> ModShl<T, &'a Natural, Output = NaturalPolynomial>
        + for<'a> ModShlAssign<T, &'a Natural>,
    for<'a, 'b> &'a NaturalPolynomial: ModShl<T, Natural, Output = NaturalPolynomial>
        + ModShl<T, &'b Natural, Output = NaturalPolynomial>
        + Shl<T, Output = NaturalPolynomial>,
    Natural: for<'a> ModShl<T, &'a Natural, Output = Natural>,
    u64: ExactFrom<T>,
{
    natural_polynomial_unsigned_natural_triple_gen_var_1::<T>().test_properties(|(p, bits, m)| {
        let q = (&p).mod_shl(bits, &m);
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(&m));
        // The forms agree.
        assert_eq!((&p).mod_shl(bits, m.clone()), q);
        assert_eq!(p.clone().mod_shl(bits, &m), q);
        assert_eq!(p.clone().mod_shl(bits, m.clone()), q);
        let mut r = p.clone();
        r.mod_shl_assign(bits, &m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_shl_assign(bits, m.clone());
        assert_eq!(r, q);
        assert_eq!(mod_shl_naive(&p, bits, &m), q);

        // It is the plain shift, reduced.
        assert_eq!(&(&p << bits) % &m, q);
        // Shifting by the same amount as a u64 gives the same result.
        assert_eq!(
            <&NaturalPolynomial as ModShl<u64, &Natural>>::mod_shl(&p, u64::exact_from(bits), &m),
            q
        );
        // The degree can only drop.
        assert!(q.len() <= p.len());
        // It commutes with negation.
        assert_eq!((&p).mod_neg(&m).mod_shl(bits, &m), (&q).mod_neg(&m));
        // Evaluation: q(3) = p(3) shifted, modulo m.
        let x = Natural::from(3u32) % &m;
        assert_eq!(
            (&q).mod_evaluate(&x, &m),
            (&p).mod_evaluate(&x, &m).mod_shl(bits, &m)
        );
    });

    natural_natural_unsigned_triple_gen_var_6::<T>().test_properties(|(x, m, bits)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            NaturalPolynomial::from(x.clone()).mod_shl(bits, &m),
            NaturalPolynomial::from(x.mod_shl(bits, &m))
        );
    });
}

#[test]
fn mod_shl_properties() {
    apply_fn_to_unsigneds!(mod_shl_properties_helper);
}
