// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_split_into_chunks);
    register_demo!(runner, demo_split_into_chunks_mut);
}

macro_rules! split_into_chunks_helper {
    ($xs: expr, $len: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {{
        split_into_chunks!($xs, $len, [$($xs_i),*], $xs_last);
        let xss = &[$($xs_i,)* $xs_last];
        let mut message = "xs := ".to_string();
        message.push_str(&$xs.to_debug_string());
        message.push_str("; split_into_chunks!(xs, ");
        message.push_str(&$len.to_string());
        message.push_str(", [");
        message.push_str(&(1..$n).map(|i| format!("xs_{}", i)).join(", "));
        message.push_str("], xs_");
        message.push_str(&$n.to_string());
        message.push_str("); ");
        message.push_str(&(1..=$n).zip(xss)
            .map(|(i, xs)| format!("xs_{} = {:?}", i, xs)).join("; "));
        println!("{}", message);
   }}
}

fn demo_split_into_chunks(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, len, n) in unsigned_vec_unsigned_unsigned_triple_gen_var_2::<u8>()
        .get(gm, config)
        .take(limit)
    {
        match n {
            0 => split_into_chunks_helper!(xs, len, 1, [], xs_1),
            1 => split_into_chunks_helper!(xs, len, 2, [xs_1], xs_2),
            2 => split_into_chunks_helper!(xs, len, 3, [xs_1, xs_2], xs_3),
            3 => split_into_chunks_helper!(xs, len, 4, [xs_1, xs_2, xs_3], xs_4),
            4 => split_into_chunks_helper!(xs, len, 5, [xs_1, xs_2, xs_3, xs_4], xs_5),
            5 => split_into_chunks_helper!(xs, len, 6, [xs_1, xs_2, xs_3, xs_4, xs_5], xs_6),
            6 => split_into_chunks_helper!(xs, len, 7, [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6], xs_7),
            7 => split_into_chunks_helper!(
                xs,
                len,
                8,
                [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6, xs_7],
                xs_8
            ),
            _ => println!("Large number of chunks"),
        }
    }
}

macro_rules! split_into_chunks_mut_helper {
    ($xs: expr, $len: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {{
        split_into_chunks_mut!($xs, $len, [$($xs_i),*], $xs_last);
        let xss = &[$($xs_i.to_vec(),)* $xs_last.to_vec()];
        let mut message = "xs := ".to_string();
        message.push_str(&$xs.to_debug_string());
        message.push_str("; split_into_chunks_mut!(xs, ");
        message.push_str(&$len.to_string());
        message.push_str(", [");
        message.push_str(&(1..$n).map(|i| format!("xs_{}", i)).join(", "));
        message.push_str("], xs_");
        message.push_str(&$n.to_string());
        message.push_str("); ");
        message.push_str(&(1..=$n).zip(xss)
            .map(|(i, xs)| format!("xs_{} = {:?}", i, xs)).join("; "));
        println!("{}", message);
   }}
}

fn demo_split_into_chunks_mut(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, len, n) in unsigned_vec_unsigned_unsigned_triple_gen_var_2::<u8>()
        .get(gm, config)
        .take(limit)
    {
        match n {
            0 => split_into_chunks_mut_helper!(xs, len, 1, [], xs_1),
            1 => split_into_chunks_mut_helper!(xs, len, 2, [xs_1], xs_2),
            2 => split_into_chunks_mut_helper!(xs, len, 3, [xs_1, xs_2], xs_3),
            3 => split_into_chunks_mut_helper!(xs, len, 4, [xs_1, xs_2, xs_3], xs_4),
            4 => split_into_chunks_mut_helper!(xs, len, 5, [xs_1, xs_2, xs_3, xs_4], xs_5),
            5 => split_into_chunks_mut_helper!(xs, len, 6, [xs_1, xs_2, xs_3, xs_4, xs_5], xs_6),
            6 => split_into_chunks_mut_helper!(
                xs,
                len,
                7,
                [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6],
                xs_7
            ),
            7 => split_into_chunks_mut_helper!(
                xs,
                len,
                8,
                [xs_1, xs_2, xs_3, xs_4, xs_5, xs_6, xs_7],
                xs_8
            ),
            _ => println!("Large number of chunks"),
        }
    }
}
