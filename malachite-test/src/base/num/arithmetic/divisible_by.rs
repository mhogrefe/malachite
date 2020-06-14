use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_divisible_by);
    register_demo!(registry, demo_u16_divisible_by);
    register_demo!(registry, demo_u32_divisible_by);
    register_demo!(registry, demo_u64_divisible_by);
    register_demo!(registry, demo_usize_divisible_by);
    register_demo!(registry, demo_i8_divisible_by);
    register_demo!(registry, demo_i16_divisible_by);
    register_demo!(registry, demo_i32_divisible_by);
    register_demo!(registry, demo_i64_divisible_by);
    register_demo!(registry, demo_isize_divisible_by);

    register_bench!(registry, None, benchmark_u8_divisible_by);
    register_bench!(registry, None, benchmark_u16_divisible_by);
    register_bench!(registry, None, benchmark_u32_divisible_by);
    register_bench!(registry, None, benchmark_u64_divisible_by);
    register_bench!(registry, None, benchmark_usize_divisible_by);
    register_bench!(registry, None, benchmark_i8_divisible_by);
    register_bench!(registry, None, benchmark_i16_divisible_by);
    register_bench!(registry, None, benchmark_i32_divisible_by);
    register_bench!(registry, None, benchmark_i64_divisible_by);
    register_bench!(registry, None, benchmark_isize_divisible_by);
}

fn demo_unsigned_divisible_by<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        if x.divisible_by(y) {
            println!("{} is divisible by {}", x, y);
        } else {
            println!("{} is not divisible by {}", x, y);
        }
    }
}

fn demo_signed_divisible_by<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        if x.divisible_by(y) {
            println!("{} is divisible by {}", x, y);
        } else {
            println!("{} is not divisible by {}", x, y);
        }
    }
}

fn benchmark_unsigned_divisible_by<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.divisible_by({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.divisible_by(y))))],
    );
}

fn benchmark_signed_divisible_by<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.divisible_by({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.divisible_by(y))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_divisible_by::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_divisible_by::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_divisible_by::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_divisible_by::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_divisible_by, benchmark_u8_divisible_by);
unsigned!(u16, demo_u16_divisible_by, benchmark_u16_divisible_by);
unsigned!(u32, demo_u32_divisible_by, benchmark_u32_divisible_by);
unsigned!(u64, demo_u64_divisible_by, benchmark_u64_divisible_by);
unsigned!(usize, demo_usize_divisible_by, benchmark_usize_divisible_by);

signed!(i8, demo_i8_divisible_by, benchmark_i8_divisible_by);
signed!(i16, demo_i16_divisible_by, benchmark_i16_divisible_by);
signed!(i32, demo_i32_divisible_by, benchmark_i32_divisible_by);
signed!(i64, demo_i64_divisible_by, benchmark_i64_divisible_by);
signed!(isize, demo_isize_divisible_by, benchmark_isize_divisible_by);
