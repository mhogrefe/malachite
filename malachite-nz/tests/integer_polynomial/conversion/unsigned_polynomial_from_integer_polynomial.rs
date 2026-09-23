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
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::conversion::unsigned_polynomial_from_integer_polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_unsigned_polynomial_from_integer_polynomial() {
    fn test<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>>(s: &str, out: Option<&str>) {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(q.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(q) = &q {
            assert!(q.is_valid());
        } else {
            assert_eq!(q, Err(UnsignedPolynomialFromIntegerPolynomialError));
        }
        assert_eq!(UnsignedPolynomial::<T>::try_from(p), q);
    }
    test::<u8>("0", Some("0"));
    test::<u8>("1", Some("1"));
    test::<u8>("x", Some("x"));
    test::<u8>("3*x^2+255", Some("3*x^2+255"));
    // Negative coefficients fail, and are never wrapped to the top of the range.
    test::<u8>("-1", None);
    test::<u64>("3*x^2-1", None);
    test::<u128>("-x", None);
    // One coefficient too large is enough, wherever it is.
    test::<u8>("3*x^2+256", None);
    test::<u8>("256*x^2+3", None);
    test::<u16>("3*x^2+256", Some("3*x^2+256"));
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
    test::<usize>("x^2+x", Some("x^2+x"));
}

fn unsigned_polynomial_from_integer_polynomial_properties_helper<
    T: PrimitiveUnsigned
        + for<'a> TryFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> TryFrom<&'a Natural>,
>()
where
    Integer: From<T> + PartialEq<T>,
    Natural: From<T>,
{
    integer_polynomial_gen().test_properties(|p| {
        let q = UnsignedPolynomial::<T>::try_from(&p);
        assert_eq!(UnsignedPolynomial::<T>::try_from(p.clone()), q);
        // The conversion succeeds exactly when every coefficient fits.
        assert_eq!(
            q.is_ok(),
            p.coefficients_asc().iter().all(|c| T::convertible_from(c))
        );
        // Going through a NaturalPolynomial gives the same answer.
        assert_eq!(
            NaturalPolynomial::try_from(&p)
                .ok()
                .and_then(|n| UnsignedPolynomial::<T>::try_from(n).ok()),
            q.clone().ok()
        );
        if let Ok(q) = q {
            assert!(q.is_valid());
            assert!(p == q);
            assert_eq!(q.degree(), p.degree());
            assert_eq!(IntegerPolynomial::from(q), p);
        }
    });

    integer_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(_, q)| {
        // Converting to an IntegerPolynomial and back is the identity.
        assert_eq!(
            UnsignedPolynomial::<T>::try_from(IntegerPolynomial::from(q.clone())),
            Ok(q)
        );
    });

    assert_eq!(
        UnsignedPolynomial::<T>::try_from(IntegerPolynomial::ZERO),
        Ok(UnsignedPolynomial::<T>::ZERO)
    );
    assert!(UnsignedPolynomial::<T>::try_from(IntegerPolynomial::negative_one()).is_err());
    assert!(
        UnsignedPolynomial::<T>::try_from(IntegerPolynomial::from(
            Integer::from(T::MAX) + Integer::ONE
        ))
        .is_err()
    );
}

#[test]
fn unsigned_polynomial_from_integer_polynomial_properties() {
    apply_fn_to_unsigneds!(unsigned_polynomial_from_integer_polynomial_properties_helper);
}
