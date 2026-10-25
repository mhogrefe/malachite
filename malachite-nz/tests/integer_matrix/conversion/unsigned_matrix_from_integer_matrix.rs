// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::test_util::generators::{unsigned_matrix_gen, unsigned_vec_gen};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::conversion::unsigned_matrix_from_integer_matrix::*;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_unsigned_matrix_from_integer_matrix() {
    fn test<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>>(s: &str, out: Option<&str>) {
        let m = IntegerMatrix::from_str(s).unwrap();
        let n = UnsignedMatrix::<T>::try_from(&m);
        assert_eq!(n.as_ref().ok().map(ToString::to_string).as_deref(), out);
        if let Ok(n) = &n {
            assert!(n.is_valid());
        } else {
            assert_eq!(n, Err(UnsignedMatrixFromIntegerMatrixError));
        }
        assert_eq!(UnsignedMatrix::<T>::try_from(m), n);
    }
    test::<u8>("[]_{0,0}", Some("[]_{0,0}"));
    test::<u8>("[]_{0,3}", Some("[]_{0,3}"));
    test::<u8>("[]_{3,0}", Some("[]_{3,0}"));
    test::<u8>("[0]", Some("[0]"));
    test::<u8>("[1, 2, 3; 4, 5, 6]", Some("[1, 2, 3; 4, 5, 6]"));
    test::<u8>("[3, 255]", Some("[3, 255]"));
    // One element too large is enough, wherever it is.
    test::<u8>("[3, 256]", None);
    test::<u8>("[256, 3]", None);
    test::<u8>("[1, 1; 1, 256]", None);
    test::<u8>("[1; 256; 1]", None);
    test::<u16>("[3, 256]", Some("[3, 256]"));
    test::<u16>("[65536]", None);
    test::<u32>("[4294967295; 0]", Some("[4294967295; 0]"));
    test::<u32>("[4294967296; 0]", None);
    test::<u64>(
        "[18446744073709551615, 1]",
        Some("[18446744073709551615, 1]"),
    );
    test::<u64>("[18446744073709551616, 1]", None);
    test::<u128>(
        "[340282366920938463463374607431768211455]",
        Some("[340282366920938463463374607431768211455]"),
    );
    test::<u128>("[340282366920938463463374607431768211456]", None);
    test::<usize>("[1, 2]", Some("[1, 2]"));
    // A negative element is never reinterpreted, whatever its size.
    test::<u8>("[-1]", None);
    test::<u64>("[3, 4; 5, -1]", None);
    test::<u128>("[-340282366920938463463374607431768211455]", None);
}

fn unsigned_matrix_from_integer_matrix_properties_helper<
    T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer> + for<'a> ConvertibleFrom<&'a Integer>,
>()
where
    Integer: From<T>,
{
    integer_matrix_gen().test_properties(|m| {
        let n = UnsignedMatrix::<T>::try_from(&m);
        assert_eq!(UnsignedMatrix::<T>::try_from(m.clone()), n);
        // The conversion succeeds exactly when every element fits.
        assert_eq!(
            n.is_ok(),
            m.elements_ref().iter().all(|x| T::convertible_from(x))
        );
        if let Ok(n) = n {
            assert!(n.is_valid());
            assert_eq!((n.height(), n.width()), (m.height(), m.width()));
            assert_eq!(n.to_string(), m.to_string());
            assert_eq!(IntegerMatrix::from(n), m);
        }
    });

    unsigned_vec_gen::<T>().test_properties(|xs| {
        // Converting to a `IntegerMatrix` and back is the identity.
        let m = UnsignedMatrix::from_owned_elements(1, u64::exact_from(xs.len()), xs);
        assert_eq!(
            UnsignedMatrix::<T>::try_from(IntegerMatrix::from(m.clone())),
            Ok(m)
        );
    });

    assert_eq!(
        UnsignedMatrix::<T>::try_from(&IntegerMatrix::from_owned_elements(
            1,
            1,
            vec![Integer::from(T::MAX) + Integer::ONE]
        )),
        Err(UnsignedMatrixFromIntegerMatrixError)
    );
    assert_eq!(
        UnsignedMatrix::<T>::try_from(&IntegerMatrix::from_owned_elements(
            1,
            1,
            vec![-Integer::ONE]
        )),
        Err(UnsignedMatrixFromIntegerMatrixError)
    );
}

#[test]
fn unsigned_matrix_from_integer_matrix_properties() {
    apply_fn_to_unsigneds!(unsigned_matrix_from_integer_matrix_properties_helper);

    unsigned_matrix_gen().test_properties(|m| {
        // Converting to a `IntegerMatrix` and back is the identity, for every shape.
        assert_eq!(
            UnsignedMatrix::<u64>::try_from(IntegerMatrix::from(m.clone())),
            Ok(m)
        );
    });
}

#[test]
fn test_unsigned_matrix_convertible_from_integer_matrix() {
    fn test<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>>(s: &str, out: bool) {
        let m = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(UnsignedMatrix::<T>::convertible_from(&m), out);
    }
    test::<u8>("[]_{0,0}", true);
    test::<u8>("[]_{4,0}", true);
    test::<u8>("[3, 255]", true);
    test::<u8>("[3; 256]", false);
    test::<u16>("[3; 256]", true);
    test::<u32>("[0]", true);
    test::<u64>("[18446744073709551616]", false);
    test::<u64>("[0, -1]", false);
}

fn unsigned_matrix_convertible_from_integer_matrix_properties_helper<
    T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer> + for<'a> TryFrom<&'a Integer>,
>() {
    integer_matrix_gen().test_properties(|m| {
        assert_eq!(
            UnsignedMatrix::<T>::convertible_from(&m),
            UnsignedMatrix::<T>::try_from(&m).is_ok()
        );
    });
}

#[test]
fn unsigned_matrix_convertible_from_integer_matrix_properties() {
    apply_fn_to_unsigneds!(unsigned_matrix_convertible_from_integer_matrix_properties_helper);
}
