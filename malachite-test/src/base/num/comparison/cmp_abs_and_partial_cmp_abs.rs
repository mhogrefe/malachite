use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_cmp_abs);
    register_demo!(registry, demo_u16_cmp_abs);
    register_demo!(registry, demo_u32_cmp_abs);
    register_demo!(registry, demo_u64_cmp_abs);
    register_demo!(registry, demo_usize_cmp_abs);
    register_demo!(registry, demo_i8_cmp_abs);
    register_demo!(registry, demo_i16_cmp_abs);
    register_demo!(registry, demo_i32_cmp_abs);
    register_demo!(registry, demo_i64_cmp_abs);
    register_demo!(registry, demo_isize_cmp_abs);
    register_demo!(registry, demo_u8_partial_cmp_abs_u8);
    register_demo!(registry, demo_u16_partial_cmp_abs_u16);
    register_demo!(registry, demo_u32_partial_cmp_abs_u32);
    register_demo!(registry, demo_u64_partial_cmp_abs_u64);
    register_demo!(registry, demo_usize_partial_cmp_abs_usize);
    register_demo!(registry, demo_i8_partial_cmp_abs_i8);
    register_demo!(registry, demo_i16_partial_cmp_abs_i16);
    register_demo!(registry, demo_i32_partial_cmp_abs_i32);
    register_demo!(registry, demo_i64_partial_cmp_abs_i64);
    register_demo!(registry, demo_isize_partial_cmp_abs_isize);
    register_bench!(registry, None, benchmark_u8_cmp_abs);
    register_bench!(registry, None, benchmark_u16_cmp_abs);
    register_bench!(registry, None, benchmark_u32_cmp_abs);
    register_bench!(registry, None, benchmark_u64_cmp_abs);
    register_bench!(registry, None, benchmark_usize_cmp_abs);
    register_bench!(registry, None, benchmark_i8_cmp_abs);
    register_bench!(registry, None, benchmark_i16_cmp_abs);
    register_bench!(registry, None, benchmark_i32_cmp_abs);
    register_bench!(registry, None, benchmark_i64_cmp_abs);
    register_bench!(registry, None, benchmark_isize_cmp_abs);
    register_bench!(registry, None, benchmark_u8_partial_cmp_abs_u8);
    register_bench!(registry, None, benchmark_u16_partial_cmp_abs_u16);
    register_bench!(registry, None, benchmark_u32_partial_cmp_abs_u32);
    register_bench!(registry, None, benchmark_u64_partial_cmp_abs_u64);
    register_bench!(registry, None, benchmark_usize_partial_cmp_abs_usize);
    register_bench!(registry, None, benchmark_i8_partial_cmp_abs_i8);
    register_bench!(registry, None, benchmark_i16_partial_cmp_abs_i16);
    register_bench!(registry, None, benchmark_i32_partial_cmp_abs_i32);
    register_bench!(registry, None, benchmark_i64_partial_cmp_abs_i64);
    register_bench!(registry, None, benchmark_isize_partial_cmp_abs_isize);
}

fn demo_unsigned_cmp_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.cmp_abs(&{}) = {:?}", x, y, x.cmp_abs(&y));
    }
}

fn demo_signed_cmp_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("{}.cmp_abs(&{}) = {:?}", x, y, x.cmp_abs(&y));
    }
}

fn demo_unsigned_partial_cmp_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn demo_signed_partial_cmp_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!(
            "{}.partial_cmp_abs(&{}) = {:?}",
            x,
            y,
            x.partial_cmp_abs(&y)
        );
    }
}

fn benchmark_unsigned_cmp_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.cmp_abs(&y))))],
    );
}

fn benchmark_signed_cmp_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.cmp_abs(&y))))],
    );
}

fn benchmark_unsigned_partial_cmp_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.partial_cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

fn benchmark_signed_partial_cmp_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.partial_cmp_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $cmp_demo_name:ident,
        $cmp_bench_name:ident,
        $partial_cmp_demo_name:ident,
        $partial_cmp_bench_name:ident
    ) => {
        fn $cmp_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_cmp_abs::<$t>(gm, limit);
        }

        fn $cmp_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_cmp_abs::<$t>(gm, limit, file_name);
        }

        fn $partial_cmp_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_partial_cmp_abs::<$t>(gm, limit);
        }

        fn $partial_cmp_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_partial_cmp_abs::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $cmp_demo_name:ident,
        $cmp_bench_name:ident,
        $partial_cmp_demo_name:ident,
        $partial_cmp_bench_name:ident
    ) => {
        fn $cmp_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_cmp_abs::<$t>(gm, limit);
        }

        fn $cmp_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_cmp_abs::<$t>(gm, limit, file_name);
        }

        fn $partial_cmp_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_partial_cmp_abs::<$t>(gm, limit);
        }

        fn $partial_cmp_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_partial_cmp_abs::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_cmp_abs,
    benchmark_u8_cmp_abs,
    demo_u8_partial_cmp_abs_u8,
    benchmark_u8_partial_cmp_abs_u8
);
unsigned!(
    u16,
    demo_u16_cmp_abs,
    benchmark_u16_cmp_abs,
    demo_u16_partial_cmp_abs_u16,
    benchmark_u16_partial_cmp_abs_u16
);
unsigned!(
    u32,
    demo_u32_cmp_abs,
    benchmark_u32_cmp_abs,
    demo_u32_partial_cmp_abs_u32,
    benchmark_u32_partial_cmp_abs_u32
);
unsigned!(
    u64,
    demo_u64_cmp_abs,
    benchmark_u64_cmp_abs,
    demo_u64_partial_cmp_abs_u64,
    benchmark_u64_partial_cmp_abs_u64
);
unsigned!(
    usize,
    demo_usize_cmp_abs,
    benchmark_usize_cmp_abs,
    demo_usize_partial_cmp_abs_usize,
    benchmark_usize_partial_cmp_abs_usize
);

signed!(
    i8,
    demo_i8_cmp_abs,
    benchmark_i8_cmp_abs,
    demo_i8_partial_cmp_abs_i8,
    benchmark_i8_partial_cmp_abs_i8
);
signed!(
    i16,
    demo_i16_cmp_abs,
    benchmark_i16_cmp_abs,
    demo_i16_partial_cmp_abs_i16,
    benchmark_i16_partial_cmp_abs_i16
);
signed!(
    i32,
    demo_i32_cmp_abs,
    benchmark_i32_cmp_abs,
    demo_i32_partial_cmp_abs_i32,
    benchmark_i32_partial_cmp_abs_i32
);
signed!(
    i64,
    demo_i64_cmp_abs,
    benchmark_i64_cmp_abs,
    demo_i64_partial_cmp_abs_i64,
    benchmark_i64_partial_cmp_abs_i64
);
signed!(
    isize,
    demo_isize_cmp_abs,
    benchmark_isize_cmp_abs,
    demo_isize_partial_cmp_abs_isize,
    benchmark_isize_partial_cmp_abs_isize
);
