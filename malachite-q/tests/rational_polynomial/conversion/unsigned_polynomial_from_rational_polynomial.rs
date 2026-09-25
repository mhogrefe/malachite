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
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_polynomial::conversion::unsigned_polynomial_from_rational_polynomial::*;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_unsigned_polynomial_from_rational_polynomial() {
    fn test<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>>(s: &str, out: Option<&str>) {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(q.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(q) = &q {
            assert!(q.is_valid());
        } else {
            assert_eq!(q, Err(UnsignedPolynomialFromRationalPolynomialError));
        }
        assert_eq!(UnsignedPolynomial::<T>::try_from(p), q);
    }
    test::<u8>("0", Some("0"));
    test::<u8>("4/2", Some("2"));
    test::<u8>("3*x^2+255", Some("3*x^2+255"));
    // Negative, fractional or too-large coefficients fail; nothing is wrapped or rounded.
    test::<u8>("-1", None);
    test::<u8>("1/2", None);
    test::<u8>("3*x^2+256", None);
    test::<u16>("3*x^2+256", Some("3*x^2+256"));
    test::<u64>("3*x^2-1", None);
    test::<u64>("3*x^2+1/2", None);
    test::<u64>("18446744073709551615*x", Some("18446744073709551615*x"));
    test::<u64>("18446744073709551616*x", None);
    test::<u128>(
        "340282366920938463463374607431768211455",
        Some("340282366920938463463374607431768211455"),
    );
    test::<usize>("x^2+x", Some("x^2+x"));
}

fn unsigned_polynomial_from_rational_polynomial_properties_helper<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>,
>()
where
    Integer: From<T>,
{
    rational_polynomial_gen().test_properties(|p| {
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(UnsignedPolynomial::<T>::try_from(p.clone()), q);
        // Converting through an IntegerPolynomial gives the same answer.
        assert_eq!(
            IntegerPolynomial::try_from(&p)
                .ok()
                .and_then(|i| UnsignedPolynomial::<T>::try_from(i).ok()),
            q.clone().ok()
        );
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert_eq!(q.degree(), p.degree());
            assert_eq!(RationalPolynomial::from(q), p);
        }
    });

    rational_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(_, q)| {
        // Converting to a RationalPolynomial and back is the identity.
        assert_eq!(
            UnsignedPolynomial::<T>::try_from(RationalPolynomial::from(q.clone())),
            Ok(q)
        );
    });

    assert_eq!(
        UnsignedPolynomial::<T>::try_from(RationalPolynomial::ZERO),
        Ok(UnsignedPolynomial::<T>::ZERO)
    );
    assert!(UnsignedPolynomial::<T>::try_from(RationalPolynomial::negative_one()).is_err());
    assert!(UnsignedPolynomial::<T>::try_from(RationalPolynomial::one_half()).is_err());
}

#[test]
fn unsigned_polynomial_from_rational_polynomial_properties() {
    apply_fn_to_unsigneds!(unsigned_polynomial_from_rational_polynomial_properties_helper);
}

#[test]
fn test_unsigned_polynomial_convertible_from_rational_polynomial() {
    fn test<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>>(s: &str, out: bool) {
        let p = RationalPolynomial::from_str(s).unwrap();
        assert_eq!(UnsignedPolynomial::<T>::convertible_from(&p), out);
    }
    test::<u8>("3*x^2+255", true);
    test::<u8>("3*x^2+256", false);
    test::<u16>("3*x^2+256", true);
    test::<u64>("3*x^2-1", false);
    test::<u64>("3*x^2+1/2", false);
    test::<u32>("0", true);
}

fn unsigned_polynomial_convertible_from_rational_polynomial_properties_helper<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer> + for<'a> TryFrom<&'a Integer>,
>() {
    rational_polynomial_gen().test_properties(|p| {
        assert_eq!(
            UnsignedPolynomial::<T>::convertible_from(&p),
            UnsignedPolynomial::<T>::try_from(&p).is_ok()
        );
    });
}

#[test]
fn unsigned_polynomial_convertible_from_rational_polynomial_properties() {
    apply_fn_to_unsigneds!(
        unsigned_polynomial_convertible_from_rational_polynomial_properties_helper
    );
}
