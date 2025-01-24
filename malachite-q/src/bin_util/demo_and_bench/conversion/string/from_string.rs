// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::string_gen;
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::generators::string_gen_var_12;
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_str);
    register_demo!(runner, demo_rational_from_str_targeted);
    register_bench!(runner, benchmark_rational_from_str_library_comparison);
}

fn demo_rational_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!("Rational::from_str({}) = {:?}", s, Rational::from_str(&s));
    }
}

fn demo_rational_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_12().get(gm, config).take(limit) {
        println!(
            "Rational::from_str({}) = {}",
            s,
            Rational::from_str(&s).unwrap()
        );
    }
}

fn benchmark_rational_from_str_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_str(&str)",
        BenchmarkType::LibraryComparison,
        string_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [
            ("Malachite", &mut |s| {
                no_out!(Rational::from_str(&s).unwrap())
            }),
            ("num", &mut |s| no_out!(BigRational::from_str(&s).unwrap())),
            (
                "rug",
                &mut |s| no_out!(rug::Rational::from_str(&s).unwrap()),
            ),
        ],
    );
}
