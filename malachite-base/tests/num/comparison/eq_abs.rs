// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::generators::{
    primitive_float_gen_var_11, primitive_float_pair_gen, primitive_float_triple_gen, signed_gen,
    signed_pair_gen, signed_triple_gen,
};

#[test]
pub fn test_eq_abs() {
    fn test<T: Copy + EqAbs>(x: T, y: T, eq: bool) {
        assert_eq!(x.eq_abs(&y), eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.ne_abs(&y), !eq);
        assert_eq!(y.ne_abs(&x), !eq);
    }
    test(123i64, 123i64, true);
    test(123i64, 456i64, false);

    test(123i64, -123i64, true);
    test(123i64, -456i64, false);

    test(-123i64, 123i64, true);
    test(-123i64, 456i64, false);

    test(-123i64, -123i64, true);
    test(-123i64, -456i64, false);

    test(123.0, 123.0, true);
    test(123.0, 456.0, false);

    test(123.0, -123.0, true);
    test(123.0, -456.0, false);

    test(-123.0, 123.0, true);
    test(-123.0, 456.0, false);

    test(-123.0, -123.0, true);
    test(-123.0, -456.0, false);

    test(123.0, f64::NAN, false);
    test(f64::NAN, f64::NAN, false);
    test(123.0, f64::INFINITY, false);
    test(123.0, f64::NEGATIVE_INFINITY, false);
    test(f64::INFINITY, f64::INFINITY, true);
    test(f64::INFINITY, f64::NEGATIVE_INFINITY, true);
    test(f64::NEGATIVE_INFINITY, f64::INFINITY, true);
    test(f64::NEGATIVE_INFINITY, f64::NEGATIVE_INFINITY, true);
    test(123.0, 0.0, false);
    test(123.0, -0.0, false);
    test(0.0, 0.0, true);
    test(0.0, -0.0, true);
    test(-0.0, 0.0, true);
    test(-0.0, -0.0, true);
}

fn properties_helper_signed<T: PrimitiveSigned>()
where
    <T as UnsignedAbs>::Output: Eq,
{
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x.unsigned_abs().eq(&y.unsigned_abs()), eq);
        if x != T::MIN {
            assert_eq!((-x).eq_abs(&y), eq);
        }
        if y != T::MIN {
            assert_eq!(x.eq_abs(&-y), eq);
        }
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.ne_abs(&y), !eq);
    });

    signed_gen::<T>().test_properties(|x| {
        assert!(x.eq_abs(&x));
        assert!(!x.ne_abs(&x));
    });

    signed_triple_gen::<T>().test_properties(|(x, y, z)| {
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });
}

fn properties_helper_primitive_float<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(x.abs().eq(&y.abs()), eq);
        assert_eq!((-x).eq_abs(&y), eq);
        assert_eq!(x.eq_abs(&-y), eq);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.ne_abs(&y), !eq);
    });

    primitive_float_gen_var_11::<T>().test_properties(|x| {
        assert!(x.eq_abs(&x));
        assert!(!x.ne_abs(&x));
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });
}

#[test]
fn eq_abs_properties() {
    apply_fn_to_signeds!(properties_helper_signed);
    apply_fn_to_primitive_floats!(properties_helper_primitive_float);
}
