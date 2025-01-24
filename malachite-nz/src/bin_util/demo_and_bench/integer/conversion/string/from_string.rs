// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{FromStringBase, WrappingFrom};
use malachite_base::test_util::bench::bucketers::{
    pair_2_string_len_bucketer, string_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    string_gen, string_gen_var_4, unsigned_string_pair_gen_var_2, unsigned_string_pair_gen_var_3,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use num::{BigInt, Num};
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_from_str);
    register_demo!(runner, demo_integer_from_str_targeted);
    register_demo!(runner, demo_integer_from_string_base);
    register_demo!(runner, demo_integer_from_string_base_targeted);
    register_bench!(runner, benchmark_integer_from_str_library_comparison);
    register_bench!(
        runner,
        benchmark_integer_from_string_base_library_comparison
    );
}

fn demo_integer_from_string_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (base, s) in unsigned_string_pair_gen_var_2().get(gm, config).take(limit) {
        println!(
            "Integer::from_string_base({}, {}) = {:?}",
            base,
            s,
            Integer::from_string_base(base, &s)
        );
    }
}

fn demo_integer_from_string_base_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (base, s) in unsigned_string_pair_gen_var_3().get(gm, config).take(limit) {
        println!(
            "Integer::from_string_base({}, {}) = {}",
            base,
            s,
            Integer::from_string_base(base, &s).unwrap()
        );
    }
}

fn demo_integer_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!("Integer::from_str({}) = {:?}", s, Integer::from_str(&s));
    }
}

fn demo_integer_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_4().get(gm, config).take(limit) {
        println!(
            "Integer::from_str({}) = {}",
            s,
            Integer::from_str(&s).unwrap()
        );
    }
}

fn benchmark_integer_from_str_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_str(&str)",
        BenchmarkType::LibraryComparison,
        string_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [
            (
                "Malachite",
                &mut |s| no_out!(Integer::from_str(&s).unwrap()),
            ),
            ("num", &mut |s| no_out!(BigInt::from_str(&s).unwrap())),
            ("rug", &mut |s| no_out!(rug::Integer::from_str(&s).unwrap())),
        ],
    );
}

fn benchmark_integer_from_string_base_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_string_base(u64, &str)",
        BenchmarkType::LibraryComparison,
        unsigned_string_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_string_len_bucketer("s"),
        &mut [
            ("Malachite", &mut |(base, s)| {
                no_out!(Integer::from_string_base(base, &s).unwrap())
            }),
            ("num", &mut |(base, s)| {
                no_out!(BigInt::from_str_radix(&s, u32::wrapping_from(base)).unwrap())
            }),
            ("rug", &mut |(base, s)| {
                no_out!(rug::Integer::from_str_radix(&s, i32::wrapping_from(base)).unwrap())
            }),
        ],
    );
}
