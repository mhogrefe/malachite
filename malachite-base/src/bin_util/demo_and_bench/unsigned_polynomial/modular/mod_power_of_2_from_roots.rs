// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::ModPowerOf2FromRoots;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_10;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_from_roots::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_power_of_2_from_roots);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_from_roots_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_from_roots<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "UnsignedPolynomial::mod_power_of_2_from_roots(&{xs:?}, {pow}) = {}",
            UnsignedPolynomial::mod_power_of_2_from_roots(&xs, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_from_roots_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>::mod_power_of_2_from_roots(&[{}], u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, pow)| {
                no_out!(UnsignedPolynomial::mod_power_of_2_from_roots(&xs, pow));
            }),
            ("naive", &mut |(xs, pow)| {
                no_out!(mod_power_of_2_from_roots_naive(&xs, pow));
            }),
        ],
    );
}
