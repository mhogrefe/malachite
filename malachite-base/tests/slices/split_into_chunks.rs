// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::slice_set_zero;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_2;

#[test]
fn test_split_into_chunks() {
    let xs = &[0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks!(xs, 3, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5]);
    assert_eq!(xs_3, &[6, 7]);

    split_into_chunks!(xs, 3, [xs_1], xs_2);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5, 6, 7]);

    split_into_chunks!(xs, 1, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0]);
    assert_eq!(xs_2, &[1]);
    assert_eq!(xs_3, &[2, 3, 4, 5, 6, 7]);

    split_into_chunks!(xs, 0, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[]);
    assert_eq!(xs_2, &[]);
    assert_eq!(xs_3, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks!(xs, 5, [], xs_1);
    assert_eq!(xs_1, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
#[should_panic]
fn split_into_chunks_fail() {
    let xs = &[0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks!(xs, 5, [_xs_1, _xs_2], _xs_3);
}

#[test]
fn test_split_into_chunks_mut() {
    let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks_mut!(xs, 3, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5]);
    assert_eq!(xs_3, &[6, 7]);

    split_into_chunks_mut!(xs, 3, [xs_1], xs_2);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 1, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0]);
    assert_eq!(xs_2, &[1]);
    assert_eq!(xs_3, &[2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 0, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[]);
    assert_eq!(xs_2, &[]);
    assert_eq!(xs_3, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 5, [], xs_1);
    assert_eq!(xs_1, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 3, [_xs_1, xs_2], _xs_3);
    slice_set_zero(xs_2);
    assert_eq!(xs, &[0, 1, 2, 0, 0, 0, 6, 7]);
}

#[test]
#[should_panic]
fn split_into_chunks_mut_fail() {
    let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks_mut!(xs, 5, [_xs_1, _xs_2], _xs_3);
}

macro_rules! split_into_chunks_helper {
    ($xs: expr, $len: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {{
        split_into_chunks!($xs, $len, [$($xs_i),*], $xs_last);
        $(
            assert_eq!($xs_i.len(), $len);
        )*
        assert_eq!($xs_last.len(), $xs.len() - ($n - 1) * $len);
        let mut xs_alt = Vec::with_capacity($xs.len());
        $(
            xs_alt.extend($xs_i.iter().cloned());
        )*
        xs_alt.extend($xs_last.iter().cloned());
        assert_eq!(xs_alt, $xs);
   }}
}

#[test]
fn split_into_chunks_properties() {
    let mut config = GenConfig::new();
    config.insert("small_unsigned_mean_n", 8);
    config.insert("small_unsigned_mean_d", 1);
    config.insert("mean_stripe_n", u8::WIDTH << 1);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_unsigned_unsigned_triple_gen_var_2::<u8>().test_properties_with_config(
        &config,
        |(xs, len, n)| match n {
            0 => split_into_chunks_helper!(xs, len, 1, [], xs_1),
            1 => split_into_chunks_helper!(xs, len, 2, [xs_1], xs_2),
            2 => split_into_chunks_helper!(xs, len, 3, [xs_1, xs_2], xs_3),
            3 => {
                split_into_chunks_helper!(xs, len, 4, [xs_1, xs_2, xs_3], xs_4);
            }
            4 => {
                split_into_chunks_helper!(xs, len, 5, [xs_1, xs_2, xs_3, xs_4], xs_5);
            }
            5 => split_into_chunks_helper!(xs, len, 6, [xs_1, xs_2, xs_3, xs_4, xs_5], xs_6),
            6 => split_into_chunks_helper!(xs, len, 7, [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6], xs_7),
            7 => split_into_chunks_helper!(
                xs,
                len,
                8,
                [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6, xs_7],
                xs_8
            ),
            _ => {}
        },
    );
}

macro_rules! split_into_chunks_mut_helper {
    ($xs: expr, $len: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {{
        let xs_len = $xs.len();
        split_into_chunks_mut!($xs, $len, [$($xs_i),*], $xs_last);
        $(
            assert_eq!($xs_i.len(), $len);
        )*
        assert_eq!($xs_last.len(), xs_len - ($n - 1) * $len);
        let mut xs_alt = Vec::with_capacity(xs_len);
        $(
            xs_alt.extend($xs_i.iter().cloned());
        )*
        xs_alt.extend($xs_last.iter().cloned());
        assert_eq!(xs_alt, $xs);
   }}
}

#[test]
fn split_into_chunks_mut_properties() {
    let mut config = GenConfig::new();
    config.insert("small_unsigned_mean_n", 8);
    config.insert("small_unsigned_mean_d", 1);
    config.insert("mean_stripe_n", u8::WIDTH << 1);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_unsigned_unsigned_triple_gen_var_2::<u8>().test_properties_with_config(
        &config,
        |(mut xs, len, n)| match n {
            0 => split_into_chunks_mut_helper!(xs, len, 1, [], xs_1),
            1 => split_into_chunks_mut_helper!(xs, len, 2, [xs_1], xs_2),
            2 => split_into_chunks_mut_helper!(xs, len, 3, [xs_1, xs_2], xs_3),
            3 => {
                split_into_chunks_mut_helper!(xs, len, 4, [xs_1, xs_2, xs_3], xs_4);
            }
            4 => {
                split_into_chunks_mut_helper!(xs, len, 5, [xs_1, xs_2, xs_3, xs_4], xs_5);
            }
            5 => split_into_chunks_mut_helper!(xs, len, 6, [xs_1, xs_2, xs_3, xs_4, xs_5], xs_6),
            6 => {
                split_into_chunks_mut_helper!(
                    xs,
                    len,
                    7,
                    [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6],
                    xs_7
                );
            }
            7 => split_into_chunks_mut_helper!(
                xs,
                len,
                8,
                [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6, xs_7],
                xs_8
            ),
            _ => {}
        },
    );
}
