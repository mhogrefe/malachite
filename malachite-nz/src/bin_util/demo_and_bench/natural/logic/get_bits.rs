// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_3;
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::logic::bit_block_access::{limbs_slice_get_bits, limbs_vec_get_bits};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_unsigned_triple_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_slice_get_bits);
    register_demo!(runner, demo_limbs_vec_get_bits);
    register_demo!(runner, demo_natural_get_bits);
    register_demo!(runner, demo_natural_get_bits_owned);

    register_bench!(runner, benchmark_limbs_get_bits_evaluation_strategy);
    register_bench!(runner, benchmark_natural_get_bits_evaluation_strategy);
    register_bench!(runner, benchmark_natural_get_bits_algorithms);
}

fn demo_limbs_slice_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, start, end) in unsigned_vec_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_slice_get_bits({:?}, {}, {}) = {:?}",
            xs,
            start,
            end,
            limbs_slice_get_bits(&xs, start, end)
        );
    }
}

fn demo_limbs_vec_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, start, end) in unsigned_vec_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_vec_get_bits({:?}, {}, {}) = {:?}",
            old_xs,
            start,
            end,
            limbs_vec_get_bits(xs, start, end)
        );
    }
}

fn demo_natural_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, start, end) in natural_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_natural_get_bits_owned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, start, end) in natural_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n.clone();
        println!(
            "{}.get_bits_owned({}, {}) = {}",
            old_n,
            start,
            end,
            n.get_bits_owned(start, end)
        );
    }
}

fn benchmark_limbs_get_bits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_get_bits(&[Limb], u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_slice_get_bits", &mut |(xs, start, end)| {
                no_out!(limbs_slice_get_bits(&xs, start, end))
            }),
            ("limbs_vec_get_bits", &mut |(xs, start, end)| {
                no_out!(limbs_vec_get_bits(xs, start, end))
            }),
        ],
    );
}

fn benchmark_natural_get_bits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.get_bits(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            ("get_bits", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("get_bits_owned", &mut |(n, start, end)| {
                no_out!(n.get_bits_owned(start, end))
            }),
        ],
    );
}

fn benchmark_natural_get_bits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.get_bits(u64, u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<Natural, Natural>(&n, start, end))
            }),
        ],
    );
}
