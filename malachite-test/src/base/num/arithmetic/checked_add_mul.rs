use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{triples_of_signeds, triples_of_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_checked_add_mul);
    register_demo!(registry, demo_u16_checked_add_mul);
    register_demo!(registry, demo_u32_checked_add_mul);
    register_demo!(registry, demo_u64_checked_add_mul);
    register_demo!(registry, demo_usize_checked_add_mul);
    register_demo!(registry, demo_i8_checked_add_mul);
    register_demo!(registry, demo_i16_checked_add_mul);
    register_demo!(registry, demo_i32_checked_add_mul);
    register_demo!(registry, demo_i64_checked_add_mul);
    register_demo!(registry, demo_isize_checked_add_mul);

    register_bench!(registry, None, benchmark_u8_checked_add_mul);
    register_bench!(registry, None, benchmark_u16_checked_add_mul);
    register_bench!(registry, None, benchmark_u32_checked_add_mul);
    register_bench!(registry, None, benchmark_u64_checked_add_mul);
    register_bench!(registry, None, benchmark_usize_checked_add_mul);
    register_bench!(registry, None, benchmark_i8_checked_add_mul);
    register_bench!(registry, None, benchmark_i16_checked_add_mul);
    register_bench!(registry, None, benchmark_i32_checked_add_mul);
    register_bench!(registry, None, benchmark_i64_checked_add_mul);
    register_bench!(registry, None, benchmark_isize_checked_add_mul);
}

fn demo_checked_add_mul_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, z) in triples_of_unsigneds::<T>(gm).take(limit) {
        println!(
            "{}.checked_add_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.checked_add_mul(y, z)
        );
    }
}

fn demo_checked_add_mul_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, z) in triples_of_signeds::<T>(gm).take(limit) {
        println!(
            "({}).checked_add_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.checked_add_mul(y, z)
        );
    }
}

fn benchmark_checked_add_mul_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_add_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x, y, z)| no_out!(x.checked_add_mul(y, z))),
        )],
    );
}

fn benchmark_checked_add_mul_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.checked_add_mul({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(x, y, z)| no_out!(x.checked_add_mul(y, z))),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_checked_add_mul_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_checked_add_mul_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_checked_add_mul, benchmark_u8_checked_add_mul);
unsigned!(u16, demo_u16_checked_add_mul, benchmark_u16_checked_add_mul);
unsigned!(u32, demo_u32_checked_add_mul, benchmark_u32_checked_add_mul);
unsigned!(u64, demo_u64_checked_add_mul, benchmark_u64_checked_add_mul);
unsigned!(
    usize,
    demo_usize_checked_add_mul,
    benchmark_usize_checked_add_mul
);

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_checked_add_mul_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_checked_add_mul_signed::<$t>(gm, limit, file_name);
        }
    };
}

signed!(i8, demo_i8_checked_add_mul, benchmark_i8_checked_add_mul);
signed!(i16, demo_i16_checked_add_mul, benchmark_i16_checked_add_mul);
signed!(i32, demo_i32_checked_add_mul, benchmark_i32_checked_add_mul);
signed!(i64, demo_i64_checked_add_mul, benchmark_i64_checked_add_mul);
signed!(
    isize,
    demo_isize_checked_add_mul,
    benchmark_isize_checked_add_mul
);
