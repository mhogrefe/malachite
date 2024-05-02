// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::prefix_to_string;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_7, rational_unsigned_pair_gen_var_2,
    rational_unsigned_pair_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_into_power_of_2_digits);
    register_demo!(runner, demo_rational_to_power_of_2_digits);
    register_demo!(runner, demo_rational_into_power_of_2_digits_binary);
    register_demo!(runner, demo_rational_power_of_2_digits);
    register_demo!(runner, demo_rational_power_of_2_digits_binary);
    register_bench!(
        runner,
        benchmark_rational_to_power_of_2_digits_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_power_of_2_digits);
}

fn to_string_helper(p: (Vec<Natural>, RationalSequence<Natural>)) -> String {
    let (before, after) = p;
    let mut s = "(".to_string();
    s.push_str(&before.to_debug_string());
    s.push_str(", ");
    s.push_str(&after.to_string());
    s.push(')');
    s
}

fn demo_rational_into_power_of_2_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in rational_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).into_power_of_2_digits({}) = {}",
            n.clone(),
            log_base,
            to_string_helper(n.into_power_of_2_digits(log_base))
        );
    }
}

fn demo_rational_to_power_of_2_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in rational_unsigned_pair_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).to_power_of_2_digits({}) = {}",
            n,
            log_base,
            to_string_helper(n.to_power_of_2_digits(log_base))
        );
    }
}

fn demo_rational_into_power_of_2_digits_binary(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen_var_7().get(gm, config).take(limit) {
        println!(
            "({}).into_power_of_2_digits(1) = {}",
            n.clone(),
            to_string_helper(n.into_power_of_2_digits(1))
        );
    }
}

fn demo_rational_power_of_2_digits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, log_base) in rational_unsigned_pair_gen_var_3::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let (before_point, after_point) = n.power_of_2_digits(log_base);
        println!(
            "({}).power_of_2_digits({}) = ({:?}, {})",
            n,
            log_base,
            before_point,
            prefix_to_string(after_point, 20)
        );
    }
}

fn demo_rational_power_of_2_digits_binary(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        let (before_point, after_point) = n.power_of_2_digits(1);
        println!(
            "({}).power_of_2_digits(1) = ({:?}, {})",
            n,
            before_point,
            prefix_to_string(after_point, 20)
        );
    }
}

fn benchmark_rational_to_power_of_2_digits_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_power_of_2_digits(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            (
                "Rational.into_power_of_2_digits(log_base)",
                &mut |(n, log_base)| no_out!(n.into_power_of_2_digits(log_base)),
            ),
            (
                "Rational.to_power_of_2_digits(log_base)",
                &mut |(n, log_base)| no_out!(n.to_power_of_2_digits(log_base)),
            ),
        ],
    );
}

fn benchmark_rational_power_of_2_digits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.power_of_2_digits(u64)",
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_3::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [(
            "Rational.power_of_2_digits(log_base).1.take(20).collect_vec()",
            &mut |(n, log_base)| no_out!(n.power_of_2_digits(log_base).1.take(20).collect_vec()),
        )],
    );
}
