// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::test_util::common::{
    parse_hex_string, ORDERED_F32S, ORDERED_F64S, ORDERED_FLOAT_HEX_STRINGS,
};
use malachite_float::test_util::generators::{
    float_primitive_float_pair_gen, float_primitive_float_pair_gen_var_1,
};
use malachite_float::Float;
use rug;
use std::cmp::Ordering::*;

#[rustfmt::skip]
const MATRIX_F32: [[u8; 17]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

#[rustfmt::skip]
const MATRIX_F64: [[u8; 17]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

#[test]
fn test_partial_eq_primitive_float() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX_F64.iter()) {
        let x = parse_hex_string(sx);
        for (&y, &e) in ORDERED_F64S.iter().zip(row.iter()) {
            assert_eq!(u8::from(x == y), e);
            assert_eq!(u8::from(rug::Float::exact_from(&x) == y), e);
            assert_eq!(u8::from(y == x), e);
            assert_eq!(u8::from(y == rug::Float::exact_from(&x)), e);
        }
    }
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX_F32.iter()) {
        let x = parse_hex_string(sx);
        for (&y, &e) in ORDERED_F32S.iter().zip(row.iter()) {
            assert_eq!(u8::from(x == y), e);
            assert_eq!(u8::from(rug::Float::exact_from(&x) == y), e);
            assert_eq!(u8::from(y == x), e);
            assert_eq!(u8::from(y == rug::Float::exact_from(&x)), e);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn partial_eq_primitive_float_properties_helper_helper<
    T: PartialEq<Float> + PartialEq<rug::Float> + PrimitiveFloat,
>(
    n: Float,
    f: T,
) where
    Float: TryFrom<T> + PartialEq<T> + PartialOrd<T>,
    rug::Float: PartialEq<T>,
{
    let eq = n == f;
    assert_eq!(f == n, eq);
    let rn = rug::Float::exact_from(&n);
    assert_eq!(rn == f, eq);
    assert_eq!(f == rn, eq);
    assert_eq!(n.partial_cmp(&f) == Some(Equal), eq);
    if f.is_finite() {
        assert_eq!(PartialEq::<Float>::eq(&n, &Float::exact_from(f)), eq);
    }
}

fn partial_eq_primitive_float_properties_helper<
    T: PartialEq<Float> + PartialEq<rug::Float> + PrimitiveFloat,
>()
where
    Float: TryFrom<T> + PartialEq<T> + PartialOrd<T>,
    rug::Float: PartialEq<T>,
{
    float_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        partial_eq_primitive_float_properties_helper_helper(n, f);
    });

    float_primitive_float_pair_gen_var_1::<T>().test_properties(|(n, f)| {
        partial_eq_primitive_float_properties_helper_helper(n, f);
    });
}

#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
