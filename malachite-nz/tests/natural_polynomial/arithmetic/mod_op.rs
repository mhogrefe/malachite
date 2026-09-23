// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModAssign, ModIsReduced, ModPowerOf2, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_pair_gen_var_1;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_natural_pair_gen_var_1,
    natural_polynomial_unsigned_pair_gen, natural_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let q = &p % &m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(&p % m.clone(), q);
        assert_eq!(p.clone() % &m, q);
        assert_eq!(p.clone() % m.clone(), q);
        let mut r = p.clone();
        r %= &m;
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p;
        r %= m;
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("0", "7", "0");
    // Every coefficient is taken modulo the divisor.
    test("x^2+4*x+5", "3", "x^2+x+2");
    test("x^2+4*x+5", "7", "x^2+4*x+5");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2+4*x+5", "1", "0");
    // Coefficients and divisor of many limbs.
    test(
        "1000000000001*x^2+2000000000003*x+5",
        "1000000000000",
        "x^2+3*x+5",
    );
    test(
        "1000000000000000000000000*x+1",
        "1234567890987",
        "530068894399*x+1",
    );
    // A divisor larger than every coefficient leaves the polynomial alone.
    test("x^2+4*x+5", "1000000000000000000000000", "x^2+4*x+5");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; a `NaturalPolynomial`'s
    // coefficients are never negative, so there is no case where the two could differ.
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_op(&m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_op(m.clone()), q);
        assert_eq!(p.clone().mod_op(&m), q);
        assert_eq!(p.clone().mod_op(m.clone()), q);
        let mut r = p.clone();
        r.mod_assign(&m);
        assert_eq!(r, q);
        let mut r = p;
        r.mod_assign(m);
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("x^2+4*x+5", "3", "x^2+x+2");
    test("x^2+4*x+5", "1", "0");
    test("4*x^2+3", "4", "3");
    test(
        "1000000000000000000000000*x+1",
        "1234567890987",
        "530068894399*x+1",
    );
}

#[test]
fn test_rem_lowers_the_degree() {
    // A leading coefficient that is a multiple of the divisor becomes zero, and a polynomial does
    // not hold trailing zero coefficients, so the degree falls.
    let test = |s, m, out| {
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            (NaturalPolynomial::from_str(s).unwrap() % m).to_string(),
            out
        );
    };
    test("4*x^2+3", "4", "3");
    test("6*x^2+3*x+2", "3", "2");
    // Every coefficient can vanish at once, leaving the zero polynomial.
    test("6*x^2+3*x+9", "3", "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3+6*x^2+2", "3", "x^3+2");
    // The degree can fall by more than one.
    test("5*x^5+10*x^4+15*x^3+1", "5", "1");
    test("2000000000000*x^2+1000000000000*x+7", "1000000000000", "7");
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = NaturalPolynomial::from_str("x").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_val_ref_fail() {
    let _ = NaturalPolynomial::from_str("x").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_val_fail() {
    let _ = &NaturalPolynomial::from_str("x").unwrap() % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &NaturalPolynomial::from_str("x").unwrap() % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    p %= Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    p %= &Natural::ZERO;
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = NaturalPolynomial::from_str("x")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x").unwrap();
    p.mod_assign(&Natural::ZERO);
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn rem_zero_polynomial_fail() {
    let _ = NaturalPolynomial::ZERO % Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_zero_polynomial_fail() {
    let _ = &NaturalPolynomial::ZERO % &Natural::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_zero_polynomial_fail() {
    let mut p = NaturalPolynomial::ZERO;
    p %= &Natural::ZERO;
}

#[test]
fn rem_properties() {
    natural_polynomial_natural_pair_gen_var_1().test_properties(|(p, m)| {
        let q = &p % &m;
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(&p % m.clone(), q);
        assert_eq!(p.clone() % &m, q);
        assert_eq!(p.clone() % m.clone(), q);
        let mut r = p.clone();
        r %= &m;
        assert_eq!(r, q);
        let mut r = p.clone();
        r %= m.clone();
        assert_eq!(r, q);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(&q % &m, q);

        // Reducing never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // A polynomial that is already reduced is left alone.
        assert_eq!(p.mod_is_reduced(&m), p == q);

        // `mod_op` is the same operation.
        assert_eq!((&p).mod_op(&m), q);
        assert_eq!(p.clone().mod_op(m.clone()), q);
        let mut r = p.clone();
        r.mod_assign(&m);
        assert_eq!(r, q);

        // This really is the remainder of a division: with the coefficient-wise quotient, `p` is
        // recovered exactly, coefficient by coefficient.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let i = u64::try_from(i).unwrap();
            assert_eq!(*c, &m * (c / &m) + q.coefficient(i));
            assert_eq!(*q.coefficient(i), c % &m);
        }
    });

    natural_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!(&p % Natural::ONE, NaturalPolynomial::ZERO);
        // A divisor above the height leaves the polynomial alone.
        let above = p
            .coefficients_asc()
            .iter()
            .max()
            .cloned()
            .unwrap_or_default()
            + Natural::ONE;
        assert_eq!(&p % &above, p);
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in [1, 7, 64, 100] {
            assert_eq!(&p % Natural::power_of_2(pow), (&p).mod_power_of_2(pow));
        }
    });

    unsigned_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, m)| {
        // Agrees with the `UnsignedPolynomial` operation on the polynomials both types can hold.
        let m = m + 1;
        assert_eq!(
            NaturalPolynomial::from(p.clone()) % Natural::from(m),
            NaturalPolynomial::from(p % m)
        );
    });
}

#[test]
fn test_rem_unsigned() {
    fn test<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(s: &str, m: T, out: &str)
    where
        Natural: From<T>,
    {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q: UnsignedPolynomial<T> = &p % m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone() % m, q);
        assert_eq!((&p).mod_op(m), q);
        assert_eq!(p.mod_op(m), q);
    }
    test("0", 1u8, "0");
    test("0", 7u64, "0");
    test("x^2+4*x+5", 3u32, "x^2+x+2");
    test("x^2+4*x+5", 1u16, "0");
    // Coefficients far larger than the modulus type are reduced into it.
    test("1000000000001*x^2+2000000000003*x+5", 1000u32, "x^2+3*x+5");
    test("1000000000001*x^2+2000000000003*x+5", 7u8, "2*x^2+5*x+5");
    test("256*x^2+257*x+3", 255u8, "x^2+2*x+3");
    test(
        "100000000000000000000*x+1",
        u64::MAX,
        "7766279631452241925*x+1",
    );
    test(
        "1000000000000000000000000000000000000000*x",
        u128::MAX,
        "319435266158123073073250785136463577090*x",
    );
    // Reducing the leading coefficient to zero lowers the degree.
    test("1024*x^2+3", 4usize, "3");
    test("4294967296*x^2+4294967296", 65536u32, "0");
}

#[test]
#[should_panic]
fn rem_unsigned_fail() {
    let _: UnsignedPolynomial<u8> = NaturalPolynomial::from_str("x").unwrap() % 0u8;
}

#[test]
#[should_panic]
fn rem_unsigned_ref_fail() {
    let _: UnsignedPolynomial<u64> = &NaturalPolynomial::from_str("x").unwrap() % 0u64;
}

#[test]
#[should_panic]
fn rem_unsigned_zero_polynomial_fail() {
    let _: UnsignedPolynomial<u32> = &NaturalPolynomial::ZERO % 0u32;
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedPolynomial<u16> = NaturalPolynomial::from_str("x").unwrap().mod_op(0u16);
}

// Comparing with a converted value is the reference the direct reduction is checked against.
#[allow(clippy::op_ref)]
fn rem_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T> + PartialEq<T>,
{
    natural_polynomial_unsigned_pair_gen::<T>().test_properties(|(p, m)| {
        if m == T::ZERO {
            return;
        }
        let q: UnsignedPolynomial<T> = &p % m;
        assert!(q.is_valid());
        assert_eq!(p.clone() % m, q);
        assert_eq!((&p).mod_op(m), q);
        assert_eq!(p.clone().mod_op(m), q);

        // The result is reduced, and reducing it again changes nothing.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(&q % m, q);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = &p % Natural::from(m);
        assert!(&r == &q);
        assert_eq!(UnsignedPolynomial::<T>::try_from(&r), Ok(q.clone()));
        assert_eq!(NaturalPolynomial::from(q), r);
    });

    natural_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(_, q)| {
        // On a polynomial that both types can hold, this is the `UnsignedPolynomial` operation.
        for m in [T::ONE, T::TWO, T::MAX] {
            assert_eq!(NaturalPolynomial::from(q.clone()) % m, &q % m);
        }
    });
}

#[test]
fn rem_unsigned_properties() {
    apply_fn_to_unsigneds!(rem_unsigned_properties_helper);
}
