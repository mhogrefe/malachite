// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen_var_1;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::triple_3_natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_significant_bits);
    register_demo!(runner, demo_natural_significant_bits);

    register_bench!(runner, benchmark_limbs_significant_bits);
    register_bench!(
        runner,
        benchmark_natural_significant_bits_library_comparison
    );
}

fn demo_limbs_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen_var_1::<Limb>().get(gm, config).take(limit) {
        println!(
            "limbs_significant_bits({:?}) = {}",
            xs,
            limbs_significant_bits(&xs)
        );
    }
}

fn demo_natural_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn benchmark_limbs_significant_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_significant_bits(&[Limb])",
        BenchmarkType::Single,
        unsigned_vec_gen_var_1::<Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(limbs_significant_bits(&xs)))],
    );
}

fn benchmark_natural_significant_bits_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.significant_bits()",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.significant_bits())),
            ("num", &mut |(n, _, _)| no_out!(n.bits())),
            ("rug", &mut |(_, n, _)| no_out!(n.significant_bits())),
        ],
    );
}
