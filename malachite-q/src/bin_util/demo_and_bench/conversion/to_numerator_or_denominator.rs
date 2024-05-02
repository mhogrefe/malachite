// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    rational_bit_bucketer, triple_3_rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_to_numerator);
    register_demo!(runner, demo_into_numerator);
    register_demo!(runner, demo_numerator_ref);
    register_demo!(runner, demo_to_denominator);
    register_demo!(runner, demo_into_denominator);
    register_demo!(runner, demo_denominator_ref);
    register_demo!(runner, demo_to_numerator_and_denominator);
    register_demo!(runner, demo_into_numerator_and_denominator);
    register_demo!(runner, demo_numerator_and_denominator_ref);

    register_bench!(runner, benchmark_to_numerator_evaluation_strategy);
    register_bench!(runner, benchmark_to_numerator_library_comparison);
    register_bench!(runner, benchmark_to_denominator_evaluation_strategy);
    register_bench!(runner, benchmark_to_denominator_library_comparison);
    register_bench!(
        runner,
        benchmark_to_numerator_and_denominator_evaluation_strategy
    );
}

fn demo_to_numerator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("to_numerator({}) = {}", x, x.to_numerator());
    }
}

fn demo_into_numerator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        println!("into_numerator({}) = {}", old_x, x.into_numerator());
    }
}

fn demo_numerator_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("numerator_ref({}) = {}", x, x.numerator_ref());
    }
}

fn demo_to_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("to_denominator({}) = {}", x, x.to_denominator());
    }
}

fn demo_into_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        println!("into_denominator({}) = {}", old_x, x.into_denominator());
    }
}

fn demo_denominator_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("denominator_ref({}) = {}", x, x.denominator_ref());
    }
}

fn demo_to_numerator_and_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "to_numerator_and_denominator({}) = {:?}",
            x,
            x.to_numerator_and_denominator()
        );
    }
}

fn demo_into_numerator_and_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        let old_x = x.clone();
        println!(
            "into_numerator_and_denominator({}) = {:?}",
            old_x,
            x.into_numerator_and_denominator()
        );
    }
}

fn demo_numerator_and_denominator_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "numerator_and_denominator_ref({}) = {:?}",
            x,
            x.numerator_and_denominator_ref()
        );
    }
}

fn benchmark_to_numerator_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_numerator()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("to_numerator", &mut |x| no_out!(x.to_numerator())),
            ("into_numerator", &mut |x| no_out!(x.into_numerator())),
            ("numerator_ref", &mut |x| no_out!(x.numerator_ref())),
        ],
    );
}

fn benchmark_to_numerator_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_numerator()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_numerator())),
            ("num", &mut |(x, _, _)| no_out!(x.numer())),
            ("rug", &mut |(_, x, _)| no_out!(x.numer())),
        ],
    );
}

fn benchmark_to_denominator_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_denominator()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("to_denominator", &mut |x| no_out!(x.to_denominator())),
            ("into_denominator", &mut |x| no_out!(x.into_denominator())),
            ("denominator_ref", &mut |x| no_out!(x.denominator_ref())),
        ],
    );
}

fn benchmark_to_denominator_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_denominator()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_denominator())),
            ("num", &mut |(x, _, _)| no_out!(x.denom())),
            ("rug", &mut |(_, x, _)| no_out!(x.denom())),
        ],
    );
}

fn benchmark_to_numerator_and_denominator_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_numerator_and_denominator()",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("to_numerator_and_denominator", &mut |x| {
                no_out!(x.to_numerator_and_denominator())
            }),
            ("into_numerator_and_denominator", &mut |x| {
                no_out!(x.into_numerator_and_denominator())
            }),
            ("numerator_and_denominator_ref", &mut |x| {
                no_out!(x.numerator_and_denominator_ref())
            }),
        ],
    );
}
