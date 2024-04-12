// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::mantissa_and_exponent::{
    from_sci_mantissa_and_exponent_round, sci_mantissa_and_exponent_round,
};
use malachite_base::num::conversion::traits::SciMantissaAndExponent;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer, triple_1_primitive_float_bucketer,
    unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen_var_12, primitive_float_signed_pair_gen_var_1,
    primitive_float_signed_pair_gen_var_2, primitive_float_unsigned_pair_gen_var_1,
    primitive_float_unsigned_pair_gen_var_2,
    primitive_float_unsigned_rounding_mode_triple_gen_var_1,
    primitive_float_unsigned_rounding_mode_triple_gen_var_2, unsigned_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_unsigned);
    register_unsigned_primitive_float_demos!(runner, demo_sci_mantissa_unsigned);
    register_unsigned_primitive_float_demos!(runner, demo_sci_exponent_unsigned);
    register_unsigned_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_round);
    register_unsigned_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent_unsigned);
    register_unsigned_primitive_float_demos!(
        runner,
        demo_from_sci_mantissa_and_exponent_targeted_unsigned
    );
    register_unsigned_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent_round);
    register_unsigned_primitive_float_demos!(
        runner,
        demo_from_sci_mantissa_and_exponent_round_targeted
    );

    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent_primitive_float);
    register_primitive_float_demos!(runner, demo_sci_mantissa_primitive_float);
    register_primitive_float_demos!(runner, demo_sci_exponent_primitive_float);
    register_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent_primitive_float);
    register_primitive_float_demos!(
        runner,
        demo_from_sci_mantissa_and_exponent_targeted_primitive_float
    );

    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_sci_mantissa_and_exponent_algorithms_unsigned
    );
    register_unsigned_primitive_float_benches!(runner, benchmark_sci_mantissa_algorithms_unsigned);
    register_unsigned_primitive_float_benches!(runner, benchmark_sci_exponent_algorithms_unsigned);
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_unsigned
    );
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_targeted_unsigned
    );
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_round
    );
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_round_targeted
    );

    register_primitive_float_benches!(
        runner,
        benchmark_sci_mantissa_and_exponent_algorithms_primitive_float
    );
    register_primitive_float_benches!(runner, benchmark_sci_mantissa_algorithms_primitive_float);
    register_primitive_float_benches!(runner, benchmark_sci_exponent_algorithms_primitive_float);
    register_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_primitive_float
    );
    register_primitive_float_benches!(
        runner,
        benchmark_from_sci_mantissa_and_exponent_targeted_primitive_float
    );
}

fn demo_sci_mantissa_and_exponent_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        let (m, e): (U, u64) = x.sci_mantissa_and_exponent();
        println!("sci_mantissa_and_exponent({}) = {:?}", x, (NiceFloat(m), e));
    }
}

fn demo_sci_mantissa_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        let m: U = x.sci_mantissa();
        println!("sci_mantissa({}) = {}", x, NiceFloat(m));
    }
}

fn demo_sci_exponent_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_1::<T>().get(gm, config).take(limit) {
        println!(
            "sci_exponent({}) = {}",
            x,
            SciMantissaAndExponent::<U, u64>::sci_exponent(x)
        );
    }
}

fn demo_sci_mantissa_and_exponent_round<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, rm) in unsigned_rounding_mode_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let o =
            sci_mantissa_and_exponent_round::<T, U>(x, rm).map(|(m, e, o)| (NiceFloat(m), e, o));
        println!("sci_mantissa_and_exponent_round({x}, {rm}) = {o:?}");
    }
}

fn demo_from_sci_mantissa_and_exponent_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_unsigned_pair_gen_var_1::<U, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            T::from_sci_mantissa_and_exponent(mantissa, exponent)
        );
    }
}

fn demo_from_sci_mantissa_and_exponent_targeted_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_unsigned_pair_gen_var_2::<U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            T::from_sci_mantissa_and_exponent(mantissa, exponent)
        );
    }
}

fn demo_from_sci_mantissa_and_exponent_round<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent, rm) in
        primitive_float_unsigned_rounding_mode_triple_gen_var_1::<U, u64>()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "from_sci_mantissa_and_exponent_round({}, {}, {}) = {:?}",
            NiceFloat(mantissa),
            exponent,
            rm,
            from_sci_mantissa_and_exponent_round::<T, U>(mantissa, exponent, rm)
        );
    }
}

fn demo_from_sci_mantissa_and_exponent_round_targeted<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent, rm) in primitive_float_unsigned_rounding_mode_triple_gen_var_2::<U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_sci_mantissa_and_exponent_round({}, {}, {}) = {:?}",
            NiceFloat(mantissa),
            exponent,
            rm,
            from_sci_mantissa_and_exponent_round::<T, U>(mantissa, exponent, rm)
        );
    }
}

fn demo_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        let (m, e) = x.sci_mantissa_and_exponent();
        println!(
            "sci_mantissa_and_exponent({}) = {:?}",
            NiceFloat(x),
            (NiceFloat(m), e)
        );
    }
}

fn demo_sci_mantissa_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "sci_mantissa({}) = {}",
            NiceFloat(x),
            NiceFloat(x.sci_mantissa())
        );
    }
}

fn demo_sci_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("sci_exponent({}) = {}", NiceFloat(x), x.sci_exponent());
    }
}

fn demo_from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            T::from_sci_mantissa_and_exponent(mantissa, exponent).map(NiceFloat)
        );
    }
}

fn demo_from_sci_mantissa_and_exponent_targeted_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            NiceFloat(T::from_sci_mantissa_and_exponent(mantissa, exponent).unwrap())
        );
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_and_exponent_algorithms_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| {
                no_out!(SciMantissaAndExponent::<U, u64>::sci_mantissa_and_exponent(
                    x
                ))
            }),
            ("alt", &mut |x| {
                no_out!((
                    SciMantissaAndExponent::<U, u64>::sci_mantissa(x),
                    SciMantissaAndExponent::<U, u64>::sci_exponent(x)
                ))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_algorithms_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| {
                no_out!(SciMantissaAndExponent::<U, u64>::sci_mantissa(x))
            }),
            ("alt", &mut |x| {
                no_out!(SciMantissaAndExponent::<U, u64>::sci_mantissa_and_exponent(x).0)
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_exponent_algorithms_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |x| {
                no_out!(SciMantissaAndExponent::<U, u64>::sci_exponent(x))
            }),
            ("alt", &mut |x| {
                no_out!(SciMantissaAndExponent::<U, u64>::sci_mantissa_and_exponent(x).1)
            }),
        ],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_sci_mantissa_and_exponent({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<U, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_targeted_unsigned<
    T: PrimitiveUnsigned + SciMantissaAndExponent<U, u64>,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_sci_mantissa_and_exponent({}, u64)",
            U::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_2::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_round<T: PrimitiveUnsigned, U: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "from_sci_mantissa_and_exponent_round({}, u64, RoundingMode)",
            U::NAME
        ),
        BenchmarkType::Single,
        primitive_float_unsigned_rounding_mode_triple_gen_var_1::<U, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent, rm)| {
            no_out!(from_sci_mantissa_and_exponent_round::<T, U>(
                mantissa, exponent, rm
            ))
        })],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_round_targeted<
    T: PrimitiveUnsigned,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "from_sci_mantissa_and_exponent_round({}, u64, RoundingMode)",
            U::NAME
        ),
        BenchmarkType::Single,
        primitive_float_unsigned_rounding_mode_triple_gen_var_2::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent, rm)| {
            no_out!(from_sci_mantissa_and_exponent_round::<T, U>(
                mantissa, exponent, rm
            ))
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_and_exponent_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_mantissa_and_exponent())),
            ("alt", &mut |x| {
                no_out!((x.sci_mantissa(), x.sci_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_mantissa())),
            ("alt", &mut |x| no_out!(x.sci_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_exponent_algorithms_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_exponent())),
            ("alt", &mut |x| no_out!(x.sci_mantissa_and_exponent().1)),
        ],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_sci_mantissa_and_exponent({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_targeted_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_sci_mantissa_and_exponent({}, u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}
