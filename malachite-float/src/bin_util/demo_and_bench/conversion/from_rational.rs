// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    rational_unsigned_rounding_mode_triple_gen_var_1,
    rational_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    pair_rational_bit_u64_max_bucketer, triple_1_2_rational_bit_u64_max_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_from_rational_prec);
    register_demo!(runner, demo_float_from_rational_prec_debug);
    register_demo!(runner, demo_float_from_rational_prec_ref);
    register_demo!(runner, demo_float_from_rational_prec_ref_debug);
    register_demo!(runner, demo_float_from_rational_prec_round);
    register_demo!(runner, demo_float_from_rational_prec_round_debug);
    register_demo!(runner, demo_float_from_rational_prec_round_ref);
    register_demo!(runner, demo_float_from_rational_prec_round_ref_debug);
    register_demo!(runner, demo_float_try_from_rational);
    register_demo!(runner, demo_float_try_from_rational_debug);
    register_demo!(runner, demo_float_try_from_rational_ref);
    register_demo!(runner, demo_float_try_from_rational_ref_debug);
    register_demo!(runner, demo_float_convertible_from_rational);

    register_bench!(
        runner,
        benchmark_float_from_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_from_rational_prec_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_from_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_from_rational_prec_round_library_comparison
    );
}

fn demo_float_from_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_rational_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::from_rational_prec(n, p)
        );
    }
}

fn demo_float_from_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_rational_prec(n.clone(), p);
        println!(
            "Float::from_rational_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_from_rational_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_rational_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::from_rational_prec_ref(&n, p)
        );
    }
}

fn demo_float_from_rational_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in rational_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_rational_prec_ref(&n, p);
        println!(
            "Float::from_rational_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_rational_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::from_rational_prec_round(n, p, rm)
        );
    }
}

fn demo_float_from_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_rational_prec_round(n.clone(), p, rm);
        println!(
            "Float::from_rational_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_rational_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_rational_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::from_rational_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_from_rational_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in rational_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_rational_prec_round_ref(&n, p, rm);
        println!(
            "Float::from_rational_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_try_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("Float::try_from({}) = {:?}", x.clone(), Float::try_from(x));
    }
}

fn demo_float_try_from_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from({}) = {:?}",
            x.clone(),
            Float::try_from(x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_try_from_rational_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!("Float::try_from(&{}) = {:?}", x, Float::try_from(&x));
    }
}

fn demo_float_try_from_rational_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from(&{}) = {:?}",
            x,
            Float::try_from(&x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_convertible_from_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Float",
            x,
            if Float::convertible_from(&x) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_float_from_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_rational_prec(Rational, u64)",
                &mut |(n, prec)| no_out!(Float::from_rational_prec(n, prec)),
            ),
            (
                "Float::from_rational_prec_ref(&Rational, u64)",
                &mut |(n, prec)| no_out!(Float::from_rational_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_from_rational_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_rational_prec(Rational, u64)",
        BenchmarkType::LibraryComparison,
        rational_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec)| {
                no_out!(Float::from_rational_prec(n, prec))
            }),
            ("rug", &mut |(n, prec)| {
                no_out!(rug::Float::with_val(
                    u32::exact_from(prec),
                    rug::Rational::from(&n),
                ))
            }),
        ],
    );
}

fn benchmark_float_from_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_rational_prec(Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_rational_prec_round(n, prec, rm)),
            ),
            (
                "Float::from_rational_prec_ref(&Rational, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_rational_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_from_rational_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        rational_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_rational_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec, rm)| {
                no_out!(Float::from_rational_prec_round(n, prec, rm))
            }),
            ("rug", &mut |(n, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Rational::from(&n),
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ))
            }),
        ],
    );
}
