// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::bench::bucketers::{triple_1_vec_len_bucketer, triple_3_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_20, unsigned_vec_unsigned_unsigned_triple_gen_var_4,
};
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::logic::bit_block_access::{
    limbs_neg_limb_get_bits, limbs_slice_neg_get_bits, limbs_vec_neg_get_bits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_bit_bucketer;
use malachite_nz::test_util::generators::integer_unsigned_unsigned_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_neg_limb_get_bits);
    register_demo!(runner, demo_limbs_slice_neg_get_bits);
    register_demo!(runner, demo_limbs_vec_neg_get_bits);
    register_demo!(runner, demo_integer_get_bits);

    register_bench!(runner, benchmark_limbs_neg_limb_get_bits);
    register_bench!(runner, benchmark_limbs_neg_get_bits_evaluation_strategy);
    register_bench!(runner, benchmark_integer_get_bits_evaluation_strategy);
    register_bench!(runner, benchmark_integer_get_bits_algorithms);
}

fn demo_limbs_neg_limb_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, start, end) in unsigned_triple_gen_var_20().get(gm, config).take(limit) {
        println!(
            "limbs_neg_limb_get_bits({}, {}, {}) = {:?}",
            x,
            start,
            end,
            limbs_neg_limb_get_bits(x, start, end)
        );
    }
}

fn demo_limbs_slice_neg_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, start, end) in unsigned_vec_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_slice_neg_get_bits({:?}, {}, {}) = {:?}",
            xs,
            start,
            end,
            limbs_slice_neg_get_bits(&xs, start, end)
        );
    }
}

fn demo_limbs_vec_neg_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, start, end) in unsigned_vec_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_vec_neg_get_bits({:?}, {}, {}) = {:?}",
            old_xs,
            start,
            end,
            limbs_vec_neg_get_bits(xs, start, end)
        );
    }
}

fn demo_integer_get_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, start, end) in integer_unsigned_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn benchmark_limbs_neg_limb_get_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_limb_get_bits(Limb, u64, u64)",
        BenchmarkType::Single,
        unsigned_triple_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("end"),
        &mut [("limbs_neg_limb_get_bits", &mut |(x, start, end)| {
            no_out!(limbs_neg_limb_get_bits(x, start, end))
        })],
    );
}

fn benchmark_limbs_neg_get_bits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_get_bits(&[Limb], u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_slice_neg_get_bits", &mut |(xs, start, end)| {
                no_out!(limbs_slice_neg_get_bits(&xs, start, end))
            }),
            ("limbs_vec_neg_get_bits", &mut |(xs, start, end)| {
                no_out!(limbs_vec_neg_get_bits(xs, start, end))
            }),
        ],
    );
}

fn benchmark_integer_get_bits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.get_bits(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
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

fn benchmark_integer_get_bits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.get_bits(u64, u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, start, end)| {
                no_out!(n.get_bits(start, end))
            }),
            ("naive", &mut |(n, start, end)| {
                no_out!(get_bits_naive::<Integer, Natural>(&n, start, end))
            }),
        ],
    );
}
