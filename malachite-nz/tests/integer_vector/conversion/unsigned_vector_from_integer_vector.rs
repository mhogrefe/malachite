// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::integer_vector::conversion::unsigned_vector_from_integer_vector::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_unsigned_vector_from_integer_vector() {
    fn test<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>>(s: &str, out: Option<&str>) {
        let v = IntegerVector::from_str(s).unwrap();
        let w = UnsignedVector::<T>::try_from(&v);
        assert_eq!(w.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if w.is_err() {
            assert_eq!(w, Err(UnsignedVectorFromIntegerVectorError));
        }
        assert_eq!(UnsignedVector::<T>::try_from(v), w);
    }
    test::<u8>("()", Some("()"));
    test::<u8>("(0)", Some("(0)"));
    test::<u8>("(1, 2, 3)", Some("(1, 2, 3)"));
    test::<u8>("(3, 255)", Some("(3, 255)"));
    // One element out of range is enough, wherever it is, whether too large or negative.
    test::<u8>("(3, 256)", None);
    test::<u8>("(256, 3)", None);
    test::<u8>("(1, -1, 1)", None);
    test::<u8>("(-1)", None);
    test::<u16>("(3, 256)", Some("(3, 256)"));
    test::<u16>("(65536)", None);
    test::<u32>("(4294967295, 0)", Some("(4294967295, 0)"));
    test::<u32>("(4294967296, 0)", None);
    test::<u64>(
        "(18446744073709551615, 1)",
        Some("(18446744073709551615, 1)"),
    );
    test::<u64>("(18446744073709551616, 1)", None);
    test::<u64>("(-18446744073709551615, 1)", None);
    test::<u128>(
        "(340282366920938463463374607431768211455)",
        Some("(340282366920938463463374607431768211455)"),
    );
    test::<u128>("(340282366920938463463374607431768211456)", None);
    test::<usize>("(1, 2)", Some("(1, 2)"));
}

fn unsigned_vector_from_integer_vector_properties_helper<
    T: PrimitiveUnsigned
        + for<'a> TryFrom<&'a Integer>
        + for<'a> TryFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Integer>,
>()
where
    Integer: From<T>,
{
    integer_vector_gen().test_properties(|v| {
        let w = UnsignedVector::<T>::try_from(&v);
        assert_eq!(UnsignedVector::<T>::try_from(v.clone()), w);
        // The conversion succeeds exactly when every element fits.
        assert_eq!(w.is_ok(), v.elements.iter().all(|x| T::convertible_from(x)));
        // It agrees with going through a `NaturalVector`.
        assert_eq!(
            w.as_ref().ok(),
            NaturalVector::try_from(&v)
                .ok()
                .and_then(|n| UnsignedVector::<T>::try_from(n).ok())
                .as_ref()
        );
        if let Ok(w) = w {
            assert_eq!(w.dimension(), v.dimension());
            assert_eq!(w.to_string(), v.to_string());
            assert_eq!(IntegerVector::from(w), v);
        }
    });

    unsigned_vec_gen::<T>().test_properties(|xs| {
        // Converting to an `IntegerVector` and back is the identity.
        let w = UnsignedVector { elements: xs };
        assert_eq!(
            UnsignedVector::<T>::try_from(IntegerVector::from(w.clone())),
            Ok(w)
        );
    });

    assert_eq!(
        UnsignedVector::<T>::try_from(IntegerVector {
            elements: vec![Integer::from(T::MAX) + Integer::ONE]
        }),
        Err(UnsignedVectorFromIntegerVectorError)
    );
    assert_eq!(
        UnsignedVector::<T>::try_from(IntegerVector {
            elements: vec![-Integer::ONE]
        }),
        Err(UnsignedVectorFromIntegerVectorError)
    );
}

#[test]
fn unsigned_vector_from_integer_vector_properties() {
    apply_fn_to_unsigneds!(unsigned_vector_from_integer_vector_properties_helper);
}

#[test]
fn test_unsigned_vector_convertible_from_integer_vector() {
    fn test<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>>(s: &str, out: bool) {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!(UnsignedVector::<T>::convertible_from(&v), out);
    }
    test::<u8>("()", true);
    test::<u8>("(3, 255)", true);
    test::<u8>("(3, 256)", false);
    test::<u8>("(3, -1)", false);
    test::<u16>("(3, 256)", true);
    test::<u32>("(0)", true);
}

fn unsigned_vector_convertible_from_integer_vector_properties_helper<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer> + for<'a> TryFrom<&'a Integer>,
>() {
    integer_vector_gen().test_properties(|v| {
        assert_eq!(
            UnsignedVector::<T>::convertible_from(&v),
            UnsignedVector::<T>::try_from(&v).is_ok()
        );
    });
}

#[test]
fn unsigned_vector_convertible_from_integer_vector_properties() {
    apply_fn_to_unsigneds!(unsigned_vector_convertible_from_integer_vector_properties_helper);
}
