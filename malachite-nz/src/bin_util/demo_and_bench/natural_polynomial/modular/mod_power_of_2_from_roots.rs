// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModPowerOf2FromRoots;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_vec_unsigned_pair_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_from_roots::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_mod_power_of_2_from_roots);
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_from_roots_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_from_roots(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, pow) in natural_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "NaturalPolynomial::mod_power_of_2_from_roots(&{xs:?}, {pow}) = {}",
            NaturalPolynomial::mod_power_of_2_from_roots(&xs, pow)
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_from_roots_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial::mod_power_of_2_from_roots(&[Natural], u64)",
        BenchmarkType::Algorithms,
        natural_vec_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, pow)| {
                no_out!(NaturalPolynomial::mod_power_of_2_from_roots(&xs, pow));
            }),
            ("naive", &mut |(xs, pow)| {
                no_out!(mod_power_of_2_from_roots_naive(&xs, pow));
            }),
        ],
    );
}
