// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::{EqAbs, OrdAbs, PartialOrdAbs};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_signed_pair_gen, gaussian_rational_unsigned_pair_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_u32() {
    let test = |s, v: u32, cmp: Option<Ordering>| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.lt_abs(&v), cmp == Some(Less));
        assert_eq!(x.gt_abs(&v), cmp == Some(Greater));
        assert_eq!(x.eq_abs(&v), cmp == Some(Equal));
    };
    test("0", 0, Some(Equal));
    test("0", 5, Some(Less));
    test("i", 1, Some(Equal));
    test("-123", 123, Some(Equal));
    test("3+4i", 5, Some(Equal));
    test("3+4i", 4, Some(Greater));
    test("3+4i", 6, Some(Less));
    test("2+2i", 3, Some(Less));
    test("3/5+4i/5", 1, Some(Equal));
    test("3/5+4i/5", 0, Some(Greater));
    test("3/5+4i/5", 2, Some(Less));
    test("22/7", 3, Some(Greater));
}

#[test]
fn test_partial_cmp_abs_i32() {
    let test = |s, v: i32, cmp: Option<Ordering>| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.lt_abs(&v), cmp == Some(Less));
        assert_eq!(x.gt_abs(&v), cmp == Some(Greater));
        assert_eq!(x.eq_abs(&v), cmp == Some(Equal));
    };
    test("0", 0, Some(Equal));
    test("i", -1, Some(Equal));
    test("3+4i", -5, Some(Equal));
    test("3+4i", -4, Some(Greater));
    test("3+4i", -6, Some(Less));
    test("8+15i", -17, Some(Equal));
    test("2+2i", -3, Some(Less));
    test("3/5+4i/5", -1, Some(Equal));
    test("-22/7", -3, Some(Greater));
}

fn partial_cmp_abs_primitive_int_properties_helper_unsigned<
    T: PartialOrdAbs<GaussianRational> + PrimitiveUnsigned,
>()
where
    GaussianRational: EqAbs<T> + From<T> + PartialOrdAbs<T>,
    Rational: From<T>,
{
    gaussian_rational_unsigned_pair_gen::<T>().test_properties(|(x, u)| {
        let cmp = x.partial_cmp_abs(&u);
        assert_eq!(u.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.eq_abs(&u), cmp == Some(Equal));
        assert_eq!(
            (&x).abs_squared()
                .partial_cmp(&Rational::from(u).abs_squared()),
            cmp
        );
        assert_eq!((&x).conjugate().partial_cmp_abs(&u), cmp);
        assert_eq!((-&x).partial_cmp_abs(&u), cmp);
        assert_eq!(Some(x.cmp_abs(&GaussianRational::from(u))), cmp);
    });
}

fn partial_cmp_abs_primitive_int_properties_helper_signed<
    T: PartialOrdAbs<GaussianRational> + PrimitiveSigned,
>()
where
    GaussianRational: EqAbs<T> + From<T> + PartialOrdAbs<T>,
    Rational: From<T>,
{
    gaussian_rational_signed_pair_gen::<T>().test_properties(|(x, i)| {
        let cmp = x.partial_cmp_abs(&i);
        assert_eq!(i.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.eq_abs(&i), cmp == Some(Equal));
        assert_eq!(
            (&x).abs_squared()
                .partial_cmp(&Rational::from(i).abs_squared()),
            cmp
        );
        assert_eq!((&x).conjugate().partial_cmp_abs(&i), cmp);
        assert_eq!((-&x).partial_cmp_abs(&i), cmp);
        assert_eq!(Some(x.cmp_abs(&GaussianRational::from(i))), cmp);
    });
}

#[test]
fn partial_cmp_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_abs_primitive_int_properties_helper_signed);
}
