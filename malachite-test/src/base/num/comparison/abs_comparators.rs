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
    register_demo!(registry, demo_u8_lt_abs_u8);
    register_demo!(registry, demo_u16_lt_abs_u16);
    register_demo!(registry, demo_u32_lt_abs_u32);
    register_demo!(registry, demo_u64_lt_abs_u64);
    register_demo!(registry, demo_usize_lt_abs_usize);
    register_demo!(registry, demo_i8_lt_abs_i8);
    register_demo!(registry, demo_i16_lt_abs_i16);
    register_demo!(registry, demo_i32_lt_abs_i32);
    register_demo!(registry, demo_i64_lt_abs_i64);
    register_demo!(registry, demo_isize_lt_abs_isize);
    register_demo!(registry, demo_u8_gt_abs_u8);
    register_demo!(registry, demo_u16_gt_abs_u16);
    register_demo!(registry, demo_u32_gt_abs_u32);
    register_demo!(registry, demo_u64_gt_abs_u64);
    register_demo!(registry, demo_usize_gt_abs_usize);
    register_demo!(registry, demo_i8_gt_abs_i8);
    register_demo!(registry, demo_i16_gt_abs_i16);
    register_demo!(registry, demo_i32_gt_abs_i32);
    register_demo!(registry, demo_i64_gt_abs_i64);
    register_demo!(registry, demo_isize_gt_abs_isize);
    register_demo!(registry, demo_u8_le_abs_u8);
    register_demo!(registry, demo_u16_le_abs_u16);
    register_demo!(registry, demo_u32_le_abs_u32);
    register_demo!(registry, demo_u64_le_abs_u64);
    register_demo!(registry, demo_usize_le_abs_usize);
    register_demo!(registry, demo_i8_le_abs_i8);
    register_demo!(registry, demo_i16_le_abs_i16);
    register_demo!(registry, demo_i32_le_abs_i32);
    register_demo!(registry, demo_i64_le_abs_i64);
    register_demo!(registry, demo_isize_le_abs_isize);
    register_demo!(registry, demo_u8_ge_abs_u8);
    register_demo!(registry, demo_u16_ge_abs_u16);
    register_demo!(registry, demo_u32_ge_abs_u32);
    register_demo!(registry, demo_u64_ge_abs_u64);
    register_demo!(registry, demo_usize_ge_abs_usize);
    register_demo!(registry, demo_i8_ge_abs_i8);
    register_demo!(registry, demo_i16_ge_abs_i16);
    register_demo!(registry, demo_i32_ge_abs_i32);
    register_demo!(registry, demo_i64_ge_abs_i64);
    register_demo!(registry, demo_isize_ge_abs_isize);
    register_bench!(registry, None, benchmark_u8_lt_abs_u8);
    register_bench!(registry, None, benchmark_u16_lt_abs_u16);
    register_bench!(registry, None, benchmark_u32_lt_abs_u32);
    register_bench!(registry, None, benchmark_u64_lt_abs_u64);
    register_bench!(registry, None, benchmark_usize_lt_abs_usize);
    register_bench!(registry, None, benchmark_i8_lt_abs_i8);
    register_bench!(registry, None, benchmark_i16_lt_abs_i16);
    register_bench!(registry, None, benchmark_i32_lt_abs_i32);
    register_bench!(registry, None, benchmark_i64_lt_abs_i64);
    register_bench!(registry, None, benchmark_isize_lt_abs_isize);
    register_bench!(registry, None, benchmark_u8_gt_abs_u8);
    register_bench!(registry, None, benchmark_u16_gt_abs_u16);
    register_bench!(registry, None, benchmark_u32_gt_abs_u32);
    register_bench!(registry, None, benchmark_u64_gt_abs_u64);
    register_bench!(registry, None, benchmark_usize_gt_abs_usize);
    register_bench!(registry, None, benchmark_i8_gt_abs_i8);
    register_bench!(registry, None, benchmark_i16_gt_abs_i16);
    register_bench!(registry, None, benchmark_i32_gt_abs_i32);
    register_bench!(registry, None, benchmark_i64_gt_abs_i64);
    register_bench!(registry, None, benchmark_isize_gt_abs_isize);
    register_bench!(registry, None, benchmark_u8_le_abs_u8);
    register_bench!(registry, None, benchmark_u16_le_abs_u16);
    register_bench!(registry, None, benchmark_u32_le_abs_u32);
    register_bench!(registry, None, benchmark_u64_le_abs_u64);
    register_bench!(registry, None, benchmark_usize_le_abs_usize);
    register_bench!(registry, None, benchmark_i8_le_abs_i8);
    register_bench!(registry, None, benchmark_i16_le_abs_i16);
    register_bench!(registry, None, benchmark_i32_le_abs_i32);
    register_bench!(registry, None, benchmark_i64_le_abs_i64);
    register_bench!(registry, None, benchmark_isize_le_abs_isize);
    register_bench!(registry, None, benchmark_u8_ge_abs_u8);
    register_bench!(registry, None, benchmark_u16_ge_abs_u16);
    register_bench!(registry, None, benchmark_u32_ge_abs_u32);
    register_bench!(registry, None, benchmark_u64_ge_abs_u64);
    register_bench!(registry, None, benchmark_usize_ge_abs_usize);
    register_bench!(registry, None, benchmark_i8_ge_abs_i8);
    register_bench!(registry, None, benchmark_i16_ge_abs_i16);
    register_bench!(registry, None, benchmark_i32_ge_abs_i32);
    register_bench!(registry, None, benchmark_i64_ge_abs_i64);
    register_bench!(registry, None, benchmark_isize_ge_abs_isize);
}

fn demo_unsigned_lt_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_signed_lt_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_unsigned_gt_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_signed_gt_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_unsigned_le_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_signed_le_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_unsigned_ge_abs<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_signed_ge_abs<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn benchmark_unsigned_lt_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.lt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_signed_lt_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.lt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_unsigned_gt_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.gt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_signed_gt_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.gt_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_unsigned_le_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.le_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_signed_le_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.le_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_unsigned_ge_abs<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.ge_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_signed_ge_abs<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.ge_abs(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $lt_demo_name:ident,
        $lt_bench_name:ident,
        $gt_demo_name:ident,
        $gt_bench_name:ident,
        $le_demo_name:ident,
        $le_bench_name:ident,
        $ge_demo_name:ident,
        $ge_bench_name:ident
    ) => {
        fn $lt_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_lt_abs::<$t>(gm, limit);
        }

        fn $lt_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_lt_abs::<$t>(gm, limit, file_name);
        }

        fn $gt_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_gt_abs::<$t>(gm, limit);
        }

        fn $gt_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_gt_abs::<$t>(gm, limit, file_name);
        }

        fn $le_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_le_abs::<$t>(gm, limit);
        }

        fn $le_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_le_abs::<$t>(gm, limit, file_name);
        }

        fn $ge_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_ge_abs::<$t>(gm, limit);
        }

        fn $ge_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_ge_abs::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $lt_demo_name:ident,
        $lt_bench_name:ident,
        $gt_demo_name:ident,
        $gt_bench_name:ident,
        $le_demo_name:ident,
        $le_bench_name:ident,
        $ge_demo_name:ident,
        $ge_bench_name:ident
    ) => {
        fn $lt_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_lt_abs::<$t>(gm, limit);
        }

        fn $lt_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_lt_abs::<$t>(gm, limit, file_name);
        }

        fn $gt_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_gt_abs::<$t>(gm, limit);
        }

        fn $gt_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_gt_abs::<$t>(gm, limit, file_name);
        }

        fn $le_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_le_abs::<$t>(gm, limit);
        }

        fn $le_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_le_abs::<$t>(gm, limit, file_name);
        }

        fn $ge_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_ge_abs::<$t>(gm, limit);
        }

        fn $ge_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_ge_abs::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_lt_abs_u8,
    benchmark_u8_lt_abs_u8,
    demo_u8_gt_abs_u8,
    benchmark_u8_gt_abs_u8,
    demo_u8_le_abs_u8,
    benchmark_u8_le_abs_u8,
    demo_u8_ge_abs_u8,
    benchmark_u8_ge_abs_u8
);
unsigned!(
    u16,
    demo_u16_lt_abs_u16,
    benchmark_u16_lt_abs_u16,
    demo_u16_gt_abs_u16,
    benchmark_u16_gt_abs_u16,
    demo_u16_le_abs_u16,
    benchmark_u16_le_abs_u16,
    demo_u16_ge_abs_u16,
    benchmark_u16_ge_abs_u16
);
unsigned!(
    u32,
    demo_u32_lt_abs_u32,
    benchmark_u32_lt_abs_u32,
    demo_u32_gt_abs_u32,
    benchmark_u32_gt_abs_u32,
    demo_u32_le_abs_u32,
    benchmark_u32_le_abs_u32,
    demo_u32_ge_abs_u32,
    benchmark_u32_ge_abs_u32
);
unsigned!(
    u64,
    demo_u64_lt_abs_u64,
    benchmark_u64_lt_abs_u64,
    demo_u64_gt_abs_u64,
    benchmark_u64_gt_abs_u64,
    demo_u64_le_abs_u64,
    benchmark_u64_le_abs_u64,
    demo_u64_ge_abs_u64,
    benchmark_u64_ge_abs_u64
);
unsigned!(
    usize,
    demo_usize_lt_abs_usize,
    benchmark_usize_lt_abs_usize,
    demo_usize_gt_abs_usize,
    benchmark_usize_gt_abs_usize,
    demo_usize_le_abs_usize,
    benchmark_usize_le_abs_usize,
    demo_usize_ge_abs_usize,
    benchmark_usize_ge_abs_usize
);

signed!(
    i8,
    demo_i8_lt_abs_i8,
    benchmark_i8_lt_abs_i8,
    demo_i8_gt_abs_i8,
    benchmark_i8_gt_abs_i8,
    demo_i8_le_abs_i8,
    benchmark_i8_le_abs_i8,
    demo_i8_ge_abs_i8,
    benchmark_i8_ge_abs_i8
);
signed!(
    i16,
    demo_i16_lt_abs_i16,
    benchmark_i16_lt_abs_i16,
    demo_i16_gt_abs_i16,
    benchmark_i16_gt_abs_i16,
    demo_i16_le_abs_i16,
    benchmark_i16_le_abs_i16,
    demo_i16_ge_abs_i16,
    benchmark_i16_ge_abs_i16
);
signed!(
    i32,
    demo_i32_lt_abs_i32,
    benchmark_i32_lt_abs_i32,
    demo_i32_gt_abs_i32,
    benchmark_i32_gt_abs_i32,
    demo_i32_le_abs_i32,
    benchmark_i32_le_abs_i32,
    demo_i32_ge_abs_i32,
    benchmark_i32_ge_abs_i32
);
signed!(
    i64,
    demo_i64_lt_abs_i64,
    benchmark_i64_lt_abs_i64,
    demo_i64_gt_abs_i64,
    benchmark_i64_gt_abs_i64,
    demo_i64_le_abs_i64,
    benchmark_i64_le_abs_i64,
    demo_i64_ge_abs_i64,
    benchmark_i64_ge_abs_i64
);
signed!(
    isize,
    demo_isize_lt_abs_isize,
    benchmark_isize_lt_abs_isize,
    demo_isize_gt_abs_isize,
    benchmark_isize_gt_abs_isize,
    demo_isize_le_abs_isize,
    benchmark_isize_le_abs_isize,
    demo_isize_ge_abs_isize,
    benchmark_isize_ge_abs_isize
);
