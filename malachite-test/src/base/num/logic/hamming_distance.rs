use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_hamming_distance);
    register_demo!(registry, demo_u16_hamming_distance);
    register_demo!(registry, demo_u32_hamming_distance);
    register_demo!(registry, demo_u64_hamming_distance);
    register_demo!(registry, demo_usize_hamming_distance);
    register_demo!(registry, demo_i8_checked_hamming_distance);
    register_demo!(registry, demo_i16_checked_hamming_distance);
    register_demo!(registry, demo_i32_checked_hamming_distance);
    register_demo!(registry, demo_i64_checked_hamming_distance);
    register_demo!(registry, demo_isize_checked_hamming_distance);
    register_bench!(registry, None, benchmark_u8_hamming_distance);
    register_bench!(registry, None, benchmark_u16_hamming_distance);
    register_bench!(registry, None, benchmark_u32_hamming_distance);
    register_bench!(registry, None, benchmark_u64_hamming_distance);
    register_bench!(registry, None, benchmark_usize_hamming_distance);
    register_bench!(registry, None, benchmark_i8_checked_hamming_distance);
    register_bench!(registry, None, benchmark_i16_checked_hamming_distance);
    register_bench!(registry, None, benchmark_i32_checked_hamming_distance);
    register_bench!(registry, None, benchmark_i64_checked_hamming_distance);
    register_bench!(registry, None, benchmark_isize_checked_hamming_distance);
}

fn demo_unsigned_checked_hamming_distance<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (u, v) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.hamming_distance({}) = {}", u, v, u.hamming_distance(v));
    }
}

fn demo_signed_checked_hamming_distance<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (i, j) in pairs_of_signeds::<T>(gm).take(limit) {
        println!(
            "{}.checked_hamming_distance({}) = {:?}",
            i,
            j,
            i.checked_hamming_distance(j)
        );
    }
}

fn benchmark_unsigned_checked_hamming_distance<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.hamming_distance({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(u, v)| usize::exact_from(max(u.significant_bits(), v.significant_bits()))),
        "max(u.significant_bits(), v.significant_bits())",
        &mut [("malachite", &mut (|(u, v)| no_out!(u.hamming_distance(v))))],
    );
}

fn benchmark_signed_checked_hamming_distance<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.checked_hamming_distance({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(i, j)| usize::exact_from(max(i.significant_bits(), j.significant_bits()))),
        "max(i.significant_bits(), j.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(i, j)| no_out!(i.checked_hamming_distance(j))),
        )],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_checked_hamming_distance::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_checked_hamming_distance::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_checked_hamming_distance::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_checked_hamming_distance::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_hamming_distance, benchmark_u8_hamming_distance);
unsigned!(
    u16,
    demo_u16_hamming_distance,
    benchmark_u16_hamming_distance
);
unsigned!(
    u32,
    demo_u32_hamming_distance,
    benchmark_u32_hamming_distance
);
unsigned!(
    u64,
    demo_u64_hamming_distance,
    benchmark_u64_hamming_distance
);
unsigned!(
    usize,
    demo_usize_hamming_distance,
    benchmark_usize_hamming_distance
);

signed!(
    i8,
    demo_i8_checked_hamming_distance,
    benchmark_i8_checked_hamming_distance
);
signed!(
    i16,
    demo_i16_checked_hamming_distance,
    benchmark_i16_checked_hamming_distance
);
signed!(
    i32,
    demo_i32_checked_hamming_distance,
    benchmark_i32_checked_hamming_distance
);
signed!(
    i64,
    demo_i64_checked_hamming_distance,
    benchmark_i64_checked_hamming_distance
);
signed!(
    isize,
    demo_isize_checked_hamming_distance,
    benchmark_isize_checked_hamming_distance
);
