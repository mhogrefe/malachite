use malachite_base::named::Named;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::integer::Integer;
use num::BigInt;
use rand::Rand;
use rug;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{signeds, unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_u8);
    register_demo!(registry, demo_integer_from_u16);
    register_demo!(registry, demo_integer_from_u32);
    register_demo!(registry, demo_integer_from_u64);
    register_demo!(registry, demo_integer_from_usize);
    register_demo!(registry, demo_integer_from_i8);
    register_demo!(registry, demo_integer_from_i16);
    register_demo!(registry, demo_integer_from_i32);
    register_demo!(registry, demo_integer_from_i64);
    register_demo!(registry, demo_integer_from_isize);
    register_bench!(registry, None, benchmark_integer_from_u8);
    register_bench!(registry, None, benchmark_integer_from_u16);
    register_bench!(registry, None, benchmark_integer_from_u32);
    register_bench!(registry, None, benchmark_integer_from_u64);
    register_bench!(registry, None, benchmark_integer_from_usize);
    register_bench!(registry, None, benchmark_integer_from_i8);
    register_bench!(registry, None, benchmark_integer_from_i16);
    register_bench!(registry, None, benchmark_integer_from_i32);
    register_bench!(registry, None, benchmark_integer_from_i64);
    register_bench!(registry, None, benchmark_integer_from_isize);
    register_bench!(
        registry,
        None,
        benchmark_integer_from_u32_library_comparison
    );
    register_bench!(
        registry,
        None,
        benchmark_integer_from_u64_library_comparison
    );
    register_bench!(
        registry,
        None,
        benchmark_integer_from_i32_library_comparison
    );
    register_bench!(
        registry,
        None,
        benchmark_integer_from_i64_library_comparison
    );
}

fn demo_integer_from_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: From<T>,
{
    for u in unsigneds::<T>(gm).take(limit) {
        println!("Integer::from({}) = {}", u, Integer::from(u));
    }
}

fn benchmark_integer_from_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("Integer::from({})", T::NAME),
        BenchmarkType::Single,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [("malachite", &mut (|u| no_out!(Integer::from(u))))],
    );
}

macro_rules! demo_and_bench_unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_from_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_from_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! demo_and_bench_signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            for i in signeds::<$t>(gm).take(limit) {
                println!("Integer::from({}) = {}", i, Integer::from(i));
            }
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark(
                &format!("Integer::from({})", $t::NAME),
                BenchmarkType::Single,
                signeds::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&i| usize::exact_from(i.significant_bits())),
                "i.significant_bits()",
                &mut [("malachite", &mut (|i| no_out!(Integer::from(i))))],
            );
        }
    };
}

demo_and_bench_unsigned!(u8, demo_integer_from_u8, benchmark_integer_from_u8);
demo_and_bench_unsigned!(u16, demo_integer_from_u16, benchmark_integer_from_u16);
demo_and_bench_unsigned!(u32, demo_integer_from_u32, benchmark_integer_from_u32);
demo_and_bench_unsigned!(u64, demo_integer_from_u64, benchmark_integer_from_u64);
demo_and_bench_unsigned!(usize, demo_integer_from_usize, benchmark_integer_from_usize);

demo_and_bench_signed!(i8, demo_integer_from_i8, benchmark_integer_from_i8);
demo_and_bench_signed!(i16, demo_integer_from_i16, benchmark_integer_from_i16);
demo_and_bench_signed!(i32, demo_integer_from_i32, benchmark_integer_from_i32);
demo_and_bench_signed!(i64, demo_integer_from_i64, benchmark_integer_from_i64);
demo_and_bench_signed!(isize, demo_integer_from_isize, benchmark_integer_from_isize);

fn benchmark_integer_from_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Integer::from(u)))),
            ("num", &mut (|u| no_out!(BigInt::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}

fn benchmark_integer_from_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::exact_from(u.significant_bits())),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Integer::from(u)))),
            ("num", &mut (|u| no_out!(BigInt::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}

fn benchmark_integer_from_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(i32)",
        BenchmarkType::LibraryComparison,
        signeds::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
            ("rug", &mut (|i| no_out!(rug::Integer::from(i)))),
        ],
    );
}

fn benchmark_integer_from_i64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from(i32)",
        BenchmarkType::LibraryComparison,
        signeds::<i64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
            ("rug", &mut (|i| no_out!(rug::Integer::from(i)))),
        ],
    );
}
