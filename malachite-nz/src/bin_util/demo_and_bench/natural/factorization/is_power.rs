// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_2_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_power);
    register_demo!(runner, demo_natural_express_as_power);

    register_bench!(runner, benchmark_natural_is_power_library_comparison);
    register_bench!(runner, benchmark_natural_express_as_power);
}

fn demo_natural_is_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_power() {
            println!("{n} is a perfect power");
        } else {
            println!("{n} is not a perfect power");
        }
    }
}

fn demo_natural_express_as_power(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("{}.express_as_power() = {:?}", n, n.express_as_power());
    }
}

fn benchmark_natural_is_power_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_power()",
        BenchmarkType::LibraryComparison,
        natural_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(n.is_power())),
            ("rug", &mut |(n, _)| no_out!(n.is_perfect_power())),
        ],
    );
}

fn benchmark_natural_express_as_power(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.express_as_power()",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.express_as_power()))],
    );
}
