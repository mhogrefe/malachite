// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    integer_unsigned_rounding_mode_triple_gen_var_3,
    integer_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, Float};
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_integer_bit_u64_max_bucketer,
    triple_1_2_integer_bit_u64_max_bucketer,
};
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_6};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_from_integer_prec);
    register_demo!(runner, demo_float_from_integer_prec_debug);
    register_demo!(runner, demo_float_from_integer_prec_ref);
    register_demo!(runner, demo_float_from_integer_prec_ref_debug);
    register_demo!(runner, demo_float_from_integer_prec_round);
    register_demo!(runner, demo_float_from_integer_prec_round_debug);
    register_demo!(runner, demo_float_from_integer_prec_round_ref);
    register_demo!(runner, demo_float_from_integer_prec_round_ref_debug);
    register_demo!(runner, demo_float_try_from_integer);
    register_demo!(runner, demo_float_try_from_integer_debug);
    register_demo!(runner, demo_float_try_from_integer_ref);
    register_demo!(runner, demo_float_try_from_integer_ref_debug);
    register_demo!(runner, demo_float_convertible_from_integer);

    register_bench!(
        runner,
        benchmark_float_from_integer_prec_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_from_integer_prec_library_comparison);
    register_bench!(
        runner,
        benchmark_float_from_integer_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_from_integer_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_try_from_integer_evaluation_strategy);
    register_bench!(runner, benchmark_float_convertible_from_integer);
}

fn demo_float_from_integer_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in integer_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_integer_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::from_integer_prec(n, p)
        );
    }
}

fn demo_float_from_integer_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in integer_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_integer_prec(n.clone(), p);
        println!(
            "Float::from_integer_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_from_integer_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in integer_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_integer_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::from_integer_prec_ref(&n, p)
        );
    }
}

fn demo_float_from_integer_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in integer_unsigned_pair_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_integer_prec_ref(&n, p);
        println!(
            "Float::from_integer_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_integer_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in integer_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_integer_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::from_integer_prec_round(n, p, rm)
        );
    }
}

fn demo_float_from_integer_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in integer_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_integer_prec_round(n.clone(), p, rm);
        println!(
            "Float::from_integer_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_integer_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in integer_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_integer_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::from_integer_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_from_integer_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in integer_unsigned_rounding_mode_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_integer_prec_round_ref(&n, p, rm);
        println!(
            "Float::from_integer_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_try_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!("Float::try_from({}) = {:?}", x.clone(), Float::try_from(x));
    }
}

fn demo_float_try_from_integer_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from({}) = {:?}",
            x.clone(),
            Float::try_from(x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_try_from_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!("Float::try_from(&{}) = {:?}", x, Float::try_from(&x));
    }
}

fn demo_float_try_from_integer_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from(&{}) = {:?}",
            x,
            Float::try_from(&x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_convertible_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_gen().get(gm, config).take(limit) {
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

fn benchmark_float_from_integer_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_integer_prec(Integer, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_integer_prec(Integer, u64)",
                &mut |(n, prec)| no_out!(Float::from_integer_prec(n, prec)),
            ),
            (
                "Float::from_integer_prec_ref(&Integer, u64)",
                &mut |(n, prec)| no_out!(Float::from_integer_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_from_integer_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_integer_prec(Integer, u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec)| {
                no_out!(Float::from_integer_prec(n, prec));
            }),
            ("rug", &mut |(n, prec)| {
                no_out!(rug::Float::with_val(
                    u32::exact_from(prec),
                    rug::Integer::from(&n),
                ));
            }),
        ],
    );
}

fn benchmark_float_from_integer_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_integer_prec(Integer, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_integer_prec_round(n, prec, rm)),
            ),
            (
                "Float::from_integer_prec_ref(&Integer, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_integer_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_from_integer_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_integer_prec_round(Integer, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_rounding_mode_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec, rm)| {
                no_out!(Float::from_integer_prec_round(n, prec, rm));
            }),
            ("rug", &mut |(n, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Integer::from(&n),
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_try_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::try_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Float::try_from(Integer)", &mut |x| {
                no_out!(Float::try_from(x));
            }),
            ("Float::try_from(&Integer)", &mut |x| {
                no_out!(Float::try_from(&x));
            }),
        ],
    );
}

fn benchmark_float_convertible_from_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::convertible_from(Integer)",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Float::convertible_from(&x)))],
    );
}
