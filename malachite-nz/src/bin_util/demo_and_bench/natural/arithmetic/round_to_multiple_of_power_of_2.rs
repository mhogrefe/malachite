// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, triple_1_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_16, unsigned_vec_unsigned_pair_gen_var_20,
    unsigned_vec_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_2::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_rounding_mode_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_round_to_multiple_of_power_of_2_down);
    register_demo!(runner, demo_limbs_round_to_multiple_of_power_of_2_up);
    register_demo!(runner, demo_limbs_round_to_multiple_of_power_of_2_nearest);
    register_demo!(runner, demo_limbs_round_to_multiple_of_power_of_2);
    register_demo!(
        runner,
        demo_limbs_round_to_multiple_of_power_of_2_down_in_place
    );
    register_demo!(
        runner,
        demo_limbs_round_to_multiple_of_power_of_2_up_in_place
    );
    register_demo!(
        runner,
        demo_limbs_round_to_multiple_of_power_of_2_nearest_in_place
    );
    register_demo!(runner, demo_limbs_round_to_multiple_of_power_of_2_in_place);
    register_demo!(runner, demo_natural_round_to_multiple_of_power_of_2_assign);
    register_demo!(runner, demo_natural_round_to_multiple_of_power_of_2);
    register_demo!(runner, demo_natural_round_to_multiple_of_power_of_2_ref);

    register_bench!(runner, benchmark_limbs_round_to_multiple_of_power_of_2_down);
    register_bench!(runner, benchmark_limbs_round_to_multiple_of_power_of_2_up);
    register_bench!(
        runner,
        benchmark_limbs_round_to_multiple_of_power_of_2_nearest
    );
    register_bench!(runner, benchmark_limbs_round_to_multiple_of_power_of_2);
    register_bench!(
        runner,
        benchmark_limbs_round_to_multiple_of_power_of_2_down_in_place
    );
    register_bench!(
        runner,
        benchmark_limbs_round_to_multiple_of_power_of_2_up_in_place
    );
    register_bench!(
        runner,
        benchmark_limbs_round_to_multiple_of_power_of_2_nearest_in_place
    );
    register_bench!(
        runner,
        benchmark_limbs_round_to_multiple_of_power_of_2_in_place
    );
    register_bench!(
        runner,
        benchmark_natural_round_to_multiple_of_power_of_2_assign
    );
    register_bench!(
        runner,
        benchmark_natural_round_to_multiple_of_power_of_2_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_round_to_multiple_of_power_of_2_evaluation_strategy
    );
}

fn demo_limbs_round_to_multiple_of_power_of_2_down(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_round_to_multiple_of_power_of_2_down({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_round_to_multiple_of_power_of_2_down(&xs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_up(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_round_to_multiple_of_power_of_2_up({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_round_to_multiple_of_power_of_2_up(&xs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_nearest(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_round_to_multiple_of_power_of_2_nearest({:?}, {}) = {:?}",
            xs,
            pow,
            limbs_round_to_multiple_of_power_of_2_nearest(&xs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, pow, rm) in unsigned_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_round_to_multiple_of_power_of_2({:?}, {}, {}) = {:?}",
            xs,
            pow,
            rm,
            limbs_round_to_multiple_of_power_of_2(&xs, pow, rm)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_down_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let o = limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_round_to_multiple_of_power_of_2_down_in_place(&mut xs, {pow}) = {o:?}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_up_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let o = limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_round_to_multiple_of_power_of_2_up_in_place(&mut xs, {pow}) = {o:?}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_nearest_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, pow) in unsigned_vec_unsigned_pair_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let o = limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, pow);
        println!(
            "xs := {xs_old:?}; \
            limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut xs, {pow}) = {o:?}; \
            xs = {xs:?}",
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut xs, pow, rm) in unsigned_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let oo = limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, pow, rm);
        println!(
            "xs := {xs_old:?}; \
            limbs_round_to_multiple_of_power_of_2_in_place(&mut xs, {pow}, {rm}) = {oo:?}; \
            xs = {xs:?}",
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow, rm) in natural_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let o = n.round_to_multiple_of_power_of_2_assign(pow, rm);
        println!(
            "x := {n_old}; x.round_to_multiple_of_power_of_2_assign({pow}, {rm}) = {o:?}; x = {n}"
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow, rm) in natural_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.round_to_multiple_of_power_of_2({}, {}) = {:?}",
            n_old,
            pow,
            rm,
            n.round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_2_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow, rm) in natural_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).round_to_multiple_of_power_of_2({}, {}) = {:?}",
            n,
            pow,
            rm,
            (&n).round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_down(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_down(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_down(&xs, pow))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_up(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_up(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_up(&xs, pow))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_nearest(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_nearest(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_nearest(&xs, pow))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2(&[Limb], u64, RoundingMode)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, pow, rm)| {
            no_out!(limbs_round_to_multiple_of_power_of_2(&xs, pow, rm))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_down_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_down_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_down_in_place(
                &mut xs, pow
            ))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_up_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_up_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_20().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_up_in_place(
                &mut xs, pow
            ))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_nearest_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_nearest_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, pow)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_nearest_in_place(
                &mut xs, pow
            ))
        })],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_2_in_place(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_round_to_multiple_of_power_of_2_in_place(&mut Vec<Limb>, u64, RoundingMode)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, pow, rm)| {
            no_out!(limbs_round_to_multiple_of_power_of_2_in_place(
                &mut xs, pow, rm
            ))
        })],
    );
}

fn benchmark_natural_round_to_multiple_of_power_of_2_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.round_to_multiple_of_power_of_2_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        natural_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2_assign(y, rm))
        })],
    );
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_round_to_multiple_of_power_of_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.round_to_multiple_of_power_of_2(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        natural_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |(x, y, rm)| {
                no_out!(x.round_to_multiple_of_power_of_2(y, rm))
            }),
            ("using shr_round", &mut |(x, y, rm)| {
                no_out!(x.shr_round(y, rm).0 << y)
            }),
            ("using round_to_multiple", &mut |(x, y, rm)| {
                no_out!(x.round_to_multiple(Natural::power_of_2(y), rm))
            }),
        ],
    );
}

fn benchmark_natural_round_to_multiple_of_power_of_2_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.round_to_multiple_of_power_of_2(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("n"),
        &mut [
            (
                "Natural.round_to_multiple_of_power_of_2(u64, RoundingMode)",
                &mut |(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_2(y, rm)),
            ),
            (
                "(&Natural).round_to_multiple_of_power_of_2(u64, RoundingMode)",
                &mut |(x, y, rm)| no_out!((&x).round_to_multiple_of_power_of_2(y, rm)),
            ),
        ],
    );
}
