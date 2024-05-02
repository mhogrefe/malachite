// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::BitAccess;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_20;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::logic::bit_access::{
    limbs_slice_clear_bit_neg, limbs_vec_clear_bit_neg,
};
use malachite_nz::test_util::bench::bucketers::pair_integer_bit_u64_max_bucketer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use malachite_nz::test_util::generators::unsigned_vec_unsigned_pair_gen_var_21;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_slice_clear_bit_neg);
    register_demo!(runner, demo_limbs_vec_clear_bit_neg);
    register_demo!(runner, demo_integer_clear_bit);

    register_bench!(runner, benchmark_limbs_slice_clear_bit_neg);
    register_bench!(runner, benchmark_limbs_vec_clear_bit_neg);
    register_bench!(runner, benchmark_integer_clear_bit);
}

fn demo_limbs_slice_clear_bit_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, index) in unsigned_vec_unsigned_pair_gen_var_21()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_slice_clear_bit_neg(&mut xs, index);
        println!("xs := {xs_old:?}; limbs_slice_clear_bit_neg(&mut xs, {index}); xs = {xs:?}");
    }
}

fn demo_limbs_vec_clear_bit_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, index) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        limbs_vec_clear_bit_neg(&mut xs, index);
        println!("xs := {old_xs:?}; limbs_vec_clear_bit_neg(&mut xs, {index}); xs = {xs:?}");
    }
}

fn demo_integer_clear_bit(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {n_old}; x.clear_bit({index}); x = {n}");
    }
}

fn benchmark_limbs_slice_clear_bit_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_clear_bit_neg(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut xs, index)| {
            no_out!(limbs_slice_clear_bit_neg(&mut xs, index))
        })],
    );
}

fn benchmark_limbs_vec_clear_bit_neg(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_clear_bit_neg(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut xs, index)| {
            no_out!(limbs_vec_clear_bit_neg(&mut xs, index))
        })],
    );
}

fn benchmark_integer_clear_bit(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.clear_bit(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_bit_u64_max_bucketer("x", "index"),
        &mut [("Malachite", &mut |(mut n, index)| n.clear_bit(index))],
    );
}
