use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::integer::Integer;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_signed, pairs_of_integer_and_unsigned, pairs_of_signed_and_integer,
    pairs_of_unsigned_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_lt_abs_u8);
    register_demo!(registry, demo_u8_lt_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_i8);
    register_demo!(registry, demo_i8_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_u8);
    register_demo!(registry, demo_u8_gt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_i8);
    register_demo!(registry, demo_i8_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_u8);
    register_demo!(registry, demo_u8_le_abs_integer);
    register_demo!(registry, demo_integer_le_abs_i8);
    register_demo!(registry, demo_i8_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_u8);
    register_demo!(registry, demo_u8_ge_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_i8);
    register_demo!(registry, demo_i8_ge_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_u16);
    register_demo!(registry, demo_u16_lt_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_i16);
    register_demo!(registry, demo_i16_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_u16);
    register_demo!(registry, demo_u16_gt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_i16);
    register_demo!(registry, demo_i16_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_u16);
    register_demo!(registry, demo_u16_le_abs_integer);
    register_demo!(registry, demo_integer_le_abs_i16);
    register_demo!(registry, demo_i16_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_u16);
    register_demo!(registry, demo_u16_ge_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_i16);
    register_demo!(registry, demo_i16_ge_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_u32);
    register_demo!(registry, demo_u32_lt_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_i32);
    register_demo!(registry, demo_i32_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_u32);
    register_demo!(registry, demo_u32_gt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_i32);
    register_demo!(registry, demo_i32_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_u32);
    register_demo!(registry, demo_u32_le_abs_integer);
    register_demo!(registry, demo_integer_le_abs_i32);
    register_demo!(registry, demo_i32_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_u32);
    register_demo!(registry, demo_u32_ge_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_i32);
    register_demo!(registry, demo_i32_ge_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_u64);
    register_demo!(registry, demo_u64_lt_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_i64);
    register_demo!(registry, demo_i64_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_u64);
    register_demo!(registry, demo_u64_gt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_i64);
    register_demo!(registry, demo_i64_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_u64);
    register_demo!(registry, demo_u64_le_abs_integer);
    register_demo!(registry, demo_integer_le_abs_i64);
    register_demo!(registry, demo_i64_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_u64);
    register_demo!(registry, demo_u64_ge_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_i64);
    register_demo!(registry, demo_i64_ge_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_usize);
    register_demo!(registry, demo_usize_lt_abs_integer);
    register_demo!(registry, demo_integer_lt_abs_isize);
    register_demo!(registry, demo_isize_lt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_usize);
    register_demo!(registry, demo_usize_gt_abs_integer);
    register_demo!(registry, demo_integer_gt_abs_isize);
    register_demo!(registry, demo_isize_gt_abs_integer);
    register_demo!(registry, demo_integer_le_abs_usize);
    register_demo!(registry, demo_usize_le_abs_integer);
    register_demo!(registry, demo_integer_le_abs_isize);
    register_demo!(registry, demo_isize_le_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_usize);
    register_demo!(registry, demo_usize_ge_abs_integer);
    register_demo!(registry, demo_integer_ge_abs_isize);
    register_demo!(registry, demo_isize_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_u8);
    register_bench!(registry, Large, benchmark_u8_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_i8);
    register_bench!(registry, Large, benchmark_i8_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_u8);
    register_bench!(registry, Large, benchmark_u8_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_i8);
    register_bench!(registry, Large, benchmark_i8_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_u8);
    register_bench!(registry, Large, benchmark_u8_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_i8);
    register_bench!(registry, Large, benchmark_i8_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_u8);
    register_bench!(registry, Large, benchmark_u8_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_i8);
    register_bench!(registry, Large, benchmark_i8_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_u16);
    register_bench!(registry, Large, benchmark_u16_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_i16);
    register_bench!(registry, Large, benchmark_i16_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_u16);
    register_bench!(registry, Large, benchmark_u16_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_i16);
    register_bench!(registry, Large, benchmark_i16_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_u16);
    register_bench!(registry, Large, benchmark_u16_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_i16);
    register_bench!(registry, Large, benchmark_i16_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_u16);
    register_bench!(registry, Large, benchmark_u16_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_i16);
    register_bench!(registry, Large, benchmark_i16_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_u32);
    register_bench!(registry, Large, benchmark_u32_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_i32);
    register_bench!(registry, Large, benchmark_i32_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_u32);
    register_bench!(registry, Large, benchmark_u32_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_i32);
    register_bench!(registry, Large, benchmark_i32_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_u32);
    register_bench!(registry, Large, benchmark_u32_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_i32);
    register_bench!(registry, Large, benchmark_i32_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_u32);
    register_bench!(registry, Large, benchmark_u32_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_i32);
    register_bench!(registry, Large, benchmark_i32_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_u64);
    register_bench!(registry, Large, benchmark_u64_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_i64);
    register_bench!(registry, Large, benchmark_i64_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_u64);
    register_bench!(registry, Large, benchmark_u64_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_i64);
    register_bench!(registry, Large, benchmark_i64_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_u64);
    register_bench!(registry, Large, benchmark_u64_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_i64);
    register_bench!(registry, Large, benchmark_i64_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_u64);
    register_bench!(registry, Large, benchmark_u64_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_i64);
    register_bench!(registry, Large, benchmark_i64_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_usize);
    register_bench!(registry, Large, benchmark_usize_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_lt_abs_isize);
    register_bench!(registry, Large, benchmark_isize_lt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_usize);
    register_bench!(registry, Large, benchmark_usize_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_gt_abs_isize);
    register_bench!(registry, Large, benchmark_isize_gt_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_usize);
    register_bench!(registry, Large, benchmark_usize_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_le_abs_isize);
    register_bench!(registry, Large, benchmark_isize_le_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_usize);
    register_bench!(registry, Large, benchmark_usize_ge_abs_integer);
    register_bench!(registry, Large, benchmark_integer_ge_abs_isize);
    register_bench!(registry, Large, benchmark_isize_ge_abs_integer);
}

fn demo_integer_lt_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_unsigned_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_integer_lt_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_signed_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_integer_gt_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_unsigned_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_integer_gt_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_signed_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_integer_le_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_unsigned_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_integer_le_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_signed_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_integer_ge_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_integer_and_unsigned::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_unsigned_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_integer::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_integer_ge_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_integer_and_signed::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_signed_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_integer::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn benchmark_integer_lt_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_unsigned_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.lt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_integer_lt_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("Integer.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_signed_lt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.lt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_integer_gt_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_unsigned_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.gt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_integer_gt_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("Integer.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_signed_gt_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.gt_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_integer_le_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_unsigned_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.le_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_integer_le_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("Integer.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_signed_le_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.le_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_integer_ge_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
{
    run_benchmark(
        &format!("Integer.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_unsigned_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ge_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_integer_ge_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Integer: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("Integer.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_signed_ge_abs_integer<T: PartialOrdAbs<Integer> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}.ge_abs(&Integer)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

macro_rules! demo_and_bench {
    (
        $u:ident,
        $s:ident,
        $integer_lt_abs_unsigned_demo_name:ident,
        $unsigned_lt_abs_integer_demo_name:ident,
        $integer_lt_abs_signed_demo_name:ident,
        $signed_lt_abs_integer_demo_name:ident,
        $integer_gt_abs_unsigned_demo_name:ident,
        $unsigned_gt_abs_integer_demo_name:ident,
        $integer_gt_abs_signed_demo_name:ident,
        $signed_gt_abs_integer_demo_name:ident,
        $integer_le_abs_unsigned_demo_name:ident,
        $unsigned_le_abs_integer_demo_name:ident,
        $integer_le_abs_signed_demo_name:ident,
        $signed_le_abs_integer_demo_name:ident,
        $integer_ge_abs_unsigned_demo_name:ident,
        $unsigned_ge_abs_integer_demo_name:ident,
        $integer_ge_abs_signed_demo_name:ident,
        $signed_ge_abs_integer_demo_name:ident,
        $integer_lt_abs_unsigned_bench_name:ident,
        $unsigned_lt_abs_integer_bench_name:ident,
        $integer_lt_abs_signed_bench_name:ident,
        $signed_lt_abs_integer_bench_name:ident,
        $integer_gt_abs_unsigned_bench_name:ident,
        $unsigned_gt_abs_integer_bench_name:ident,
        $integer_gt_abs_signed_bench_name:ident,
        $signed_gt_abs_integer_bench_name:ident,
        $integer_le_abs_unsigned_bench_name:ident,
        $unsigned_le_abs_integer_bench_name:ident,
        $integer_le_abs_signed_bench_name:ident,
        $signed_le_abs_integer_bench_name:ident,
        $integer_ge_abs_unsigned_bench_name:ident,
        $unsigned_ge_abs_integer_bench_name:ident,
        $integer_ge_abs_signed_bench_name:ident,
        $signed_ge_abs_integer_bench_name:ident
    ) => {
        fn $integer_lt_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_lt_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_lt_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_lt_abs_integer::<$u>(gm, limit);
        }

        fn $integer_lt_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_lt_abs_signed::<$s>(gm, limit);
        }

        fn $signed_lt_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_lt_abs_integer::<$s>(gm, limit);
        }

        fn $integer_gt_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_gt_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_gt_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_gt_abs_integer::<$u>(gm, limit);
        }

        fn $integer_gt_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_gt_abs_signed::<$s>(gm, limit);
        }

        fn $signed_gt_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_gt_abs_integer::<$s>(gm, limit);
        }

        fn $integer_le_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_le_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_le_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_le_abs_integer::<$u>(gm, limit);
        }

        fn $integer_le_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_le_abs_signed::<$s>(gm, limit);
        }

        fn $signed_le_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_le_abs_integer::<$s>(gm, limit);
        }

        fn $integer_ge_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_ge_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_ge_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_ge_abs_integer::<$u>(gm, limit);
        }

        fn $integer_ge_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_integer_ge_abs_signed::<$s>(gm, limit);
        }

        fn $signed_ge_abs_integer_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_ge_abs_integer::<$s>(gm, limit);
        }

        fn $integer_lt_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_lt_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_lt_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_lt_abs_integer::<$u>(gm, limit, file_name);
        }

        fn $integer_lt_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_lt_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_lt_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_lt_abs_integer::<$s>(gm, limit, file_name);
        }

        fn $integer_gt_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_gt_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_gt_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_gt_abs_integer::<$u>(gm, limit, file_name);
        }

        fn $integer_gt_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_gt_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_gt_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_gt_abs_integer::<$s>(gm, limit, file_name);
        }

        fn $integer_le_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_le_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_le_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_le_abs_integer::<$u>(gm, limit, file_name);
        }

        fn $integer_le_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_le_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_le_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_le_abs_integer::<$s>(gm, limit, file_name);
        }

        fn $integer_ge_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_ge_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_ge_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_ge_abs_integer::<$u>(gm, limit, file_name);
        }

        fn $integer_ge_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_integer_ge_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_ge_abs_integer_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_ge_abs_integer::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_integer_lt_abs_u8,
    demo_u8_lt_abs_integer,
    demo_integer_lt_abs_i8,
    demo_i8_lt_abs_integer,
    demo_integer_gt_abs_u8,
    demo_u8_gt_abs_integer,
    demo_integer_gt_abs_i8,
    demo_i8_gt_abs_integer,
    demo_integer_le_abs_u8,
    demo_u8_le_abs_integer,
    demo_integer_le_abs_i8,
    demo_i8_le_abs_integer,
    demo_integer_ge_abs_u8,
    demo_u8_ge_abs_integer,
    demo_integer_ge_abs_i8,
    demo_i8_ge_abs_integer,
    benchmark_integer_lt_abs_u8,
    benchmark_u8_lt_abs_integer,
    benchmark_integer_lt_abs_i8,
    benchmark_i8_lt_abs_integer,
    benchmark_integer_gt_abs_u8,
    benchmark_u8_gt_abs_integer,
    benchmark_integer_gt_abs_i8,
    benchmark_i8_gt_abs_integer,
    benchmark_integer_le_abs_u8,
    benchmark_u8_le_abs_integer,
    benchmark_integer_le_abs_i8,
    benchmark_i8_le_abs_integer,
    benchmark_integer_ge_abs_u8,
    benchmark_u8_ge_abs_integer,
    benchmark_integer_ge_abs_i8,
    benchmark_i8_ge_abs_integer
);
demo_and_bench!(
    u16,
    i16,
    demo_integer_lt_abs_u16,
    demo_u16_lt_abs_integer,
    demo_integer_lt_abs_i16,
    demo_i16_lt_abs_integer,
    demo_integer_gt_abs_u16,
    demo_u16_gt_abs_integer,
    demo_integer_gt_abs_i16,
    demo_i16_gt_abs_integer,
    demo_integer_le_abs_u16,
    demo_u16_le_abs_integer,
    demo_integer_le_abs_i16,
    demo_i16_le_abs_integer,
    demo_integer_ge_abs_u16,
    demo_u16_ge_abs_integer,
    demo_integer_ge_abs_i16,
    demo_i16_ge_abs_integer,
    benchmark_integer_lt_abs_u16,
    benchmark_u16_lt_abs_integer,
    benchmark_integer_lt_abs_i16,
    benchmark_i16_lt_abs_integer,
    benchmark_integer_gt_abs_u16,
    benchmark_u16_gt_abs_integer,
    benchmark_integer_gt_abs_i16,
    benchmark_i16_gt_abs_integer,
    benchmark_integer_le_abs_u16,
    benchmark_u16_le_abs_integer,
    benchmark_integer_le_abs_i16,
    benchmark_i16_le_abs_integer,
    benchmark_integer_ge_abs_u16,
    benchmark_u16_ge_abs_integer,
    benchmark_integer_ge_abs_i16,
    benchmark_i16_ge_abs_integer
);
demo_and_bench!(
    u32,
    i32,
    demo_integer_lt_abs_u32,
    demo_u32_lt_abs_integer,
    demo_integer_lt_abs_i32,
    demo_i32_lt_abs_integer,
    demo_integer_gt_abs_u32,
    demo_u32_gt_abs_integer,
    demo_integer_gt_abs_i32,
    demo_i32_gt_abs_integer,
    demo_integer_le_abs_u32,
    demo_u32_le_abs_integer,
    demo_integer_le_abs_i32,
    demo_i32_le_abs_integer,
    demo_integer_ge_abs_u32,
    demo_u32_ge_abs_integer,
    demo_integer_ge_abs_i32,
    demo_i32_ge_abs_integer,
    benchmark_integer_lt_abs_u32,
    benchmark_u32_lt_abs_integer,
    benchmark_integer_lt_abs_i32,
    benchmark_i32_lt_abs_integer,
    benchmark_integer_gt_abs_u32,
    benchmark_u32_gt_abs_integer,
    benchmark_integer_gt_abs_i32,
    benchmark_i32_gt_abs_integer,
    benchmark_integer_le_abs_u32,
    benchmark_u32_le_abs_integer,
    benchmark_integer_le_abs_i32,
    benchmark_i32_le_abs_integer,
    benchmark_integer_ge_abs_u32,
    benchmark_u32_ge_abs_integer,
    benchmark_integer_ge_abs_i32,
    benchmark_i32_ge_abs_integer
);
demo_and_bench!(
    u64,
    i64,
    demo_integer_lt_abs_u64,
    demo_u64_lt_abs_integer,
    demo_integer_lt_abs_i64,
    demo_i64_lt_abs_integer,
    demo_integer_gt_abs_u64,
    demo_u64_gt_abs_integer,
    demo_integer_gt_abs_i64,
    demo_i64_gt_abs_integer,
    demo_integer_le_abs_u64,
    demo_u64_le_abs_integer,
    demo_integer_le_abs_i64,
    demo_i64_le_abs_integer,
    demo_integer_ge_abs_u64,
    demo_u64_ge_abs_integer,
    demo_integer_ge_abs_i64,
    demo_i64_ge_abs_integer,
    benchmark_integer_lt_abs_u64,
    benchmark_u64_lt_abs_integer,
    benchmark_integer_lt_abs_i64,
    benchmark_i64_lt_abs_integer,
    benchmark_integer_gt_abs_u64,
    benchmark_u64_gt_abs_integer,
    benchmark_integer_gt_abs_i64,
    benchmark_i64_gt_abs_integer,
    benchmark_integer_le_abs_u64,
    benchmark_u64_le_abs_integer,
    benchmark_integer_le_abs_i64,
    benchmark_i64_le_abs_integer,
    benchmark_integer_ge_abs_u64,
    benchmark_u64_ge_abs_integer,
    benchmark_integer_ge_abs_i64,
    benchmark_i64_ge_abs_integer
);
demo_and_bench!(
    usize,
    isize,
    demo_integer_lt_abs_usize,
    demo_usize_lt_abs_integer,
    demo_integer_lt_abs_isize,
    demo_isize_lt_abs_integer,
    demo_integer_gt_abs_usize,
    demo_usize_gt_abs_integer,
    demo_integer_gt_abs_isize,
    demo_isize_gt_abs_integer,
    demo_integer_le_abs_usize,
    demo_usize_le_abs_integer,
    demo_integer_le_abs_isize,
    demo_isize_le_abs_integer,
    demo_integer_ge_abs_usize,
    demo_usize_ge_abs_integer,
    demo_integer_ge_abs_isize,
    demo_isize_ge_abs_integer,
    benchmark_integer_lt_abs_usize,
    benchmark_usize_lt_abs_integer,
    benchmark_integer_lt_abs_isize,
    benchmark_isize_lt_abs_integer,
    benchmark_integer_gt_abs_usize,
    benchmark_usize_gt_abs_integer,
    benchmark_integer_gt_abs_isize,
    benchmark_isize_gt_abs_integer,
    benchmark_integer_le_abs_usize,
    benchmark_usize_le_abs_integer,
    benchmark_integer_le_abs_isize,
    benchmark_isize_le_abs_integer,
    benchmark_integer_ge_abs_usize,
    benchmark_usize_ge_abs_integer,
    benchmark_integer_ge_abs_isize,
    benchmark_isize_ge_abs_integer
);
