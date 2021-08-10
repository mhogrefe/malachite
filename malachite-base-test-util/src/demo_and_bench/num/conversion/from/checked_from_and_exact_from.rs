use malachite_base::named::Named;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_13, primitive_float_gen_var_14, signed_gen,
    signed_gen_var_2, signed_gen_var_7, unsigned_gen, unsigned_gen_var_18,
};
use malachite_base_test_util::runner::Runner;
use std::fmt::{Debug, Display};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_checked_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_checked_from_signed);
    register_primitive_int_primitive_float_demos!(
        runner,
        demo_primitive_int_checked_from_primitive_float
    );
    register_unsigned_primitive_float_demos!(runner, demo_primitive_float_checked_from_unsigned);
    register_signed_primitive_float_demos!(runner, demo_primitive_float_checked_from_signed);

    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_exact_from_signed);
    register_unsigned_primitive_float_demos!(runner, demo_unsigned_exact_from_primitive_float);
    register_signed_primitive_float_demos!(runner, demo_signed_exact_from_primitive_float);
    register_primitive_float_unsigned_demos!(runner, demo_primitive_float_exact_from_unsigned);
    register_primitive_float_signed_demos!(runner, demo_primitive_float_exact_from_signed);

    register_primitive_int_unsigned_benches!(runner, benchmark_primitive_int_checked_from_unsigned);
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_checked_from_signed);
    register_primitive_int_primitive_float_benches!(
        runner,
        benchmark_primitive_int_checked_from_primitive_float
    );
    register_primitive_float_unsigned_benches!(
        runner,
        benchmark_primitive_float_checked_from_unsigned
    );
    register_primitive_float_signed_benches!(runner, benchmark_primitive_float_checked_from_signed);

    register_primitive_int_unsigned_benches!(runner, benchmark_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_exact_from_signed);
    register_unsigned_primitive_float_benches!(
        runner,
        benchmark_unsigned_exact_from_primitive_float
    );
    register_signed_primitive_float_benches!(runner, benchmark_signed_exact_from_primitive_float);
    register_primitive_float_unsigned_benches!(
        runner,
        benchmark_primitive_float_exact_from_unsigned
    );
    register_primitive_float_signed_benches!(runner, benchmark_primitive_float_exact_from_signed);
}

fn demo_primitive_int_checked_from_unsigned<
    T: CheckedFrom<U> + Debug + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            T::NAME,
            u,
            T::checked_from(u)
        );
    }
}

fn demo_primitive_int_checked_from_signed<T: CheckedFrom<U> + Debug + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for i in signed_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            T::NAME,
            i,
            T::checked_from(i)
        );
    }
}

fn demo_primitive_int_checked_from_primitive_float<
    T: CheckedFrom<U> + Debug + Named,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in primitive_float_gen::<U>().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            T::NAME,
            NiceFloat(u),
            T::checked_from(u)
        );
    }
}

fn demo_primitive_float_checked_from_unsigned<
    T: PrimitiveUnsigned,
    U: CheckedFrom<T> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            U::NAME,
            u,
            U::checked_from(u)
        );
    }
}

fn demo_primitive_float_checked_from_signed<
    T: PrimitiveSigned,
    U: CheckedFrom<T> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in signed_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            U::NAME,
            u,
            U::checked_from(u)
        );
    }
}

fn demo_primitive_int_exact_from_unsigned<
    T: CheckedFrom<U> + Display + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<U>().get(gm, &config).take(limit) {
        println!("{}::exact_from({}) = {}", T::NAME, u, T::exact_from(u));
    }
}

fn demo_primitive_int_exact_from_signed<T: CheckedFrom<U> + Display + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for i in signed_gen_var_2::<U>().get(gm, &config).take(limit) {
        println!("{}::exact_from({}) = {}", T::NAME, i, T::exact_from(i));
    }
}

fn demo_unsigned_exact_from_primitive_float<
    T: CheckedFrom<U> + PrimitiveUnsigned,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in primitive_float_gen_var_13::<U, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            NiceFloat(u),
            T::exact_from(u)
        );
    }
}

fn demo_signed_exact_from_primitive_float<
    T: CheckedFrom<U> + PrimitiveSigned + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in primitive_float_gen_var_14::<U, T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            NiceFloat(u),
            T::exact_from(u)
        );
    }
}

fn demo_primitive_float_exact_from_unsigned<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen_var_18::<U, T>().get(gm, &config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            u,
            NiceFloat(T::exact_from(u))
        );
    }
}

fn demo_primitive_float_exact_from_signed<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for i in signed_gen_var_7::<U, T>().get(gm, &config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            i,
            NiceFloat(T::exact_from(i))
        );
    }
}

fn benchmark_primitive_int_checked_from_unsigned<
    T: CheckedFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(n)))],
    );
}

fn benchmark_primitive_int_checked_from_signed<T: CheckedFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(n)))],
    );
}

fn benchmark_primitive_int_checked_from_primitive_float<
    T: CheckedFrom<U> + Named,
    U: PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(n)))],
    );
}

fn benchmark_primitive_float_checked_from_unsigned<
    T: CheckedFrom<U> + Named,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(n)))],
    );
}

fn benchmark_primitive_float_checked_from_signed<T: CheckedFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(n)))],
    );
}

fn benchmark_primitive_int_exact_from_unsigned<T: CheckedFrom<U> + Named, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_primitive_int_exact_from_signed<T: CheckedFrom<U> + Named, U: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_unsigned_exact_from_primitive_float<
    T: CheckedFrom<U> + PrimitiveUnsigned,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_13::<U, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_signed_exact_from_primitive_float<
    T: CheckedFrom<U> + PrimitiveSigned,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_14::<U, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_primitive_float_exact_from_unsigned<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveUnsigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_18::<U, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}

fn benchmark_primitive_float_exact_from_signed<
    T: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: PrimitiveSigned + RoundingFrom<T>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.exact_from({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_gen_var_7::<U, T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(n)))],
    );
}
