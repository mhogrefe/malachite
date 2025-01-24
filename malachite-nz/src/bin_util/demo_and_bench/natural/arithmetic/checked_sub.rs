// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_natural_max_bit_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_nrm};
use malachite_nz::test_util::natural::arithmetic::checked_sub::checked_sub;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_checked_sub);
    register_demo!(runner, demo_natural_checked_sub_val_ref);
    register_demo!(runner, demo_natural_checked_sub_ref_val);
    register_demo!(runner, demo_natural_checked_sub_ref_ref);

    register_bench!(runner, benchmark_natural_checked_sub_library_comparison);
    register_bench!(runner, benchmark_natural_checked_sub_evaluation_strategy);
}

fn demo_natural_checked_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.checked_sub({}) = {:?}", x_old, y_old, x.checked_sub(y));
    }
}

fn demo_natural_checked_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.checked_sub(&{}) = {:?}", x_old, y, x.checked_sub(&y));
    }
}

fn demo_natural_checked_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).checked_sub({}) = {:?}",
            x,
            y_old,
            (&x).checked_sub(y)
        );
    }
}

fn demo_natural_checked_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).checked_sub(&{}) = {:?}", x, y, (&x).checked_sub(&y));
    }
}

fn benchmark_natural_checked_sub_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.checked_sub(y))),
            ("num", &mut |((x, y), _, _)| no_out!(checked_sub(x, y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(checked_sub(x, y))),
        ],
    );
}

fn benchmark_natural_checked_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.checked_sub(Natural)", &mut |(x, y)| {
                no_out!(x.checked_sub(y))
            }),
            ("Natural.checked_sub(&Natural)", &mut |(x, y)| {
                no_out!(x.checked_sub(&y))
            }),
            ("&Natural.checked_sub(Natural)", &mut |(x, y)| {
                no_out!((&x).checked_sub(y))
            }),
            ("&Natural.checked_sub(&Natural)", &mut |(x, y)| {
                no_out!((&x).checked_sub(&y))
            }),
        ],
    );
}
