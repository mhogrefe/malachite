// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ExtendedGcd;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_integer_max_bit_bucketer, triple_3_pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen, integer_pair_gen_nrm};
use num::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_extended_gcd);
    register_demo!(runner, demo_integer_extended_gcd_val_ref);
    register_demo!(runner, demo_integer_extended_gcd_ref_val);
    register_demo!(runner, demo_integer_extended_gcd_ref_ref);

    register_bench!(runner, benchmark_integer_extended_gcd_library_comparison);
    register_bench!(runner, benchmark_integer_extended_gcd_evaluation_strategy);
}

fn demo_integer_extended_gcd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).extended_gcd({}) = {:?}",
            x_old,
            y_old,
            x.extended_gcd(y)
        );
    }
}

fn demo_integer_extended_gcd_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).extended_gcd(&{}) = {:?}",
            x_old,
            y,
            x.extended_gcd(&y)
        );
    }
}

fn demo_integer_extended_gcd_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).extended_gcd({}) = {:?}",
            x,
            y_old,
            (&x).extended_gcd(y)
        );
    }
}

fn demo_integer_extended_gcd_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).extended_gcd(&{}) = {:?}",
            x,
            y,
            (&x).extended_gcd(&y)
        );
    }
}

fn benchmark_integer_extended_gcd_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.extended_gcd(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Malachite",
                &mut |(_, _, (x, y))| no_out!(x.extended_gcd(y)),
            ),
            ("num", &mut |((x, y), _, _)| no_out!(x.extended_gcd(&y))),
            ("rug", &mut |(_, (x, y), _)| {
                no_out!(x.extended_gcd(y, rug::Integer::new()))
            }),
        ],
    );
}

fn benchmark_integer_extended_gcd_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.extended_gcd(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer.extended_gcd(Integer)", &mut |(x, y)| {
                no_out!(x.extended_gcd(y))
            }),
            ("Integer.extended_gcd(&Integer)", &mut |(x, y)| {
                no_out!(x.extended_gcd(&y))
            }),
            ("&Integer.extended_gcd(Integer)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(y))
            }),
            ("&Integer.extended_gcd(&Integer)", &mut |(x, y)| {
                no_out!((&x).extended_gcd(&y))
            }),
        ],
    );
}
