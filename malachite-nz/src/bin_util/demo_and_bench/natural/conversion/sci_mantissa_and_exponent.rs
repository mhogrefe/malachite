// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, triple_1_primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_unsigned_pair_gen_var_1, primitive_float_unsigned_pair_gen_var_2,
    primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    primitive_float_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_gen_var_2, natural_rounding_mode_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_natural_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_natural_sci_mantissa);
    register_primitive_float_demos!(runner, demo_natural_sci_exponent);
    register_primitive_float_demos!(runner, demo_natural_sci_mantissa_and_exponent_round);
    register_primitive_float_demos!(runner, demo_natural_from_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_natural_from_sci_mantissa_and_exponent_targeted);
    register_primitive_float_demos!(runner, demo_natural_from_sci_mantissa_and_exponent_round);
    register_primitive_float_demos!(
        runner,
        demo_natural_from_sci_mantissa_and_exponent_round_targeted
    );
    register_primitive_float_benches!(runner, benchmark_natural_sci_mantissa_and_exponent);
    register_primitive_float_benches!(runner, benchmark_natural_sci_mantissa_and_exponent_round);
    register_primitive_float_benches!(runner, benchmark_natural_from_sci_mantissa_and_exponent);
    register_primitive_float_benches!(
        runner,
        benchmark_natural_from_sci_mantissa_and_exponent_round
    );
}

fn demo_natural_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        let (mantissa, exponent) = n.sci_mantissa_and_exponent();
        println!(
            "sci_mantissa_and_exponent({}) = {:?}",
            n,
            (NiceFloat(mantissa), exponent)
        );
    }
}

fn demo_natural_sci_mantissa<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("sci_mantissa({}) = {}", n, NiceFloat(n.sci_mantissa()));
    }
}

fn demo_natural_sci_exponent<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    for n in natural_gen_var_2().get(gm, config).take(limit) {
        println!("sci_exponent({}) = {}", n, n.sci_exponent());
    }
}

fn demo_natural_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, rm) in natural_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "sci_mantissa_and_exponent_round({}, {}) = {:?}",
            n,
            rm,
            n.sci_mantissa_and_exponent_round::<T>(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    }
}

fn demo_natural_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    for (m, e) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            NiceFloat(m),
            e,
            <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
        );
    }
}

fn demo_natural_from_sci_mantissa_and_exponent_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    for (m, e) in primitive_float_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            NiceFloat(m),
            e,
            <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
        );
    }
}

fn demo_natural_from_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (m, e, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_mantissa_and_exponent_round({}, {}, {}) = {:?}",
            NiceFloat(m),
            e,
            rm,
            Natural::from_sci_mantissa_and_exponent_round(m, e, rm)
        );
    }
}

fn demo_natural_from_sci_mantissa_and_exponent_round_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (m, e, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_sci_mantissa_and_exponent_round({}, {}, {}) = {:?}",
            NiceFloat(m),
            e,
            rm,
            Natural::from_sci_mantissa_and_exponent_round(m, e, rm)
        );
    }
}

fn benchmark_natural_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    run_benchmark(
        "Natural.sci_mantissa_and_exponent()",
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.sci_mantissa_and_exponent()))],
    );
}

fn benchmark_natural_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.sci_mantissa_and_exponent_round(RoundingMode)",
        BenchmarkType::Single,
        natural_rounding_mode_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, rm)| {
            no_out!(n.sci_mantissa_and_exponent_round::<T>(rm))
        })],
    );
}

fn benchmark_natural_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    run_benchmark(
        &format!("Natural::from_sci_mantissa_and_exponent({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(m, e)| {
            no_out!(
                <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
            )
        })],
    );
}

fn benchmark_natural_from_sci_mantissa_and_exponent_round<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "Natural::from_sci_mantissa_and_exponent_round({}, u64, RoundingMode)",
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_unsigned_rounding_mode_triple_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(m, e, rm)| {
            no_out!(Natural::from_sci_mantissa_and_exponent_round(m, e, rm))
        })],
    );
}
