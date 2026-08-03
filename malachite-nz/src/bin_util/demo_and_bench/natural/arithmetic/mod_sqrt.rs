// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModSqrt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_8;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_sqrt);
    register_demo!(runner, demo_natural_mod_sqrt_ref_ref);
    register_bench!(runner, benchmark_natural_mod_sqrt);
}

fn demo_natural_mod_sqrt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{x_old}.mod_sqrt({m_old}) = {:?}", x.mod_sqrt(m));
    }
}

fn demo_natural_mod_sqrt_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in natural_pair_gen_var_8().get(gm, config).take(limit) {
        println!("(&{x}).mod_sqrt(&{m}) = {:?}", (&x).mod_sqrt(&m));
    }
}

fn benchmark_natural_mod_sqrt(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.mod_sqrt(Natural)",
        BenchmarkType::Single,
        natural_pair_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [("Malachite", &mut |(x, m)| {
            no_out!(x.mod_sqrt(m));
        })],
    );
}
