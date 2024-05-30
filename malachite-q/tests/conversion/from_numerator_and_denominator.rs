// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CoprimeWith, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen_var_6, unsigned_gen, unsigned_pair_gen_var_12,
    unsigned_unsigned_bool_triple_gen_var_2,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen_var_1, natural_gen, natural_natural_bool_triple_gen_var_1,
    natural_pair_gen_var_5,
};
use malachite_q::Rational;
use num::{BigInt, BigRational};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_from_naturals() {
    let test = |n, d, out| {
        let n = Natural::from_str(n).unwrap();
        let d = Natural::from_str(d).unwrap();
        let x = Rational::from_naturals(n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_naturals_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "5", "0");
    test("3", "6", "1/2");
    test("100", "101", "100/101");
}

#[test]
fn from_naturals_fail() {
    assert_panic!(Rational::from_naturals(Natural::ZERO, Natural::ZERO));
    assert_panic!(Rational::from_naturals(Natural::ONE, Natural::ZERO));
    assert_panic!(Rational::from_naturals_ref(&Natural::ZERO, &Natural::ZERO));
    assert_panic!(Rational::from_naturals_ref(&Natural::ONE, &Natural::ZERO));
}

#[test]
fn from_naturals_properties() {
    natural_pair_gen_var_5().test_properties(|(n, d)| {
        let x = Rational::from_naturals(n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_naturals_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        if (&n).coprime_with(&d) {
            assert_eq!(x.into_numerator_and_denominator(), (n, d));
        }
    });

    natural_gen()
        .test_properties(|n| assert_eq!(Rational::from_naturals_ref(&n, &Natural::ONE), n));
}

fn test_from_unsigneds_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
    Limb: ExactFrom<T>,
{
    let test = |n: u8, d: u8, out| {
        let n = T::from(n);
        let d = T::from(d);
        let x = Rational::from_unsigneds(n, d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        if T::WIDTH == Limb::WIDTH {
            let x_alt = Rational::const_from_unsigneds(Limb::exact_from(n), Limb::exact_from(d));
            assert!(x_alt.is_valid());
            assert!(PartialEq::<Rational>::eq(&x_alt, &x));
        }
    };
    test(0, 1, "0");
    test(0, 5, "0");
    test(3, 6, "1/2");
    test(100, 101, "100/101");
}

fn from_unsigneds_fail_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    assert_panic!(Rational::from_unsigneds(T::ZERO, T::ZERO));
    assert_panic!(Rational::from_unsigneds(T::ONE, T::ZERO));
}

#[test]
fn from_unsigneds_fail() {
    apply_fn_to_unsigneds!(from_unsigneds_fail_helper);
}

#[test]
fn test_from_unsigneds() {
    apply_fn_to_unsigneds!(test_from_unsigneds_helper);
}

#[test]
fn test_const_from_unsigneds() {
    let test = |n: Limb, d: Limb, out| {
        let x = Rational::from_unsigneds(n, d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x_alt = Rational::const_from_unsigneds(Limb::exact_from(n), Limb::exact_from(d));
        assert!(x_alt.is_valid());
        assert_eq!(x_alt.to_string(), out);
    };
    test(0, 1, "0");
    test(0, 5, "0");
    test(3, 6, "1/2");
    test(100, 101, "100/101");
    // The following test case has the highest number of steps in the Euclidean GCD algorithm.
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        7540113804746346429,
        12200160415121876738,
        "7540113804746346429/12200160415121876738",
    );
    #[cfg(not(feature = "32_bit_limbs"))]
    test(
        7540113804746346430,
        12200160415121876738,
        "3770056902373173215/6100080207560938369",
    );
}

#[test]
fn const_from_unsigneds_fail() {
    assert_panic!(Rational::const_from_unsigneds(0, 0));
    assert_panic!(Rational::const_from_unsigneds(1, 0));
}

fn from_unsigneds_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PartialEq<T>,
    Rational: PartialEq<T>,
    Limb: ExactFrom<T>,
{
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(n, d)| {
        let x = Rational::from_unsigneds(n, d);
        assert!(x.is_valid());
        if n.coprime_with(d) {
            let (n_alt, d_alt) = x.clone().into_numerator_and_denominator();
            assert_eq!(n_alt, n);
            assert_eq!(d_alt, d);
        }
        if T::WIDTH == Limb::WIDTH {
            let x_alt = Rational::const_from_unsigneds(Limb::exact_from(n), Limb::exact_from(d));
            assert!(x_alt.is_valid());
            assert!(PartialEq::<Rational>::eq(&x_alt, &x));
        }
    });

    unsigned_gen::<T>().test_properties(|n| assert_eq!(Rational::from_unsigneds(n, T::ONE), n));
}

#[test]
fn from_unsigneds_properties() {
    apply_fn_to_unsigneds!(from_unsigneds_properties_helper);
}

#[test]
fn const_from_unsigneds_properties() {
    unsigned_pair_gen_var_12::<Limb, Limb>().test_properties(|(n, d)| {
        let x = Rational::from_unsigneds(n, d);
        let x_alt = Rational::const_from_unsigneds(n, d);
        assert!(x_alt.is_valid());
        assert_eq!(x_alt, x);
    });
}

#[test]
fn test_from_integers() {
    let test = |s, t, out| {
        let n = Integer::from_str(s).unwrap();
        let d = Integer::from_str(t).unwrap();
        let x = Rational::from_integers(n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_integers_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        assert_eq!(
            BigRational::new(BigInt::from_str(s).unwrap(), BigInt::from_str(t).unwrap())
                .to_string(),
            out
        );
        assert_eq!(
            rug::Rational::from((
                rug::Integer::from_str(s).unwrap(),
                rug::Integer::from_str(t).unwrap()
            ))
            .to_string(),
            out
        );
    };
    test("0", "1", "0");
    test("0", "-1", "0");
    test("0", "5", "0");
    test("0", "-5", "0");
    test("3", "6", "1/2");
    test("3", "-6", "-1/2");
    test("-3", "6", "-1/2");
    test("-3", "-6", "1/2");
    test("100", "101", "100/101");
    test("100", "-101", "-100/101");
    test("-100", "101", "-100/101");
    test("-100", "-101", "100/101");
}

#[test]
fn from_integers_fail() {
    assert_panic!(Rational::from_integers(Integer::ZERO, Integer::ZERO));
    assert_panic!(Rational::from_integers(Integer::ONE, Integer::ZERO));
    assert_panic!(Rational::from_integers_ref(&Integer::ZERO, &Integer::ZERO));
    assert_panic!(Rational::from_integers_ref(&Integer::ONE, &Integer::ZERO));
}

#[test]
fn from_integers_properties() {
    integer_pair_gen_var_1().test_properties(|(n, d)| {
        let x = Rational::from_integers(n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_integers_ref(&n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        assert_eq!(
            Rational::from(&BigRational::new(BigInt::from(&n), BigInt::from(&d))),
            x
        );
        assert_eq!(
            Rational::from(&rug::Rational::from((
                rug::Integer::from(&n),
                rug::Integer::from(&d)
            ))),
            x
        );

        if n != 0 {
            assert_eq!(x >= 0, (n >= 0) == (d >= 0));
        }
        if n >= 0 && d > 0 && (n.unsigned_abs_ref()).coprime_with(d.unsigned_abs_ref()) {
            let (n_2, d_2) = x.into_numerator_and_denominator();
            assert_eq!((Integer::from(n_2), Integer::from(d_2)), (n, d));
        }
    });

    integer_gen()
        .test_properties(|n| assert_eq!(Rational::from_integers_ref(&n, &Integer::ONE), n));
}

fn test_from_signeds_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
    SignedLimb: ExactFrom<T>,
{
    let test = |n: i8, d: i8, out| {
        let n = T::from(n);
        let d = T::from(d);
        let x = Rational::from_signeds(n, d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        if T::WIDTH == Limb::WIDTH {
            let x_alt =
                Rational::const_from_signeds(SignedLimb::exact_from(n), SignedLimb::exact_from(d));
            assert!(x_alt.is_valid());
            assert!(PartialEq::<Rational>::eq(&x_alt, &x));
        }
    };
    test(0, 1, "0");
    test(0, -1, "0");
    test(0, 5, "0");
    test(0, -5, "0");
    test(3, 6, "1/2");
    test(3, -6, "-1/2");
    test(-3, 6, "-1/2");
    test(-3, -6, "1/2");
    test(100, 101, "100/101");
    test(100, -101, "-100/101");
    test(-100, 101, "-100/101");
    test(-100, -101, "100/101");
}

#[test]
fn test_from_signeds() {
    apply_fn_to_signeds!(test_from_signeds_helper);
}

fn from_signeds_fail_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
{
    assert_panic!(Rational::from_signeds(T::ZERO, T::ZERO));
    assert_panic!(Rational::from_signeds(T::ONE, T::ZERO));
}

#[test]
fn from_signeds_fail() {
    apply_fn_to_signeds!(from_signeds_fail_helper);
}

#[test]
fn const_from_signeds_fail() {
    assert_panic!(Rational::const_from_signeds(0, 0));
    assert_panic!(Rational::const_from_signeds(1, 0));
}

fn from_signeds_properties_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
    rug::Rational: From<(T, T)>,
    <T as UnsignedAbs>::Output: CoprimeWith,
    Natural: PartialEq<T>,
    Rational: PartialEq<T>,
    SignedLimb: ExactFrom<T>,
{
    signed_pair_gen_var_6::<T>().test_properties(|(n, d)| {
        let x = Rational::from_signeds(n, d);
        assert!(x.is_valid());

        let x_alt = Rational::from(&rug::Rational::from((n, d)));
        assert!(PartialEq::<Rational>::eq(&x_alt, &x));
        if n != T::ZERO {
            assert_eq!(
                PartialOrd::<u32>::ge(&x, &0u32),
                (n >= T::ZERO) == (d >= T::ZERO)
            );
        }
        if n >= T::ZERO && d > T::ZERO && (n.unsigned_abs()).coprime_with(d.unsigned_abs()) {
            let (n_2, d_2) = x.clone().into_numerator_and_denominator();
            assert_eq!(n_2, n);
            assert_eq!(d_2, d);
        }
        if T::WIDTH == Limb::WIDTH {
            let x_alt =
                Rational::const_from_signeds(SignedLimb::exact_from(n), SignedLimb::exact_from(d));
            assert!(x_alt.is_valid());
            assert!(PartialEq::<Rational>::eq(&x_alt, &x));
        }
    });

    signed_gen::<T>().test_properties(|n| assert_eq!(Rational::from_signeds(n, T::ONE), n));
}

#[test]
fn from_signeds_properties() {
    apply_fn_to_signeds!(from_signeds_properties_helper);
}

#[test]
fn const_from_signeds_properties() {
    signed_pair_gen_var_6::<SignedLimb>().test_properties(|(n, d)| {
        let x = Rational::from_signeds(n, d);
        let x_alt = Rational::const_from_signeds(n, d);
        assert!(x_alt.is_valid());
        assert_eq!(x_alt, x);
    });
}

#[test]
fn test_from_sign_and_naturals() {
    let test = |sign, n, d, out| {
        let n = Natural::from_str(n).unwrap();
        let d = Natural::from_str(d).unwrap();
        let x = Rational::from_sign_and_naturals(sign, n.clone(), d.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Rational::from_sign_and_naturals_ref(sign, &n, &d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(false, "0", "1", "0");
    test(true, "0", "1", "0");
    test(false, "0", "5", "0");
    test(true, "0", "5", "0");
    test(false, "3", "6", "-1/2");
    test(true, "3", "6", "1/2");
    test(false, "100", "101", "-100/101");
    test(true, "100", "101", "100/101");
}

#[test]
fn from_sign_and_naturals_properties() {
    natural_natural_bool_triple_gen_var_1().test_properties(|(n, d, sign)| {
        let x = Rational::from_sign_and_naturals(sign, n.clone(), d.clone());
        assert!(x.is_valid());

        let x_alt = Rational::from_sign_and_naturals_ref(sign, &n, &d);
        assert!(x.is_valid());
        assert_eq!(x, x_alt);

        if n != 0 {
            assert_eq!(x >= 0, sign);
        }
        if (&n).coprime_with(&d) {
            assert_eq!(x.into_numerator_and_denominator(), (n, d));
        }
    });

    natural_pair_gen_var_5().test_properties(|(n, d)| {
        assert_eq!(
            Rational::from_naturals(n.clone(), d.clone()),
            Rational::from_sign_and_naturals(true, n, d)
        );
    });

    natural_gen().test_properties(|n| {
        assert_eq!(
            Rational::from_sign_and_naturals_ref(true, &n, &Natural::ONE),
            n
        );
    });
}

fn test_from_sign_and_unsigneds_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    let test = |sign, n: u8, d: u8, out| {
        let n = T::from(n);
        let d = T::from(d);
        let x = Rational::from_sign_and_unsigneds(sign, n, d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(false, 0, 1, "0");
    test(true, 0, 1, "0");
    test(false, 0, 5, "0");
    test(true, 0, 5, "0");
    test(false, 3, 6, "-1/2");
    test(true, 3, 6, "1/2");
    test(false, 100, 101, "-100/101");
    test(true, 100, 101, "100/101");
}

#[test]
fn test_from_sign_and_unsigneds() {
    apply_fn_to_unsigneds!(test_from_sign_and_unsigneds_helper);
}

fn from_sign_and_unsigneds_properties_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T> + PartialEq<T>,
    Rational: PartialEq<T>,
{
    unsigned_unsigned_bool_triple_gen_var_2::<T, T>().test_properties(|(n, d, sign)| {
        let x = Rational::from_sign_and_unsigneds(sign, n, d);
        assert!(x.is_valid());
        if n != T::ZERO {
            assert_eq!(PartialOrd::<u32>::ge(&x, &0u32), sign);
        }
        if n.coprime_with(d) {
            let (n_alt, d_alt) = x.into_numerator_and_denominator();
            assert_eq!(n_alt, n);
            assert_eq!(d_alt, d);
        }
    });

    unsigned_pair_gen_var_12::<T, T>().test_properties(|(n, d)| {
        assert!(PartialEq::<Rational>::eq(
            &Rational::from_unsigneds(n, d),
            &Rational::from_sign_and_unsigneds(true, n, d)
        ));
    });

    unsigned_gen::<T>()
        .test_properties(|n| assert_eq!(Rational::from_sign_and_unsigneds(true, n, T::ONE), n));
}

#[test]
fn from_sign_and_unsigneds_properties() {
    apply_fn_to_unsigneds!(from_sign_and_unsigneds_properties_helper);
}
