// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bucketer, pair_2_bucketer, triple_2_bucketer, unsigned_direct_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_gen_var_31, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::constants::liouvilles_constant::*;
use malachite_float::test_util::float::constants::digit_constants::*;
use malachite_float::test_util::generators::{
    unsigned_pair_gen_var_51, unsigned_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_liouvilles_constant_base_prec_round);
    register_demo!(runner, demo_float_liouvilles_constant_base_prec_round_debug);
    register_demo!(runner, demo_float_liouvilles_constant_base_prec);
    register_demo!(runner, demo_float_liouvilles_constant_base_prec_debug);
    register_demo!(runner, demo_float_liouvilles_constant_prec_round);
    register_demo!(runner, demo_float_liouvilles_constant_prec_round_debug);
    register_demo!(runner, demo_float_liouvilles_constant_prec);
    register_demo!(runner, demo_float_liouvilles_constant_prec_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_liouvilles_constant_base);

    register_bench!(
        runner,
        benchmark_float_liouvilles_constant_base_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_float_liouvilles_constant_base_prec_algorithms
    );
    register_bench!(
        runner,
        benchmark_float_liouvilles_constant_prec_round_algorithms
    );
    register_bench!(runner, benchmark_float_liouvilles_constant_prec_algorithms);
    register_primitive_float_benches!(runner, benchmark_primitive_float_liouvilles_constant_base);
}

fn demo_float_liouvilles_constant_base_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (b, p, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "liouvilles_constant_base_prec_round({}, {}, {}) = {:?}",
            b,
            p,
            rm,
            Float::liouvilles_constant_base_prec_round(b, p, rm)
        );
    }
}

fn demo_float_liouvilles_constant_base_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (b, p, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = Float::liouvilles_constant_base_prec_round(b, p, rm);
        println!(
            "liouvilles_constant_base_prec_round({}, {}, {}) = ({:#x}, {:?})",
            b,
            p,
            rm,
            ComparableFloat(x),
            o
        );
    }
}

fn demo_float_liouvilles_constant_base_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (b, p) in unsigned_pair_gen_var_51().get(gm, config).take(limit) {
        println!(
            "liouvilles_constant_base_prec({}, {}) = {:?}",
            b,
            p,
            Float::liouvilles_constant_base_prec(b, p)
        );
    }
}

fn demo_float_liouvilles_constant_base_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (b, p) in unsigned_pair_gen_var_51().get(gm, config).take(limit) {
        let (x, o) = Float::liouvilles_constant_base_prec(b, p);
        println!(
            "liouvilles_constant_base_prec({}, {}) = ({:#x}, {:?})",
            b,
            p,
            ComparableFloat(x),
            o
        );
    }
}

fn benchmark_float_liouvilles_constant_base_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::liouvilles_constant_base_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        unsigned_unsigned_rounding_mode_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("prec"),
        &mut [
            ("default", &mut |(b, p, rm)| {
                no_out!(Float::liouvilles_constant_base_prec_round(b, p, rm));
            }),
            ("naive", &mut |(b, p, rm)| {
                no_out!(liouvilles_constant_base_prec_round_naive(b, p, rm));
            }),
        ],
    );
}

fn benchmark_float_liouvilles_constant_base_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::liouvilles_constant_base_prec(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_51().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("prec"),
        &mut [
            ("default", &mut |(b, p)| {
                no_out!(Float::liouvilles_constant_base_prec(b, p));
            }),
            ("naive", &mut |(b, p)| {
                no_out!(liouvilles_constant_base_prec_round_naive(b, p, Nearest));
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_liouvilles_constant_base<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for base in unsigned_gen_var_31::<u64>().get(gm, config).take(limit) {
        println!(
            "primitive_float_liouvilles_constant_base::<{}>({}) = {}",
            T::NAME,
            base,
            NiceFloat(primitive_float_liouvilles_constant_base::<T>(base))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_liouvilles_constant_base<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!(
            "primitive_float_liouvilles_constant_base::<{}>(u64)",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_gen_var_31::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("malachite", &mut |base| {
            no_out!(primitive_float_liouvilles_constant_base::<T>(base));
        })],
    );
}

fn demo_float_liouvilles_constant_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "liouvilles_constant_prec_round({}, {}) = {:?}",
            p,
            rm,
            Float::liouvilles_constant_prec_round(p, rm)
        );
    }
}

fn demo_float_liouvilles_constant_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = Float::liouvilles_constant_prec_round(p, rm);
        println!(
            "liouvilles_constant_prec_round({}, {}) = ({:#x}, {:?})",
            p,
            rm,
            ComparableFloat(x),
            o
        );
    }
}

fn demo_float_liouvilles_constant_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "liouvilles_constant_prec({}) = {:?}",
            p,
            Float::liouvilles_constant_prec(p)
        );
    }
}

fn demo_float_liouvilles_constant_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        let (x, o) = Float::liouvilles_constant_prec(p);
        println!(
            "liouvilles_constant_prec({}) = ({:#x}, {:?})",
            p,
            ComparableFloat(x),
            o
        );
    }
}

fn benchmark_float_liouvilles_constant_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::liouvilles_constant_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        unsigned_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bucketer("prec"),
        &mut [
            ("default", &mut |(p, rm)| {
                no_out!(Float::liouvilles_constant_prec_round(p, rm));
            }),
            ("naive", &mut |(p, rm)| {
                no_out!(liouvilles_constant_base_prec_round_naive(10, p, rm));
            }),
        ],
    );
}

fn benchmark_float_liouvilles_constant_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::liouvilles_constant_prec(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |p| {
                no_out!(Float::liouvilles_constant_prec(p));
            }),
            ("naive", &mut |p| {
                no_out!(liouvilles_constant_base_prec_round_naive(10, p, Nearest));
            }),
        ],
    );
}
