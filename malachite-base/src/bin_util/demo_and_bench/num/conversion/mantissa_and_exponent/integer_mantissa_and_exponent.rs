// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bit_bucketer, primitive_float_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen_var_12, unsigned_gen_var_1, unsigned_pair_gen_var_2,
    unsigned_pair_gen_var_30, unsigned_signed_pair_gen_var_1, unsigned_signed_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_integer_mantissa_and_exponent_unsigned);
    register_unsigned_demos!(runner, demo_integer_mantissa_unsigned);
    register_unsigned_demos!(runner, demo_integer_exponent_unsigned);
    register_unsigned_demos!(runner, demo_from_integer_mantissa_and_exponent_unsigned);
    register_unsigned_demos!(
        runner,
        demo_from_integer_mantissa_and_exponent_targeted_unsigned
    );

    register_primitive_float_demos!(runner, demo_integer_mantissa_and_exponent_primitive_float);
    register_primitive_float_demos!(runner, demo_integer_mantissa_primitive_float);
    register_primitive_float_demos!(runner, demo_integer_exponent_primitive_float);
    register_primitive_float_demos!(
        runner,
        demo_from_integer_mantissa_and_exponent_primitive_float
    );
    register_primitive_float_demos!(
        runner,
        demo_from_integer_mantissa_and_exponent_targeted_primitive_float
    );

    register_unsigned_benches!(
        runner,
        benchmark_integer_mantissa_and_exponent_algorithms_unsigned
    );
    register_unsigned_benches!(runner, benchmark_integer_mantissa_algorithms_unsigned);
    register_unsigned_benches!(runner, benchmark_integer_exponent_algorithms_unsigned);
    register_unsigned_benches!(
        runner,
        benchmark_from_integer_mantissa_and_exponent_unsigned
    );
    register_unsigned_benches!(
        runner,
        benchmark_from_integer_mantissa_and_exponent_targeted_unsigned
    );

    register_primitive_float_benches!(
        runner,
        benchmark_integer_mantissa_and_exponent_algorithms_primitive_float
    );
    register_primitive_float_benches!(
        runner,
        benchmark_integer_mantissa_algorithms_primitive_float
    );
    register_primitive_float_benches!(
        runner,
        benchmark_integer_exponent_algorithms_primitive_float
    );
    register_primitive_float_benches!(
        runner,
        benchmark_from_integer_mantissa_and_exponent_primitive_float
    );
    register_primitive_float_benches!(
        runner,
        benchmark_from_integer_mantissa_and_exponent_targeted_primitive_float
    );
}

fn demo_integer_mantissa_and_exponent_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!(
            "integer_mantissa_and_exponent({}) = {:?}",
            x,
            x.integer_mantissa_and_exponent()
        );
    }
}

fn demo_integer_mantissa_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("integer_mantissa({}) = {}", x, x.integer_mantissa());
    }
}

fn demo_integer_exponent_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("integer_exponent({}) = {}", x, x.integer_exponent());
    }
}

fn demo_from_integer_mantissa_and_exponent_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_integer_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            mantissa,
            exponent,
            T::from_integer_mantissa_and_exponent(mantissa, exponent)
        );
    }
}

fn demo_from_integer_mantissa_and_exponent_targeted_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in unsigned_pair_gen_var_30::<T>().get(gm, config).take(limit) {
        println!(
            "{}::from_integer_mantissa_and_exponent({}, {}) = {}",
            T::NAME,
            mantissa,
            exponent,
            T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap()
        );
    }
}

fn demo_integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "integer_mantissa_and_exponent({}) = {:?}",
            NiceFloat(x),
            x.integer_mantissa_and_exponent()
        );
    }
}

fn demo_integer_mantissa_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "integer_mantissa({}) = {}",
            NiceFloat(x),
            x.integer_mantissa()
        );
    }
}

fn demo_integer_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "integer_exponent({}) = {}",
            NiceFloat(x),
            x.integer_exponent()
        );
    }
}

fn demo_from_integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in unsigned_signed_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "{}::from_integer_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            mantissa,
            exponent,
            T::from_integer_mantissa_and_exponent(mantissa, exponent).map(NiceFloat)
        );
    }
}

fn demo_from_integer_mantissa_and_exponent_targeted_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in unsigned_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_integer_mantissa_and_exponent({}, {}) = {}",
            T::NAME,
            mantissa,
            exponent,
            NiceFloat(T::from_integer_mantissa_and_exponent(mantissa, exponent).unwrap())
        );
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_and_exponent_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| {
                no_out!(x.integer_mantissa_and_exponent())
            }),
            ("alt", &mut |x| {
                no_out!((x.integer_mantissa(), x.integer_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| no_out!(x.integer_mantissa())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_exponent_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| no_out!(x.integer_exponent())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().1)),
        ],
    );
}

fn benchmark_from_integer_mantissa_and_exponent_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_integer_mantissa_and_exponent({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_integer_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_integer_mantissa_and_exponent_targeted_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_integer_mantissa_and_exponent({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_30::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_integer_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_and_exponent_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| {
                no_out!(x.integer_mantissa_and_exponent())
            }),
            ("alt", &mut |x| {
                no_out!((x.integer_mantissa(), x.integer_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.integer_mantissa())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_exponent_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.integer_exponent())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().1)),
        ],
    );
}

fn benchmark_from_integer_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_integer_mantissa_and_exponent(u64, u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_signed_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_integer_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_integer_mantissa_and_exponent_targeted_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_integer_mantissa_and_exponent(u64, u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_integer_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}
