use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    pairs_of_natural_and_signed, pairs_of_natural_and_unsigned, pairs_of_signed_and_natural,
    pairs_of_unsigned_and_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_lt_abs_u8);
    register_demo!(registry, demo_u8_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_i8);
    register_demo!(registry, demo_i8_lt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_u8);
    register_demo!(registry, demo_u8_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_i8);
    register_demo!(registry, demo_i8_gt_abs_natural);
    register_demo!(registry, demo_natural_le_abs_u8);
    register_demo!(registry, demo_u8_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_i8);
    register_demo!(registry, demo_i8_le_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_u8);
    register_demo!(registry, demo_u8_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_i8);
    register_demo!(registry, demo_i8_ge_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_u16);
    register_demo!(registry, demo_u16_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_i16);
    register_demo!(registry, demo_i16_lt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_u16);
    register_demo!(registry, demo_u16_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_i16);
    register_demo!(registry, demo_i16_gt_abs_natural);
    register_demo!(registry, demo_natural_le_abs_u16);
    register_demo!(registry, demo_u16_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_i16);
    register_demo!(registry, demo_i16_le_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_u16);
    register_demo!(registry, demo_u16_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_i16);
    register_demo!(registry, demo_i16_ge_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_u32);
    register_demo!(registry, demo_u32_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_i32);
    register_demo!(registry, demo_i32_lt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_u32);
    register_demo!(registry, demo_u32_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_i32);
    register_demo!(registry, demo_i32_gt_abs_natural);
    register_demo!(registry, demo_natural_le_abs_u32);
    register_demo!(registry, demo_u32_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_i32);
    register_demo!(registry, demo_i32_le_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_u32);
    register_demo!(registry, demo_u32_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_i32);
    register_demo!(registry, demo_i32_ge_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_u64);
    register_demo!(registry, demo_u64_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_i64);
    register_demo!(registry, demo_i64_lt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_u64);
    register_demo!(registry, demo_u64_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_i64);
    register_demo!(registry, demo_i64_gt_abs_natural);
    register_demo!(registry, demo_natural_le_abs_u64);
    register_demo!(registry, demo_u64_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_i64);
    register_demo!(registry, demo_i64_le_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_u64);
    register_demo!(registry, demo_u64_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_i64);
    register_demo!(registry, demo_i64_ge_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_usize);
    register_demo!(registry, demo_usize_lt_abs_natural);
    register_demo!(registry, demo_natural_lt_abs_isize);
    register_demo!(registry, demo_isize_lt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_usize);
    register_demo!(registry, demo_usize_gt_abs_natural);
    register_demo!(registry, demo_natural_gt_abs_isize);
    register_demo!(registry, demo_isize_gt_abs_natural);
    register_demo!(registry, demo_natural_le_abs_usize);
    register_demo!(registry, demo_usize_le_abs_natural);
    register_demo!(registry, demo_natural_le_abs_isize);
    register_demo!(registry, demo_isize_le_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_usize);
    register_demo!(registry, demo_usize_ge_abs_natural);
    register_demo!(registry, demo_natural_ge_abs_isize);
    register_demo!(registry, demo_isize_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_u8);
    register_bench!(registry, Large, benchmark_u8_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_i8);
    register_bench!(registry, Large, benchmark_i8_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_u8);
    register_bench!(registry, Large, benchmark_u8_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_i8);
    register_bench!(registry, Large, benchmark_i8_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_u8);
    register_bench!(registry, Large, benchmark_u8_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_i8);
    register_bench!(registry, Large, benchmark_i8_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_u8);
    register_bench!(registry, Large, benchmark_u8_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_i8);
    register_bench!(registry, Large, benchmark_i8_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_u16);
    register_bench!(registry, Large, benchmark_u16_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_i16);
    register_bench!(registry, Large, benchmark_i16_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_u16);
    register_bench!(registry, Large, benchmark_u16_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_i16);
    register_bench!(registry, Large, benchmark_i16_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_u16);
    register_bench!(registry, Large, benchmark_u16_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_i16);
    register_bench!(registry, Large, benchmark_i16_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_u16);
    register_bench!(registry, Large, benchmark_u16_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_i16);
    register_bench!(registry, Large, benchmark_i16_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_u32);
    register_bench!(registry, Large, benchmark_u32_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_i32);
    register_bench!(registry, Large, benchmark_i32_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_u32);
    register_bench!(registry, Large, benchmark_u32_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_i32);
    register_bench!(registry, Large, benchmark_i32_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_u32);
    register_bench!(registry, Large, benchmark_u32_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_i32);
    register_bench!(registry, Large, benchmark_i32_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_u32);
    register_bench!(registry, Large, benchmark_u32_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_i32);
    register_bench!(registry, Large, benchmark_i32_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_u64);
    register_bench!(registry, Large, benchmark_u64_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_i64);
    register_bench!(registry, Large, benchmark_i64_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_u64);
    register_bench!(registry, Large, benchmark_u64_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_i64);
    register_bench!(registry, Large, benchmark_i64_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_u64);
    register_bench!(registry, Large, benchmark_u64_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_i64);
    register_bench!(registry, Large, benchmark_i64_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_u64);
    register_bench!(registry, Large, benchmark_u64_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_i64);
    register_bench!(registry, Large, benchmark_i64_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_usize);
    register_bench!(registry, Large, benchmark_usize_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_lt_abs_isize);
    register_bench!(registry, Large, benchmark_isize_lt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_usize);
    register_bench!(registry, Large, benchmark_usize_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_gt_abs_isize);
    register_bench!(registry, Large, benchmark_isize_gt_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_usize);
    register_bench!(registry, Large, benchmark_usize_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_le_abs_isize);
    register_bench!(registry, Large, benchmark_isize_le_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_usize);
    register_bench!(registry, Large, benchmark_usize_ge_abs_natural);
    register_bench!(registry, Large, benchmark_natural_ge_abs_isize);
    register_bench!(registry, Large, benchmark_isize_ge_abs_natural);
}

fn demo_natural_lt_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_unsigned_lt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_natural_lt_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_signed_lt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        println!("{}.lt_abs(&{}) = {}", x, y, x.lt_abs(&y));
    }
}

fn demo_natural_gt_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_unsigned_gt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_natural_gt_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_signed_gt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        println!("{}.gt_abs(&{}) = {}", x, y, x.gt_abs(&y));
    }
}

fn demo_natural_le_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_unsigned_le_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_natural_le_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_signed_le_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        println!("{}.le_abs(&{}) = {}", x, y, x.le_abs(&y));
    }
}

fn demo_natural_ge_abs_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
{
    for (x, y) in pairs_of_natural_and_unsigned::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_unsigned_ge_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (x, y) in pairs_of_unsigned_and_natural::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_natural_ge_abs_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_natural_and_signed::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn demo_signed_ge_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_natural::<T>(gm).take(limit) {
        println!("{}.ge_abs(&{}) = {}", x, y, x.ge_abs(&y));
    }
}

fn benchmark_natural_lt_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
{
    m_run_benchmark(
        &format!("Natural.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_unsigned_lt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.lt_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_natural_lt_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Natural.lt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_signed_lt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.lt_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.lt_abs(&y))))],
    );
}

fn benchmark_natural_gt_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
{
    m_run_benchmark(
        &format!("Natural.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_unsigned_gt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.gt_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_natural_gt_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Natural.gt_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_signed_gt_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.gt_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.gt_abs(&y))))],
    );
}

fn benchmark_natural_le_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
{
    m_run_benchmark(
        &format!("Natural.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_unsigned_le_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.le_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_natural_le_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Natural.le_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_signed_le_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.le_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.le_abs(&y))))],
    );
}

fn benchmark_natural_ge_abs_unsigned<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
{
    m_run_benchmark(
        &format!("Natural.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_unsigned_ge_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.ge_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, y)| usize::exact_from(y.significant_bits())),
        "y.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_natural_ge_abs_signed<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    Natural: PartialOrdAbs<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("Natural.ge_abs(&{})", T::NAME),
        BenchmarkType::Single,
        pairs_of_natural_and_signed::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.ge_abs(&y))))],
    );
}

fn benchmark_signed_ge_abs_natural<T: PartialOrdAbs<Natural> + PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.ge_abs(&Natural)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_natural::<T>(gm),
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
        $natural_lt_abs_unsigned_demo_name:ident,
        $unsigned_lt_abs_natural_demo_name:ident,
        $natural_lt_abs_signed_demo_name:ident,
        $signed_lt_abs_natural_demo_name:ident,
        $natural_gt_abs_unsigned_demo_name:ident,
        $unsigned_gt_abs_natural_demo_name:ident,
        $natural_gt_abs_signed_demo_name:ident,
        $signed_gt_abs_natural_demo_name:ident,
        $natural_le_abs_unsigned_demo_name:ident,
        $unsigned_le_abs_natural_demo_name:ident,
        $natural_le_abs_signed_demo_name:ident,
        $signed_le_abs_natural_demo_name:ident,
        $natural_ge_abs_unsigned_demo_name:ident,
        $unsigned_ge_abs_natural_demo_name:ident,
        $natural_ge_abs_signed_demo_name:ident,
        $signed_ge_abs_natural_demo_name:ident,
        $natural_lt_abs_unsigned_bench_name:ident,
        $unsigned_lt_abs_natural_bench_name:ident,
        $natural_lt_abs_signed_bench_name:ident,
        $signed_lt_abs_natural_bench_name:ident,
        $natural_gt_abs_unsigned_bench_name:ident,
        $unsigned_gt_abs_natural_bench_name:ident,
        $natural_gt_abs_signed_bench_name:ident,
        $signed_gt_abs_natural_bench_name:ident,
        $natural_le_abs_unsigned_bench_name:ident,
        $unsigned_le_abs_natural_bench_name:ident,
        $natural_le_abs_signed_bench_name:ident,
        $signed_le_abs_natural_bench_name:ident,
        $natural_ge_abs_unsigned_bench_name:ident,
        $unsigned_ge_abs_natural_bench_name:ident,
        $natural_ge_abs_signed_bench_name:ident,
        $signed_ge_abs_natural_bench_name:ident
    ) => {
        fn $natural_lt_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_lt_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_lt_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_lt_abs_natural::<$u>(gm, limit);
        }

        fn $natural_lt_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_lt_abs_signed::<$s>(gm, limit);
        }

        fn $signed_lt_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_lt_abs_natural::<$s>(gm, limit);
        }

        fn $natural_gt_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_gt_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_gt_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_gt_abs_natural::<$u>(gm, limit);
        }

        fn $natural_gt_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_gt_abs_signed::<$s>(gm, limit);
        }

        fn $signed_gt_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_gt_abs_natural::<$s>(gm, limit);
        }

        fn $natural_le_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_le_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_le_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_le_abs_natural::<$u>(gm, limit);
        }

        fn $natural_le_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_le_abs_signed::<$s>(gm, limit);
        }

        fn $signed_le_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_le_abs_natural::<$s>(gm, limit);
        }

        fn $natural_ge_abs_unsigned_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_ge_abs_unsigned::<$u>(gm, limit);
        }

        fn $unsigned_ge_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_ge_abs_natural::<$u>(gm, limit);
        }

        fn $natural_ge_abs_signed_demo_name(gm: GenerationMode, limit: usize) {
            demo_natural_ge_abs_signed::<$s>(gm, limit);
        }

        fn $signed_ge_abs_natural_demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_ge_abs_natural::<$s>(gm, limit);
        }

        fn $natural_lt_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_lt_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_lt_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_lt_abs_natural::<$u>(gm, limit, file_name);
        }

        fn $natural_lt_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_lt_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_lt_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_lt_abs_natural::<$s>(gm, limit, file_name);
        }

        fn $natural_gt_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_gt_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_gt_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_gt_abs_natural::<$u>(gm, limit, file_name);
        }

        fn $natural_gt_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_gt_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_gt_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_gt_abs_natural::<$s>(gm, limit, file_name);
        }

        fn $natural_le_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_le_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_le_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_le_abs_natural::<$u>(gm, limit, file_name);
        }

        fn $natural_le_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_le_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_le_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_le_abs_natural::<$s>(gm, limit, file_name);
        }

        fn $natural_ge_abs_unsigned_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_ge_abs_unsigned::<$u>(gm, limit, file_name);
        }

        fn $unsigned_ge_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_ge_abs_natural::<$u>(gm, limit, file_name);
        }

        fn $natural_ge_abs_signed_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_natural_ge_abs_signed::<$s>(gm, limit, file_name);
        }

        fn $signed_ge_abs_natural_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_ge_abs_natural::<$s>(gm, limit, file_name);
        }
    };
}

demo_and_bench!(
    u8,
    i8,
    demo_natural_lt_abs_u8,
    demo_u8_lt_abs_natural,
    demo_natural_lt_abs_i8,
    demo_i8_lt_abs_natural,
    demo_natural_gt_abs_u8,
    demo_u8_gt_abs_natural,
    demo_natural_gt_abs_i8,
    demo_i8_gt_abs_natural,
    demo_natural_le_abs_u8,
    demo_u8_le_abs_natural,
    demo_natural_le_abs_i8,
    demo_i8_le_abs_natural,
    demo_natural_ge_abs_u8,
    demo_u8_ge_abs_natural,
    demo_natural_ge_abs_i8,
    demo_i8_ge_abs_natural,
    benchmark_natural_lt_abs_u8,
    benchmark_u8_lt_abs_natural,
    benchmark_natural_lt_abs_i8,
    benchmark_i8_lt_abs_natural,
    benchmark_natural_gt_abs_u8,
    benchmark_u8_gt_abs_natural,
    benchmark_natural_gt_abs_i8,
    benchmark_i8_gt_abs_natural,
    benchmark_natural_le_abs_u8,
    benchmark_u8_le_abs_natural,
    benchmark_natural_le_abs_i8,
    benchmark_i8_le_abs_natural,
    benchmark_natural_ge_abs_u8,
    benchmark_u8_ge_abs_natural,
    benchmark_natural_ge_abs_i8,
    benchmark_i8_ge_abs_natural
);
demo_and_bench!(
    u16,
    i16,
    demo_natural_lt_abs_u16,
    demo_u16_lt_abs_natural,
    demo_natural_lt_abs_i16,
    demo_i16_lt_abs_natural,
    demo_natural_gt_abs_u16,
    demo_u16_gt_abs_natural,
    demo_natural_gt_abs_i16,
    demo_i16_gt_abs_natural,
    demo_natural_le_abs_u16,
    demo_u16_le_abs_natural,
    demo_natural_le_abs_i16,
    demo_i16_le_abs_natural,
    demo_natural_ge_abs_u16,
    demo_u16_ge_abs_natural,
    demo_natural_ge_abs_i16,
    demo_i16_ge_abs_natural,
    benchmark_natural_lt_abs_u16,
    benchmark_u16_lt_abs_natural,
    benchmark_natural_lt_abs_i16,
    benchmark_i16_lt_abs_natural,
    benchmark_natural_gt_abs_u16,
    benchmark_u16_gt_abs_natural,
    benchmark_natural_gt_abs_i16,
    benchmark_i16_gt_abs_natural,
    benchmark_natural_le_abs_u16,
    benchmark_u16_le_abs_natural,
    benchmark_natural_le_abs_i16,
    benchmark_i16_le_abs_natural,
    benchmark_natural_ge_abs_u16,
    benchmark_u16_ge_abs_natural,
    benchmark_natural_ge_abs_i16,
    benchmark_i16_ge_abs_natural
);
demo_and_bench!(
    u32,
    i32,
    demo_natural_lt_abs_u32,
    demo_u32_lt_abs_natural,
    demo_natural_lt_abs_i32,
    demo_i32_lt_abs_natural,
    demo_natural_gt_abs_u32,
    demo_u32_gt_abs_natural,
    demo_natural_gt_abs_i32,
    demo_i32_gt_abs_natural,
    demo_natural_le_abs_u32,
    demo_u32_le_abs_natural,
    demo_natural_le_abs_i32,
    demo_i32_le_abs_natural,
    demo_natural_ge_abs_u32,
    demo_u32_ge_abs_natural,
    demo_natural_ge_abs_i32,
    demo_i32_ge_abs_natural,
    benchmark_natural_lt_abs_u32,
    benchmark_u32_lt_abs_natural,
    benchmark_natural_lt_abs_i32,
    benchmark_i32_lt_abs_natural,
    benchmark_natural_gt_abs_u32,
    benchmark_u32_gt_abs_natural,
    benchmark_natural_gt_abs_i32,
    benchmark_i32_gt_abs_natural,
    benchmark_natural_le_abs_u32,
    benchmark_u32_le_abs_natural,
    benchmark_natural_le_abs_i32,
    benchmark_i32_le_abs_natural,
    benchmark_natural_ge_abs_u32,
    benchmark_u32_ge_abs_natural,
    benchmark_natural_ge_abs_i32,
    benchmark_i32_ge_abs_natural
);
demo_and_bench!(
    u64,
    i64,
    demo_natural_lt_abs_u64,
    demo_u64_lt_abs_natural,
    demo_natural_lt_abs_i64,
    demo_i64_lt_abs_natural,
    demo_natural_gt_abs_u64,
    demo_u64_gt_abs_natural,
    demo_natural_gt_abs_i64,
    demo_i64_gt_abs_natural,
    demo_natural_le_abs_u64,
    demo_u64_le_abs_natural,
    demo_natural_le_abs_i64,
    demo_i64_le_abs_natural,
    demo_natural_ge_abs_u64,
    demo_u64_ge_abs_natural,
    demo_natural_ge_abs_i64,
    demo_i64_ge_abs_natural,
    benchmark_natural_lt_abs_u64,
    benchmark_u64_lt_abs_natural,
    benchmark_natural_lt_abs_i64,
    benchmark_i64_lt_abs_natural,
    benchmark_natural_gt_abs_u64,
    benchmark_u64_gt_abs_natural,
    benchmark_natural_gt_abs_i64,
    benchmark_i64_gt_abs_natural,
    benchmark_natural_le_abs_u64,
    benchmark_u64_le_abs_natural,
    benchmark_natural_le_abs_i64,
    benchmark_i64_le_abs_natural,
    benchmark_natural_ge_abs_u64,
    benchmark_u64_ge_abs_natural,
    benchmark_natural_ge_abs_i64,
    benchmark_i64_ge_abs_natural
);
demo_and_bench!(
    usize,
    isize,
    demo_natural_lt_abs_usize,
    demo_usize_lt_abs_natural,
    demo_natural_lt_abs_isize,
    demo_isize_lt_abs_natural,
    demo_natural_gt_abs_usize,
    demo_usize_gt_abs_natural,
    demo_natural_gt_abs_isize,
    demo_isize_gt_abs_natural,
    demo_natural_le_abs_usize,
    demo_usize_le_abs_natural,
    demo_natural_le_abs_isize,
    demo_isize_le_abs_natural,
    demo_natural_ge_abs_usize,
    demo_usize_ge_abs_natural,
    demo_natural_ge_abs_isize,
    demo_isize_ge_abs_natural,
    benchmark_natural_lt_abs_usize,
    benchmark_usize_lt_abs_natural,
    benchmark_natural_lt_abs_isize,
    benchmark_isize_lt_abs_natural,
    benchmark_natural_gt_abs_usize,
    benchmark_usize_gt_abs_natural,
    benchmark_natural_gt_abs_isize,
    benchmark_isize_gt_abs_natural,
    benchmark_natural_le_abs_usize,
    benchmark_usize_le_abs_natural,
    benchmark_natural_le_abs_isize,
    benchmark_isize_le_abs_natural,
    benchmark_natural_ge_abs_usize,
    benchmark_usize_ge_abs_natural,
    benchmark_natural_ge_abs_isize,
    benchmark_isize_ge_abs_natural
);
