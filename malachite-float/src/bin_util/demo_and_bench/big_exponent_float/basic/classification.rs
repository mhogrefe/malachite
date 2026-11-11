// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer, pair_2_big_exponent_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_is_nan);
    register_demo!(runner, demo_big_exponent_float_is_finite);
    register_demo!(runner, demo_big_exponent_float_is_infinite);
    register_demo!(runner, demo_big_exponent_float_is_zero);
    register_demo!(runner, demo_big_exponent_float_is_normal);
    register_demo!(runner, demo_big_exponent_float_is_sign_positive);
    register_demo!(runner, demo_big_exponent_float_is_sign_negative);
    register_demo!(runner, demo_big_exponent_float_classify);
    register_demo!(runner, demo_big_exponent_float_into_non_nan);
    register_demo!(runner, demo_big_exponent_float_to_non_nan);
    register_demo!(runner, demo_big_exponent_float_into_finite);
    register_demo!(runner, demo_big_exponent_float_to_finite);

    register_bench!(runner, benchmark_big_exponent_float_is_nan);
    register_bench!(runner, benchmark_big_exponent_float_is_finite);
    register_bench!(runner, benchmark_big_exponent_float_is_infinite);
    register_bench!(runner, benchmark_big_exponent_float_is_zero);
    register_bench!(runner, benchmark_big_exponent_float_is_normal);
    register_bench!(runner, benchmark_big_exponent_float_is_sign_positive);
    register_bench!(runner, benchmark_big_exponent_float_is_sign_negative);
    register_bench!(
        runner,
        benchmark_big_exponent_float_classify_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_to_non_nan_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_to_finite_evaluation_strategy
    );
}

fn demo_big_exponent_float_is_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_nan() = {}", x.is_nan());
    }
}

fn demo_big_exponent_float_is_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_finite() = {}", x.is_finite());
    }
}

fn demo_big_exponent_float_is_infinite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_infinite() = {}", x.is_infinite());
    }
}

fn demo_big_exponent_float_is_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_zero() = {}", x.is_zero());
    }
}

fn demo_big_exponent_float_is_normal(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_normal() = {}", x.is_normal());
    }
}

fn demo_big_exponent_float_is_sign_positive(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_sign_positive() = {}", x.is_sign_positive());
    }
}

fn demo_big_exponent_float_is_sign_negative(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.is_sign_negative() = {}", x.is_sign_negative());
    }
}

fn demo_big_exponent_float_classify(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.classify() = {:?}", x.classify());
    }
}

fn demo_big_exponent_float_into_non_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.into_non_nan() = {:?}", x.clone().into_non_nan());
    }
}

fn demo_big_exponent_float_to_non_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.to_non_nan() = {:?}", x.to_non_nan());
    }
}

fn demo_big_exponent_float_into_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.into_finite() = {:?}", x.clone().into_finite());
    }
}

fn demo_big_exponent_float_to_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.to_finite() = {:?}", x.to_finite());
    }
}

fn benchmark_big_exponent_float_is_nan(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_nan()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_nan()))],
    );
}

fn benchmark_big_exponent_float_is_finite(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_finite()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_finite()))],
    );
}

fn benchmark_big_exponent_float_is_infinite(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_infinite()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_infinite()))],
    );
}

fn benchmark_big_exponent_float_is_zero(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_zero()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_zero()))],
    );
}

fn benchmark_big_exponent_float_is_normal(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_normal()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_normal()))],
    );
}

fn benchmark_big_exponent_float_is_sign_positive(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_sign_positive()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_sign_positive()))],
    );
}

fn benchmark_big_exponent_float_is_sign_negative(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.is_sign_negative()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.is_sign_negative()))],
    );
}

fn benchmark_big_exponent_float_classify_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.classify()",
        BenchmarkType::LibraryComparison,
        big_exponent_float_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(_, x)| {
                no_out!(x.classify());
            }),
            ("Float", &mut |(f, _)| {
                no_out!(f.classify());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_to_non_nan_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_non_nan()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("to_non_nan", &mut |x| {
                no_out!(x.to_non_nan());
            }),
            ("into_non_nan", &mut |x| {
                no_out!(x.into_non_nan());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_to_finite_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_finite()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("to_finite", &mut |x| {
                no_out!(x.to_finite());
            }),
            ("into_finite", &mut |x| {
                no_out!(x.into_finite());
            }),
        ],
    );
}
