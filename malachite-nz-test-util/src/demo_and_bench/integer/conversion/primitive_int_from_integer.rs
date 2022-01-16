use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::bench::bucketers::{integer_bit_bucketer, pair_2_integer_bit_bucketer};
use malachite_nz_test_util::generators::{
    integer_gen, integer_gen_rm, integer_gen_var_5, integer_gen_var_6,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_primitive_int_checked_from_integer);
    register_unsigned_demos!(runner, demo_unsigned_exact_from_integer);
    register_signed_demos!(runner, demo_signed_exact_from_integer);
    register_primitive_int_demos!(runner, demo_primitive_int_wrapping_from_integer);
    register_primitive_int_demos!(runner, demo_primitive_int_saturating_from_integer);
    register_primitive_int_demos!(runner, demo_primitive_int_overflowing_from_integer);
    register_primitive_int_demos!(runner, demo_primitive_int_convertible_from_integer);

    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_checked_from_integer_algorithms
    );
    register_unsigned_benches!(runner, benchmark_unsigned_exact_from_integer);
    register_signed_benches!(runner, benchmark_signed_exact_from_integer);
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_wrapping_from_integer_algorithms
    );
    register_primitive_int_benches!(runner, benchmark_primitive_int_saturating_from_integer);
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_overflowing_from_integer_algorithms
    );
    register_primitive_int_benches!(
        runner,
        benchmark_primitive_int_convertible_from_integer_algorithms
    );
    register_bench!(
        runner,
        benchmark_u32_checked_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_u32_wrapping_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_u64_checked_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_u64_wrapping_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_i32_checked_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_i32_wrapping_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_i64_checked_from_integer_library_comparison
    );
    register_bench!(
        runner,
        benchmark_i64_wrapping_from_integer_library_comparison
    );
}

fn demo_primitive_int_checked_from_integer<T: for<'a> CheckedFrom<&'a Integer> + PrimitiveInt>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from(&{}) = {:?}",
            T::NAME,
            n,
            T::checked_from(&n)
        );
    }
}

fn demo_unsigned_exact_from_integer<T: for<'a> CheckedFrom<&'a Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Integer: From<T>,
{
    for n in integer_gen_var_5::<T>().get(gm, &config).take(limit) {
        println!("{}::exact_from(&{}) = {}", T::NAME, n, T::exact_from(&n));
    }
}

fn demo_signed_exact_from_integer<T: for<'a> CheckedFrom<&'a Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Integer: From<T>,
{
    for n in integer_gen_var_6::<T>().get(gm, &config).take(limit) {
        println!("{}::exact_from(&{}) = {}", T::NAME, n, T::exact_from(&n));
    }
}

fn demo_primitive_int_wrapping_from_integer<T: PrimitiveInt + for<'a> WrappingFrom<&'a Integer>>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!(
            "{}::wrapping_from(&{}) = {}",
            T::NAME,
            n,
            T::wrapping_from(&n)
        );
    }
}

fn demo_primitive_int_saturating_from_integer<
    T: PrimitiveInt + for<'a> SaturatingFrom<&'a Integer>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!(
            "{}::saturating_from(&{}) = {}",
            T::NAME,
            n,
            T::saturating_from(&n)
        );
    }
}

fn demo_primitive_int_overflowing_from_integer<
    T: for<'a> OverflowingFrom<&'a Integer> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!(
            "{}::overflowing_from(&{}) = {:?}",
            T::NAME,
            n,
            T::overflowing_from(&n)
        );
    }
}

fn demo_primitive_int_convertible_from_integer<
    T: for<'a> ConvertibleFrom<&'a Integer> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!(
            "{} is {}convertible to a {}",
            n,
            if T::convertible_from(&n) { "" } else { "not " },
            T::NAME,
        );
    }
}

fn benchmark_primitive_int_checked_from_integer_algorithms<
    T: for<'a> CheckedFrom<&'a Integer> + for<'a> OverflowingFrom<&'a Integer> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_from(&Integer)", T::NAME),
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |n| no_out!(T::checked_from(&n))),
            ("using overflowing_from", &mut |n| {
                let (value, overflow) = T::overflowing_from(&n);
                if overflow {
                    None
                } else {
                    Some(value)
                };
            }),
        ],
    );
}

fn benchmark_unsigned_exact_from_integer<T: for<'a> CheckedFrom<&'a Integer> + PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("{}::exact_from(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

fn benchmark_signed_exact_from_integer<T: for<'a> CheckedFrom<&'a Integer> + PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("{}::exact_from(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_gen_var_6::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_primitive_int_wrapping_from_integer_algorithms<
    T: for<'a> OverflowingFrom<&'a Integer> + PrimitiveInt + for<'a> WrappingFrom<&'a Integer>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::wrapping_from(&Integer)", T::NAME),
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |n| no_out!(T::wrapping_from(&n))),
            ("using overflowing_from", &mut |n| {
                T::overflowing_from(&n).0;
            }),
        ],
    );
}

fn benchmark_primitive_int_saturating_from_integer<
    T: PrimitiveInt + for<'a> SaturatingFrom<&'a Integer>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::saturating_from(&Integer)", T::NAME),
        BenchmarkType::Single,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |n| no_out!(T::saturating_from(&n)))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_primitive_int_overflowing_from_integer_algorithms<
    T: for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> OverflowingFrom<&'a Integer>
        + PrimitiveInt
        + for<'a> WrappingFrom<&'a Integer>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::overflowing_from(&Integer)", T::NAME),
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |n| no_out!(T::overflowing_from(&n))),
            ("using wrapping_from and convertible_from", &mut |n| {
                no_out!((T::wrapping_from(&n), !T::convertible_from(&n)))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_primitive_int_convertible_from_integer_algorithms<
    T: for<'a> CheckedFrom<&'a Integer> + for<'a> ConvertibleFrom<&'a Integer> + PrimitiveInt,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(&Integer)", T::NAME),
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |n| no_out!(T::convertible_from(&n))),
            ("using checked_from", &mut |n| {
                no_out!(T::checked_from(&n).is_some())
            }),
        ],
    );
}

fn benchmark_u32_checked_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u32::checked_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u32())),
        ],
    );
}

fn benchmark_u32_wrapping_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u32::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u32_wrapping())),
        ],
    );
}

fn benchmark_u64_checked_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u64::checked_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u64())),
        ],
    );
}

fn benchmark_u64_wrapping_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(u64::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_u64_wrapping())),
        ],
    );
}

fn benchmark_i32_checked_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i32::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i32::checked_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i32())),
        ],
    );
}

fn benchmark_i32_wrapping_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i32::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i32::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i32_wrapping())),
        ],
    );
}

fn benchmark_i64_checked_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i64::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i64::checked_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i64())),
        ],
    );
}

fn benchmark_i64_wrapping_from_integer_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "i64::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, n)| no_out!(i64::wrapping_from(&n))),
            ("rug", &mut |(n, _)| no_out!(n.to_i64_wrapping())),
        ],
    );
}
