// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::float::arithmetic::dot::primitive_float_dot;
use malachite_float::test_util::bench::bucketers::{
    pair_2_vec_pair_float_sum_complexity_bucketer, quadruple_1_2_vec_float_sum_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::dot::rug_dot;
use malachite_float::test_util::generators::{
    float_vec_pair_gen_var_1, float_vec_pair_gen_var_1_rm, float_vec_pair_gen_var_2,
    float_vec_pair_rounding_mode_triple_gen_var_1, float_vec_pair_rounding_mode_triple_gen_var_2,
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1,
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_2,
    float_vec_pair_unsigned_triple_gen_var_1, primitive_float_vec_pair_gen_var_1,
};
use malachite_float::{ComparableFloat, Float};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_dot);
    register_demo!(runner, demo_float_dot_debug);
    register_demo!(runner, demo_float_dot_extreme);
    register_demo!(runner, demo_float_dot_extreme_debug);
    register_demo!(runner, demo_float_dot_prec);
    register_demo!(runner, demo_float_dot_prec_debug);
    register_demo!(runner, demo_float_dot_round);
    register_demo!(runner, demo_float_dot_round_debug);
    register_demo!(runner, demo_float_dot_round_extreme);
    register_demo!(runner, demo_float_dot_round_extreme_debug);
    register_demo!(runner, demo_float_dot_prec_round);
    register_demo!(runner, demo_float_dot_prec_round_debug);
    register_demo!(runner, demo_float_dot_prec_round_extreme);
    register_demo!(runner, demo_float_dot_prec_round_extreme_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_dot);

    register_bench!(runner, benchmark_float_dot_library_comparison);
    register_bench!(runner, benchmark_float_dot_prec_round_algorithms);
    register_primitive_float_benches!(runner, benchmark_primitive_float_dot);
}

fn debug_vec(xs: &[Float]) -> Vec<ComparableFloat> {
    xs.iter().map(|x| ComparableFloat(x.clone())).collect()
}

fn demo_float_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in float_vec_pair_gen_var_1().get(gm, config).take(limit) {
        println!("Float::dot(&{xs:?}, &{ys:?}) = {}", Float::dot(&xs, &ys));
    }
}

fn demo_float_dot_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in float_vec_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "Float::dot(&{:?}, &{:?}) = {:#x}",
            debug_vec(&xs),
            debug_vec(&ys),
            ComparableFloat(Float::dot(&xs, &ys))
        );
    }
}

fn demo_float_dot_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in float_vec_pair_gen_var_2().get(gm, config).take(limit) {
        println!("Float::dot(&{xs:?}, &{ys:?}) = {}", Float::dot(&xs, &ys));
    }
}

fn demo_float_dot_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in float_vec_pair_gen_var_2().get(gm, config).take(limit) {
        println!(
            "Float::dot(&{:?}, &{:?}) = {:#x}",
            debug_vec(&xs),
            debug_vec(&ys),
            ComparableFloat(Float::dot(&xs, &ys))
        );
    }
}

fn demo_float_dot_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec) in float_vec_pair_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec(&xs, &ys, prec);
        println!("Float::dot_prec(&{xs:?}, &{ys:?}, {prec}) = ({dot}, {o:?})");
    }
}

fn demo_float_dot_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec) in float_vec_pair_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec(&xs, &ys, prec);
        println!(
            "Float::dot_prec(&{:?}, &{:?}, {}) = ({:#x}, {:?})",
            debug_vec(&xs),
            debug_vec(&ys),
            prec,
            ComparableFloat(dot),
            o
        );
    }
}

fn demo_float_dot_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, rm) in float_vec_pair_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_round(&xs, &ys, rm);
        println!("Float::dot_round(&{xs:?}, &{ys:?}, {rm}) = ({dot}, {o:?})");
    }
}

fn demo_float_dot_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, rm) in float_vec_pair_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_round(&xs, &ys, rm);
        println!(
            "Float::dot_round(&{:?}, &{:?}, {}) = ({:#x}, {:?})",
            debug_vec(&xs),
            debug_vec(&ys),
            rm,
            ComparableFloat(dot),
            o
        );
    }
}

fn demo_float_dot_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, rm) in float_vec_pair_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_round(&xs, &ys, rm);
        println!("Float::dot_round(&{xs:?}, &{ys:?}, {rm}) = ({dot}, {o:?})");
    }
}

fn demo_float_dot_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, rm) in float_vec_pair_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_round(&xs, &ys, rm);
        println!(
            "Float::dot_round(&{:?}, &{:?}, {}) = ({:#x}, {:?})",
            debug_vec(&xs),
            debug_vec(&ys),
            rm,
            ComparableFloat(dot),
            o
        );
    }
}

fn demo_float_dot_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec, rm) in float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
        println!("Float::dot_prec_round(&{xs:?}, &{ys:?}, {prec}, {rm}) = ({dot}, {o:?})");
    }
}

fn demo_float_dot_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec, rm) in float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
        println!(
            "Float::dot_prec_round(&{:?}, &{:?}, {}, {}) = ({:#x}, {:?})",
            debug_vec(&xs),
            debug_vec(&ys),
            prec,
            rm,
            ComparableFloat(dot),
            o
        );
    }
}

fn demo_float_dot_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec, rm) in float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
        println!("Float::dot_prec_round(&{xs:?}, &{ys:?}, {prec}, {rm}) = ({dot}, {o:?})");
    }
}

fn demo_float_dot_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec, rm) in float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
        println!(
            "Float::dot_prec_round(&{:?}, &{:?}, {}, {}) = ({:#x}, {:?})",
            debug_vec(&xs),
            debug_vec(&ys),
            prec,
            rm,
            ComparableFloat(dot),
            o
        );
    }
}

fn benchmark_float_dot_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::dot(&[Float], &[Float])",
        BenchmarkType::LibraryComparison,
        float_vec_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_pair_float_sum_complexity_bucketer("xs", "ys"),
        &mut [
            ("Malachite", &mut |(_, (xs, ys))| {
                no_out!(Float::dot(&xs, &ys));
            }),
            ("rug", &mut |((xs, ys), _)| no_out!(rug_dot(&xs, &ys))),
        ],
    );
}

fn benchmark_float_dot_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::dot_prec_round(&[Float], &[Float], u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_float_sum_complexity_bucketer("xs", "ys"),
        &mut [
            ("default", &mut |(xs, ys, prec, rm)| {
                no_out!(Float::dot_prec_round(&xs, &ys, prec, rm));
            }),
            ("exact Rational route", &mut |(xs, ys, prec, rm)| {
                // The Rational route only applies to finite inputs; fall back for the rare
                // vectors containing specials.
                if xs.iter().chain(ys.iter()).all(Float::is_finite) {
                    let exact: Rational = xs
                        .iter()
                        .zip(ys.iter())
                        .map(|(x, y)| Rational::exact_from(x) * Rational::exact_from(y))
                        .sum();
                    no_out!(Float::from_rational_prec_round(exact, prec, rm));
                } else {
                    no_out!(Float::dot_prec_round(&xs, &ys, prec, rm));
                }
            }),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_dot<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    for (xs, ys) in primitive_float_vec_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_dot({:?}, {:?}) = {}",
            xs.iter().copied().map(NiceFloat).collect::<Vec<_>>(),
            ys.iter().copied().map(NiceFloat).collect::<Vec<_>>(),
            NiceFloat(primitive_float_dot(&xs, &ys))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_dot<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_dot(&[{}], &[{}])", T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_vec_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("malachite", &mut |(xs, ys)| {
            no_out!(primitive_float_dot(&xs, &ys));
        })],
    );
}
