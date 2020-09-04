use std::fmt::{Debug, Display};

use malachite_base::named::Named;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};

use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, signed_gen_var_2, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_checked_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_checked_from_signed);
    register_primitive_int_unsigned_demos!(runner, demo_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_demos!(runner, demo_primitive_int_exact_from_signed);

    register_primitive_int_unsigned_benches!(runner, benchmark_primitive_int_checked_from_unsigned);
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_checked_from_signed);
    register_primitive_int_unsigned_benches!(runner, benchmark_primitive_int_exact_from_unsigned);
    register_primitive_int_signed_benches!(runner, benchmark_primitive_int_exact_from_signed);
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
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(T::checked_from(n))))],
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
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(T::checked_from(n))))],
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
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(T::exact_from(n))))],
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
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(T::exact_from(n))))],
    );
}
