// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::sum::primitive_float_sum;
use malachite_float::test_util::bench::bucketers::{
    pair_2_triple_1_vec_float_sum_complexity_bucketer, pair_2_vec_float_sum_complexity_bucketer,
    triple_1_vec_float_sum_complexity_bucketer, vec_float_sum_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::sum::{rug_sum, rug_sum_prec_round};
use malachite_float::test_util::generators::{
    float_vec_gen, float_vec_gen_rm, float_vec_gen_var_1, float_vec_rounding_mode_pair_gen_var_1,
    float_vec_rounding_mode_pair_gen_var_2, float_vec_unsigned_pair_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_1_rm,
    float_vec_unsigned_rounding_mode_triple_gen_var_2, primitive_float_vec_gen_var_1,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sum);
    register_demo!(runner, demo_float_sum_debug);
    register_demo!(runner, demo_float_sum_extreme);
    register_demo!(runner, demo_float_sum_extreme_debug);
    register_demo!(runner, demo_float_ref_sum);
    register_demo!(runner, demo_float_ref_sum_debug);
    register_demo!(runner, demo_float_sum_prec);
    register_demo!(runner, demo_float_sum_prec_debug);
    register_demo!(runner, demo_float_sum_round);
    register_demo!(runner, demo_float_sum_round_debug);
    register_demo!(runner, demo_float_sum_round_extreme);
    register_demo!(runner, demo_float_sum_round_extreme_debug);
    register_demo!(runner, demo_float_sum_prec_round);
    register_demo!(runner, demo_float_sum_prec_round_debug);
    register_demo!(runner, demo_float_sum_prec_round_extreme);
    register_demo!(runner, demo_float_sum_prec_round_extreme_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_sum);

    register_bench!(runner, benchmark_float_sum_evaluation_strategy);
    register_bench!(runner, benchmark_float_sum_library_comparison);
    register_bench!(runner, benchmark_float_sum_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_sum_prec_round_algorithms);
    register_primitive_float_benches!(runner, benchmark_primitive_float_sum);
}

fn demo_float_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!("sum({:?}) = {}", xs.clone(), Float::sum(xs.into_iter()));
    }
}

fn demo_float_sum_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!(
            "sum({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::sum(xs.into_iter()))
        );
    }
}

fn demo_float_sum_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen_var_1().get(gm, config).take(limit) {
        println!("sum({:?}) = {}", xs.clone(), Float::sum(xs.into_iter()));
    }
}

fn demo_float_sum_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "sum({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::sum(xs.into_iter()))
        );
    }
}

fn demo_float_ref_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!("sum({:?}) = {}", xs, Float::sum(xs.iter()));
    }
}

fn demo_float_ref_sum_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!(
            "sum({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::sum(xs.iter()))
        );
    }
}

fn demo_float_sum_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec) in float_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec(&xs, prec);
        println!("Float::sum_prec(&{xs:?}, {prec}) = ({sum}, {o:?})");
    }
}

fn demo_float_sum_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec) in float_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec(&xs, prec);
        println!(
            "Float::sum_prec(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_sum_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_round(&xs, rm);
        println!("Float::sum_round(&{xs:?}, {rm}) = ({sum}, {o:?})");
    }
}

fn demo_float_sum_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_round(&xs, rm);
        println!(
            "Float::sum_round(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_sum_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_round(&xs, rm);
        println!("Float::sum_round(&{xs:?}, {rm}) = ({sum}, {o:?})");
    }
}

fn demo_float_sum_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_round(&xs, rm);
        println!(
            "Float::sum_round(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_sum_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
        println!("Float::sum_prec_round(&{xs:?}, {prec}, {rm}) = ({sum}, {o:?})");
    }
}

fn demo_float_sum_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
        println!(
            "Float::sum_prec_round(&{:?}, {}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_sum_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
        println!("Float::sum_prec_round(&{xs:?}, {prec}, {rm}) = ({sum}, {o:?})");
    }
}

fn demo_float_sum_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
        println!(
            "Float::sum_prec_round(&{:?}, {}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn benchmark_float_sum_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sum(Iterator<Item=Float>)",
        BenchmarkType::EvaluationStrategy,
        float_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("Float::sum(Iterator<Item=Float>)", &mut |xs| {
                no_out!(Float::sum(xs.into_iter()));
            }),
            ("Float::sum(Iterator<Item=&Float>)", &mut |xs| {
                no_out!(Float::sum(xs.iter()));
            }),
        ],
    );
}

fn benchmark_float_sum_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sum(Iterator<Item=Float>)",
        BenchmarkType::LibraryComparison,
        float_vec_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("Malachite", &mut |(_, xs)| {
                no_out!(Float::sum(xs.into_iter()));
            }),
            ("rug", &mut |(xs, _)| no_out!(rug_sum(&xs))),
        ],
    );
}

fn benchmark_float_sum_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sum_prec_round(&[Float], u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_vec_unsigned_rounding_mode_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("Malachite", &mut |(_, (xs, prec, rm))| {
                no_out!(Float::sum_prec_round(&xs, prec, rm));
            }),
            ("rug", &mut |((xs, prec, rm), _)| {
                no_out!(rug_sum_prec_round(&xs, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_sum_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::sum_prec_round(&[Float], u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_vec_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, prec, rm)| {
                no_out!(Float::sum_prec_round(&xs, prec, rm));
            }),
            ("exact Rational route", &mut |(xs, prec, rm)| {
                // The Rational route only applies to finite inputs; fall back for the rare vectors
                // containing specials.
                if xs.iter().all(Float::is_finite) {
                    let exact: Rational = xs.iter().map(Rational::exact_from).sum();
                    no_out!(Float::from_rational_prec_round(exact, prec, rm));
                } else {
                    no_out!(Float::sum_prec_round(&xs, prec, rm));
                }
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_sum<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    for xs in primitive_float_vec_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_sum({:?}) = {}",
            xs.iter().copied().map(NiceFloat).collect::<Vec<_>>(),
            NiceFloat(primitive_float_sum(&xs))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_sum<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_sum(&[{}])", T::NAME),
        BenchmarkType::Single,
        primitive_float_vec_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("malachite", &mut |xs| {
            no_out!(primitive_float_sum(&xs));
        })],
    );
}
