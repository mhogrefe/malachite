// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{primitive_float_gen, primitive_float_gen_var_11};
use malachite_float::test_util::common::{
    parse_hex_string, ORDERED_F32S, ORDERED_F64S, ORDERED_FLOAT_HEX_STRINGS,
};
use malachite_float::test_util::generators::{
    float_float_primitive_float_triple_gen, float_gen, float_gen_var_2,
    float_primitive_float_pair_gen, float_primitive_float_primitive_float_triple_gen,
};
use malachite_float::Float;
use rug;
use std::cmp::Ordering::{self, *};

const fn encode(oo: Option<Ordering>) -> u8 {
    match oo {
        None => 9,
        Some(Less) => 0,
        Some(Equal) => 1,
        Some(Greater) => 2,
    }
}

#[rustfmt::skip]
const MATRIX_F32: [[u8; 17]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 1, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [2, 2, 2, 2, 2, 1, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], // NaN
    [2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 1, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#1
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#2
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#100
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 1, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 1],
];

#[rustfmt::skip]
const MATRIX_F64: [[u8; 17]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 1, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 1, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 1, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [2, 2, 2, 2, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [2, 2, 2, 2, 2, 1, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 1, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], // NaN
    [2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 1, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 1, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#1
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#2
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#100
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 1, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 1, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 1, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 1],
];

#[test]
fn test_partial_cmp_primitive_float() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX_F64.iter()) {
        let x = parse_hex_string(sx);
        for (&y, &e) in ORDERED_F64S.iter().zip(row.iter()) {
            assert_eq!(encode(x.partial_cmp(&y)), e);
            assert_eq!(encode(rug::Float::exact_from(&x).partial_cmp(&y)), e);
            assert_eq!(encode(y.partial_cmp(&x).map(Ordering::reverse)), e);
            assert_eq!(
                encode(
                    y.partial_cmp(&rug::Float::exact_from(&x))
                        .map(Ordering::reverse)
                ),
                e
            );
        }
    }
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX_F32.iter()) {
        let x = parse_hex_string(sx);
        for (&y, &e) in ORDERED_F32S.iter().zip(row.iter()) {
            assert_eq!(encode(x.partial_cmp(&y)), e);
            assert_eq!(encode(rug::Float::exact_from(&x).partial_cmp(&y)), e);
            assert_eq!(encode(y.partial_cmp(&x).map(Ordering::reverse)), e);
            assert_eq!(
                encode(
                    y.partial_cmp(&rug::Float::exact_from(&x))
                        .map(Ordering::reverse)
                ),
                e
            );
        }
    }
}

#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn partial_cmp_primitive_float_properties_helper<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveFloat,
>()
where
    Float: TryFrom<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    float_primitive_float_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp(&u);
        assert_eq!(rug::Float::exact_from(&n).partial_cmp(&u), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp(&n), cmp_rev);
        assert_eq!(u.partial_cmp(&rug::Float::exact_from(&n)), cmp_rev);

        assert_eq!(
            PartialOrd::<Float>::partial_cmp(&n, &Float::exact_from(u)),
            cmp
        );
    });

    float_float_primitive_float_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Less));
        } else if n > u && u > m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Greater));
        }
    });

    float_primitive_float_primitive_float_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    float_gen().test_properties(|x| {
        assert!(!(x < T::NAN));
        assert!(!(x > T::NAN));
    });

    float_gen_var_2().test_properties(|x| {
        assert!(x >= T::NEGATIVE_INFINITY);
        assert!(x <= T::INFINITY);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        assert!(!(x < Float::NAN));
        assert!(!(x > Float::NAN));
    });

    primitive_float_gen_var_11::<T>().test_properties(|x| {
        assert!(x >= Float::NEGATIVE_INFINITY);
        assert!(x <= Float::INFINITY);
    });
}

#[test]
fn partial_cmp_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_primitive_float_properties_helper);
}
