// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    triple_1_vec_float_sum_complexity_bucketer, vec_float_sum_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_vec_gen, float_vec_gen_var_1, float_vec_rounding_mode_pair_gen_var_3,
    float_vec_rounding_mode_pair_gen_var_4, float_vec_unsigned_pair_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_3,
    float_vec_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_product);
    register_demo!(runner, demo_float_product_debug);
    register_demo!(runner, demo_float_product_extreme);
    register_demo!(runner, demo_float_product_extreme_debug);
    register_demo!(runner, demo_float_ref_product);
    register_demo!(runner, demo_float_ref_product_debug);
    register_demo!(runner, demo_float_product_prec);
    register_demo!(runner, demo_float_product_prec_debug);
    register_demo!(runner, demo_float_product_round);
    register_demo!(runner, demo_float_product_round_debug);
    register_demo!(runner, demo_float_product_round_extreme);
    register_demo!(runner, demo_float_product_round_extreme_debug);
    register_demo!(runner, demo_float_product_prec_round);
    register_demo!(runner, demo_float_product_prec_round_debug);
    register_demo!(runner, demo_float_product_prec_round_extreme);
    register_demo!(runner, demo_float_product_prec_round_extreme_debug);

    register_bench!(runner, benchmark_float_product_evaluation_strategy);
    register_bench!(runner, benchmark_float_product_prec_round_algorithms);
}

fn demo_float_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {}",
            xs.clone(),
            Float::product(xs.into_iter())
        );
    }
}

fn demo_float_product_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::product(xs.into_iter()))
        );
    }
}

fn demo_float_product_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {}",
            xs.clone(),
            Float::product(xs.into_iter())
        );
    }
}

fn demo_float_product_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen_var_1().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::product(xs.into_iter()))
        );
    }
}

fn demo_float_ref_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!("product({:?}) = {}", xs, Float::product(xs.iter()));
    }
}

fn demo_float_ref_product_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in float_vec_gen().get(gm, config).take(limit) {
        println!(
            "product({:?}) = {:#x}",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            ComparableFloat(Float::product(xs.iter()))
        );
    }
}

fn demo_float_product_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec) in float_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec(&xs, prec);
        println!("Float::product_prec(&{xs:?}, {prec}) = ({product}, {o:?})");
    }
}

fn demo_float_product_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec) in float_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec(&xs, prec);
        println!(
            "Float::product_prec(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            ComparableFloat(product),
            o
        );
    }
}

fn demo_float_product_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_round(&xs, rm);
        println!("Float::product_round(&{xs:?}, {rm}) = ({product}, {o:?})");
    }
}

fn demo_float_product_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_round(&xs, rm);
        println!(
            "Float::product_round(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            rm,
            ComparableFloat(product),
            o
        );
    }
}

fn demo_float_product_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_round(&xs, rm);
        println!("Float::product_round(&{xs:?}, {rm}) = ({product}, {o:?})");
    }
}

fn demo_float_product_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, rm) in float_vec_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_round(&xs, rm);
        println!(
            "Float::product_round(&{:?}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            rm,
            ComparableFloat(product),
            o
        );
    }
}

fn demo_float_product_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec_round(&xs, prec, rm);
        println!("Float::product_prec_round(&{xs:?}, {prec}, {rm}) = ({product}, {o:?})");
    }
}

fn demo_float_product_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec_round(&xs, prec, rm);
        println!(
            "Float::product_prec_round(&{:?}, {}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            rm,
            ComparableFloat(product),
            o
        );
    }
}

fn demo_float_product_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec_round(&xs, prec, rm);
        println!("Float::product_prec_round(&{xs:?}, {prec}, {rm}) = ({product}, {o:?})");
    }
}

fn demo_float_product_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in float_vec_unsigned_rounding_mode_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (product, o) = Float::product_prec_round(&xs, prec, rm);
        println!(
            "Float::product_prec_round(&{:?}, {}, {}) = ({:#x}, {:?})",
            xs.iter()
                .map(|x| ComparableFloat(x.clone()))
                .collect::<Vec<_>>(),
            prec,
            rm,
            ComparableFloat(product),
            o
        );
    }
}

fn benchmark_float_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::product(Iterator<Item=Float>)",
        BenchmarkType::EvaluationStrategy,
        float_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("Float::product(Iterator<Item=Float>)", &mut |xs| {
                no_out!(Float::product(xs.into_iter()));
            }),
            ("Float::product(Iterator<Item=&Float>)", &mut |xs| {
                no_out!(Float::product(xs.iter()));
            }),
        ],
    );
}

fn benchmark_float_product_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::product_prec_round(&[Float], u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_vec_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_float_sum_complexity_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, prec, rm)| {
                no_out!(Float::product_prec_round(&xs, prec, rm));
            }),
            ("exact Rational route", &mut |(xs, prec, rm)| {
                // The Rational route only applies to finite inputs; fall back for the rare vectors
                // containing specials.
                if xs.iter().all(Float::is_finite) {
                    let exact = Rational::product(xs.iter().map(Rational::exact_from));
                    no_out!(Float::from_rational_prec_round(exact, prec, rm));
                } else {
                    no_out!(Float::product_prec_round(&xs, prec, rm));
                }
            }),
        ],
    );
}
