// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::pair_1_primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_signed_pair_gen_var_1, primitive_float_signed_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_gen_var_1, rational_rounding_mode_pair_gen_var_4,
};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa_and_exponent_ref);
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa);
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa_ref);
    register_primitive_float_demos!(runner, demo_rational_sci_exponent);
    register_primitive_float_demos!(runner, demo_rational_sci_exponent_ref);
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa_and_exponent_round);
    register_primitive_float_demos!(runner, demo_rational_sci_mantissa_and_exponent_round_ref);
    register_primitive_float_demos!(runner, demo_rational_from_sci_mantissa_and_exponent);
    register_primitive_float_demos!(
        runner,
        demo_rational_from_sci_mantissa_and_exponent_targeted
    );
    register_primitive_float_benches!(
        runner,
        benchmark_rational_sci_mantissa_and_exponent_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_rational_sci_mantissa_and_exponent_round_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_rational_from_sci_mantissa_and_exponent);
}

fn demo_rational_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Rational: SciMantissaAndExponent<T, i64>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        let n_old = n.clone();
        let (mantissa, exponent) = n.sci_mantissa_and_exponent();
        println!(
            "sci_mantissa_and_exponent({}) = {:?}",
            n_old,
            (NiceFloat(mantissa), exponent)
        );
    }
}

fn demo_rational_sci_mantissa_and_exponent_ref<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        let (mantissa, exponent) = (&n).sci_mantissa_and_exponent();
        println!(
            "sci_mantissa_and_exponent(&{}) = {:?}",
            n,
            (NiceFloat(mantissa), exponent)
        );
    }
}

fn demo_rational_sci_mantissa<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Rational: SciMantissaAndExponent<T, i64>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("sci_mantissa({}) = {}", n_old, NiceFloat(n.sci_mantissa()));
    }
}

fn demo_rational_sci_mantissa_ref<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        println!("sci_mantissa({}) = {}", n, NiceFloat((&n).sci_mantissa()));
    }
}

fn demo_rational_sci_exponent<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Rational: SciMantissaAndExponent<T, i64>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("sci_exponent({}) = {}", n_old, n.sci_exponent());
    }
}

fn demo_rational_sci_exponent_ref<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    for n in rational_gen_var_1().get(gm, config).take(limit) {
        println!("sci_exponent({}) = {}", n, (&n).sci_exponent());
    }
}

fn demo_rational_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in rational_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "sci_mantissa_and_exponent_round({}, {}) = {:?}",
            n_old,
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    }
}

fn demo_rational_sci_mantissa_and_exponent_round_ref<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in rational_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "sci_mantissa_and_exponent_round({}, {}) = {:?}",
            n,
            rm,
            n.sci_mantissa_and_exponent_round_ref::<T>(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    }
}

fn demo_rational_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    for (m, e) in primitive_float_signed_pair_gen_var_1::<T, i64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Rational::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            NiceFloat(m),
            e,
            <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
        );
    }
}

fn demo_rational_from_sci_mantissa_and_exponent_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    for (m, e) in primitive_float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Rational::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            NiceFloat(m),
            e,
            <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
        );
    }
}

fn benchmark_rational_sci_mantissa_and_exponent_evaluation_strategy<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Rational: SciMantissaAndExponent<T, i64>,
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    run_benchmark(
        "Rational.sci_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        rational_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.sci_mantissa_and_exponent()", &mut |n| {
                no_out!(n.sci_mantissa_and_exponent())
            }),
            ("(&Rational).sci_mantissa_and_exponent()", &mut |n| {
                no_out!((&n).sci_mantissa_and_exponent())
            }),
        ],
    );
}

fn benchmark_rational_sci_mantissa_and_exponent_round_evaluation_strategy<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sci_mantissa_and_exponent_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [
            (
                "Rational.sci_mantissa_and_exponent_round(RoundingMode)",
                &mut |(n, rm)| no_out!(n.sci_mantissa_and_exponent_round::<T>(rm)),
            ),
            (
                "Rational.sci_mantissa_and_exponent_round_ref(RoundingMode)",
                &mut |(n, rm)| no_out!(n.sci_mantissa_and_exponent_round_ref::<T>(rm)),
            ),
        ],
    );
}

fn benchmark_rational_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    run_benchmark(
        &format!("Rational::from_sci_mantissa_and_exponent({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_1::<T, i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(m, e)| {
            no_out!(
                <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
                    m, e
                )
            )
        })],
    );
}
