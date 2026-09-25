// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::conversion::unsigned_polynomial_from_natural_polynomial::*;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_unsigned_polynomial_from_natural_polynomial() {
    fn test<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>>(s: &str, out: Option<&str>) {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(q.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(q) = &q {
            assert!(q.is_valid());
        } else {
            assert_eq!(q, Err(UnsignedPolynomialFromNaturalPolynomialError));
        }
        assert_eq!(UnsignedPolynomial::<T>::try_from(p), q);
    }
    test::<u8>("0", Some("0"));
    test::<u8>("1", Some("1"));
    test::<u8>("x", Some("x"));
    test::<u8>("3*x^2+255", Some("3*x^2+255"));
    // One coefficient too large is enough, wherever it is.
    test::<u8>("3*x^2+256", None);
    test::<u8>("256*x^2+3", None);
    test::<u8>("x^3+256*x+1", None);
    test::<u16>("3*x^2+256", Some("3*x^2+256"));
    test::<u16>("65536", None);
    test::<u32>("4294967295*x", Some("4294967295*x"));
    test::<u32>("4294967296*x", None);
    test::<u64>(
        "18446744073709551615*x^2+1",
        Some("18446744073709551615*x^2+1"),
    );
    test::<u64>("18446744073709551616*x^2+1", None);
    test::<u128>(
        "340282366920938463463374607431768211455",
        Some("340282366920938463463374607431768211455"),
    );
    test::<u128>("340282366920938463463374607431768211456", None);
    test::<usize>("x^2+x", Some("x^2+x"));
}

fn unsigned_polynomial_from_natural_polynomial_properties_helper<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural> + for<'a> ConvertibleFrom<&'a Natural>,
>()
where
    Natural: From<T> + PartialEq<T>,
{
    natural_polynomial_gen().test_properties(|p| {
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(UnsignedPolynomial::<T>::try_from(p.clone()), q);
        // The conversion succeeds exactly when every coefficient fits.
        assert_eq!(
            q.is_ok(),
            p.coefficients_asc().iter().all(|c| T::convertible_from(c))
        );
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert!(p == q);
            assert_eq!(q.degree(), p.degree());
            assert_eq!(NaturalPolynomial::from(q), p);
        }
    });

    natural_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(_, q)| {
        // Converting to a NaturalPolynomial and back is the identity.
        assert_eq!(
            UnsignedPolynomial::<T>::try_from(NaturalPolynomial::from(q.clone())),
            Ok(q)
        );
    });

    assert_eq!(
        UnsignedPolynomial::<T>::try_from(NaturalPolynomial::ZERO),
        Ok(UnsignedPolynomial::<T>::ZERO)
    );
    assert_eq!(
        UnsignedPolynomial::<T>::try_from(NaturalPolynomial::from(
            Natural::from(T::MAX) + Natural::ONE
        )),
        Err(UnsignedPolynomialFromNaturalPolynomialError)
    );
}

#[test]
fn unsigned_polynomial_from_natural_polynomial_properties() {
    apply_fn_to_unsigneds!(unsigned_polynomial_from_natural_polynomial_properties_helper);
}

#[test]
fn test_unsigned_polynomial_convertible_from_natural_polynomial() {
    fn test<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural>>(s: &str, out: bool) {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert_eq!(UnsignedPolynomial::<T>::convertible_from(&p), out);
    }
    test::<u8>("3*x^2+255", true);
    test::<u8>("3*x^2+256", false);
    test::<u16>("3*x^2+256", true);
    test::<u32>("0", true);
}

fn unsigned_polynomial_convertible_from_natural_polynomial_properties_helper<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>() {
    natural_polynomial_gen().test_properties(|p| {
        assert_eq!(
            UnsignedPolynomial::<T>::convertible_from(&p),
            UnsignedPolynomial::<T>::try_from(&p).is_ok()
        );
    });
}

#[test]
fn unsigned_polynomial_convertible_from_natural_polynomial_properties() {
    apply_fn_to_unsigneds!(
        unsigned_polynomial_convertible_from_natural_polynomial_properties_helper
    );
}
