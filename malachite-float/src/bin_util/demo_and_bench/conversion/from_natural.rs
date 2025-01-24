// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    natural_unsigned_rounding_mode_triple_gen_var_2,
    natural_unsigned_rounding_mode_triple_gen_var_3,
};
use malachite_float::{ComparableFloat, Float};
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_natural_bit_u64_max_bucketer,
    triple_1_2_natural_bit_u64_max_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_7};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_from_natural_prec);
    register_demo!(runner, demo_float_from_natural_prec_debug);
    register_demo!(runner, demo_float_from_natural_prec_ref);
    register_demo!(runner, demo_float_from_natural_prec_ref_debug);
    register_demo!(runner, demo_float_from_natural_prec_round);
    register_demo!(runner, demo_float_from_natural_prec_round_debug);
    register_demo!(runner, demo_float_from_natural_prec_round_ref);
    register_demo!(runner, demo_float_from_natural_prec_round_ref_debug);
    register_demo!(runner, demo_float_try_from_natural);
    register_demo!(runner, demo_float_try_from_natural_debug);
    register_demo!(runner, demo_float_try_from_natural_ref);
    register_demo!(runner, demo_float_try_from_natural_ref_debug);
    register_demo!(runner, demo_float_convertible_from_natural);

    register_bench!(
        runner,
        benchmark_float_from_natural_prec_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_from_natural_prec_library_comparison);
    register_bench!(
        runner,
        benchmark_float_from_natural_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_from_natural_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_try_from_natural_evaluation_strategy);
    register_bench!(runner, benchmark_float_convertible_from_natural);
}

fn demo_float_from_natural_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_natural_prec({}, {}) = {:?}",
            n.clone(),
            p,
            Float::from_natural_prec(n, p)
        );
    }
}

fn demo_float_from_natural_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_natural_prec(n.clone(), p);
        println!(
            "Float::from_natural_prec({}, {}) = ({:#x}, {:?})",
            n,
            p,
            ComparableFloat(f),
            o
        );
    }
}

fn demo_float_from_natural_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_natural_prec_ref(&{}, {}) = {:?}",
            n,
            p,
            Float::from_natural_prec_ref(&n, p)
        );
    }
}

fn demo_float_from_natural_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p) in natural_unsigned_pair_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_natural_prec_ref(&n, p);
        println!(
            "Float::from_natural_prec_ref(&{}, {}) = {:x?}",
            n,
            p,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_natural_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in natural_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_natural_prec_round({}, {}, {:?}) = {:?}",
            n.clone(),
            p,
            rm,
            Float::from_natural_prec_round(n, p, rm)
        );
    }
}

fn demo_float_from_natural_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in natural_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_natural_prec_round(n.clone(), p, rm);
        println!(
            "Float::from_natural_prec_round({}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_from_natural_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in natural_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_natural_prec_round_ref(&{}, {}, {:?}) = {:?}",
            n,
            p,
            rm,
            Float::from_natural_prec_round_ref(&n, p, rm)
        );
    }
}

fn demo_float_from_natural_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, p, rm) in natural_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (f, o) = Float::from_natural_prec_round_ref(&n, p, rm);
        println!(
            "Float::from_natural_prec_round_ref(&{}, {}, {:?}) = {:x?}",
            n,
            p,
            rm,
            (ComparableFloat(f), o)
        );
    }
}

fn demo_float_try_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("Float::try_from({}) = {:?}", x.clone(), Float::try_from(x));
    }
}

fn demo_float_try_from_natural_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from({}) = {:?}",
            x.clone(),
            Float::try_from(x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_try_from_natural_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("Float::try_from(&{}) = {:?}", x, Float::try_from(&x));
    }
}

fn demo_float_try_from_natural_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from(&{}) = {:?}",
            x,
            Float::try_from(&x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_float_convertible_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
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

fn benchmark_float_from_natural_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_natural_prec(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_natural_prec(Natural, u64)",
                &mut |(n, prec)| no_out!(Float::from_natural_prec(n, prec)),
            ),
            (
                "Float::from_natural_prec_ref(&Natural, u64)",
                &mut |(n, prec)| no_out!(Float::from_natural_prec_ref(&n, prec)),
            ),
        ],
    );
}

fn benchmark_float_from_natural_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_natural_prec(Natural, u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec)| {
                no_out!(Float::from_natural_prec(n, prec))
            }),
            ("rug", &mut |(n, prec)| {
                no_out!(rug::Float::with_val(
                    u32::exact_from(prec),
                    rug::Integer::from(&n),
                ))
            }),
        ],
    );
}

fn benchmark_float_from_natural_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_natural_prec_round(Natural, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_rounding_mode_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            (
                "Float::from_natural_prec(Natural, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_natural_prec_round(n, prec, rm)),
            ),
            (
                "Float::from_natural_prec_ref(&Natural, u64, RoundingMode)",
                &mut |(n, prec, rm)| no_out!(Float::from_natural_prec_round_ref(&n, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_from_natural_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_natural_prec_round(Natural, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_rounding_mode_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_bit_u64_max_bucketer("n", "prec"),
        &mut [
            ("Malachite", &mut |(n, prec, rm)| {
                no_out!(Float::from_natural_prec_round(n, prec, rm))
            }),
            ("rug", &mut |(n, prec, rm)| {
                no_out!(rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Integer::from(&n),
                    rug_round_try_from_rounding_mode(rm).unwrap()
                ))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_try_from_natural_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::try_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [
            ("Float::try_from(Natural)", &mut |x| {
                no_out!(Float::try_from(x))
            }),
            ("Float::try_from(&Natural)", &mut |x| {
                no_out!(Float::try_from(&x))
            }),
        ],
    );
}

fn benchmark_float_convertible_from_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::convertible_from(Natural)",
        BenchmarkType::Single,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(Float::convertible_from(&x)))],
    );
}
