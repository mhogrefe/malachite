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
use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    gaussian_integer_signed_pair_gen, gaussian_integer_unsigned_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_eq_abs_u32() {
    let test = |s, v: u32, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(x.eq_abs(&v), out);
        assert_eq!(v.eq_abs(&x), out);
    };
    test("0", 0, true);
    test("i", 1, true);
    test("-123", 123, true);
    test("3+4i", 5, true);
    test("3+4i", 4, false);
    test("5+12i", 13, true);
    test("2+2i", 3, false);
}

#[test]
fn test_eq_abs_i32() {
    let test = |s, v: i32, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(x.eq_abs(&v), out);
        assert_eq!(v.eq_abs(&x), out);
    };
    test("0", 0, true);
    test("i", -1, true);
    test("-123", 123, true);
    test("3+4i", -5, true);
    test("3+4i", 4, false);
    test("8+15i", -17, true);
    test("2+2i", -3, false);
}

fn eq_abs_primitive_int_properties_helper_unsigned<T: EqAbs<GaussianInteger> + PrimitiveUnsigned>()
where
    GaussianInteger: EqAbs<T> + From<T>,
    Integer: From<T> + EqAbs<T>,
{
    gaussian_integer_unsigned_pair_gen::<T>().test_properties(|(x, u)| {
        let eq = x.eq_abs(&u);
        assert_eq!(u.eq_abs(&x), eq);
        assert_eq!((&x).abs_squared() == Integer::from(u).abs_squared(), eq);
        assert_eq!((&x).conjugate().eq_abs(&u), eq);
        assert_eq!((-&x).eq_abs(&u), eq);
        assert_eq!(
            <GaussianInteger as EqAbs>::eq_abs(&x, &GaussianInteger::from(u)),
            eq
        );
    });
}

fn eq_abs_primitive_int_properties_helper_signed<T: EqAbs<GaussianInteger> + PrimitiveSigned>()
where
    GaussianInteger: EqAbs<T> + From<T>,
    Integer: From<T> + EqAbs<T>,
{
    gaussian_integer_signed_pair_gen::<T>().test_properties(|(x, i)| {
        let eq = x.eq_abs(&i);
        assert_eq!(i.eq_abs(&x), eq);
        assert_eq!((&x).abs_squared() == Integer::from(i).abs_squared(), eq);
        assert_eq!((&x).conjugate().eq_abs(&i), eq);
        assert_eq!((-&x).eq_abs(&i), eq);
        assert_eq!(
            <GaussianInteger as EqAbs>::eq_abs(&x, &GaussianInteger::from(i)),
            eq
        );
    });
}

#[test]
fn eq_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(eq_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(eq_abs_primitive_int_properties_helper_signed);
}
