use std::cmp::max;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, WrappingFrom,
};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_signed_signed_and_rounding_mode_var_1,
    triples_of_unsigned_unsigned_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_round_to_multiple);
    register_demo!(registry, demo_u16_round_to_multiple);
    register_demo!(registry, demo_u32_round_to_multiple);
    register_demo!(registry, demo_u64_round_to_multiple);
    register_demo!(registry, demo_usize_round_to_multiple);
    register_demo!(registry, demo_i8_round_to_multiple);
    register_demo!(registry, demo_i16_round_to_multiple);
    register_demo!(registry, demo_i32_round_to_multiple);
    register_demo!(registry, demo_i64_round_to_multiple);
    register_demo!(registry, demo_isize_round_to_multiple);

    register_demo!(registry, demo_u8_round_to_multiple_assign);
    register_demo!(registry, demo_u16_round_to_multiple_assign);
    register_demo!(registry, demo_u32_round_to_multiple_assign);
    register_demo!(registry, demo_u64_round_to_multiple_assign);
    register_demo!(registry, demo_usize_round_to_multiple_assign);
    register_demo!(registry, demo_i8_round_to_multiple_assign);
    register_demo!(registry, demo_i16_round_to_multiple_assign);
    register_demo!(registry, demo_i32_round_to_multiple_assign);
    register_demo!(registry, demo_i64_round_to_multiple_assign);
    register_demo!(registry, demo_isize_round_to_multiple_assign);

    register_bench!(registry, None, benchmark_u8_round_to_multiple);
    register_bench!(registry, None, benchmark_u16_round_to_multiple);
    register_bench!(registry, None, benchmark_u32_round_to_multiple);
    register_bench!(registry, None, benchmark_u64_round_to_multiple);
    register_bench!(registry, None, benchmark_usize_round_to_multiple);
    register_bench!(registry, None, benchmark_i8_round_to_multiple);
    register_bench!(registry, None, benchmark_i16_round_to_multiple);
    register_bench!(registry, None, benchmark_i32_round_to_multiple);
    register_bench!(registry, None, benchmark_i64_round_to_multiple);
    register_bench!(registry, None, benchmark_isize_round_to_multiple);

    register_bench!(registry, None, benchmark_u8_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_u16_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_u32_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_u64_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_usize_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_i8_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_i16_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_i32_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_i64_round_to_multiple_assign);
    register_bench!(registry, None, benchmark_isize_round_to_multiple_assign);
}

fn demo_unsigned_round_to_multiple<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_unsigned_unsigned_and_rounding_mode_var_1::<T>(gm).take(limit) {
        println!(
            "{}.round_to_multiple({}, {}) = {}",
            x,
            y,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_unsigned_round_to_multiple_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
) {
    for (mut x, y, rm) in triples_of_unsigned_unsigned_and_rounding_mode_var_1::<T>(gm).take(limit)
    {
        let old_x = x;
        x.round_to_multiple_assign(y, rm);
        println!(
            "x := {}; x.round_to_multiple({}, {}); x = {}",
            old_x, y, rm, x
        );
    }
}

fn demo_signed_round_to_multiple<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    for (x, y, rm) in triples_of_signed_signed_and_rounding_mode_var_1::<T>(gm).take(limit) {
        println!(
            "{}.round_to_multiple({}, {}) = {}",
            x,
            y,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_signed_round_to_multiple_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    for (mut x, y, rm) in triples_of_signed_signed_and_rounding_mode_var_1::<T>(gm).take(limit) {
        let old_x = x;
        x.round_to_multiple_assign(y, rm);
        println!(
            "x := {}; x.round_to_multiple({}, {}); x = {}",
            old_x, y, rm, x
        );
    }
}

fn benchmark_unsigned_round_to_multiple<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.round_to_multiple({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(x, y, rm)| no_out!(x.round_to_multiple(y, rm))),
        )],
    );
}

fn benchmark_unsigned_round_to_multiple_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_assign({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(mut x, y, rm)| x.round_to_multiple_assign(y, rm)),
        )],
    );
}

fn benchmark_signed_round_to_multiple<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    run_benchmark_old(
        &format!("{}.round_to_multiple({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signed_signed_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(x, y, rm)| no_out!(x.round_to_multiple(y, rm))),
        )],
    );
}

fn benchmark_signed_round_to_multiple_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    run_benchmark_old(
        &format!(
            "{}.round_to_multiple_assign({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        triples_of_signed_signed_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(mut x, y, rm)| x.round_to_multiple_assign(y, rm)),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name_floor:ident,
        $demo_name_ceiling:ident,
        $bench_name_floor:ident,
        $bench_name_ceiling:ident
    ) => {
        fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_unsigned_round_to_multiple::<$t>(gm, limit);
        }

        fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_unsigned_round_to_multiple_assign::<$t>(gm, limit);
        }

        fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_round_to_multiple::<$t>(gm, limit, file_name);
        }

        fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_round_to_multiple_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name_floor:ident,
        $demo_name_ceiling:ident,
        $bench_name_floor:ident,
        $bench_name_ceiling:ident
    ) => {
        fn $demo_name_floor(gm: GenerationMode, limit: usize) {
            demo_signed_round_to_multiple::<$t>(gm, limit);
        }

        fn $demo_name_ceiling(gm: GenerationMode, limit: usize) {
            demo_signed_round_to_multiple_assign::<$t>(gm, limit);
        }

        fn $bench_name_floor(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_round_to_multiple::<$t>(gm, limit, file_name);
        }

        fn $bench_name_ceiling(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_round_to_multiple_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_round_to_multiple,
    demo_u8_round_to_multiple_assign,
    benchmark_u8_round_to_multiple,
    benchmark_u8_round_to_multiple_assign
);
unsigned!(
    u16,
    demo_u16_round_to_multiple,
    demo_u16_round_to_multiple_assign,
    benchmark_u16_round_to_multiple,
    benchmark_u16_round_to_multiple_assign
);
unsigned!(
    u32,
    demo_u32_round_to_multiple,
    demo_u32_round_to_multiple_assign,
    benchmark_u32_round_to_multiple,
    benchmark_u32_round_to_multiple_assign
);
unsigned!(
    u64,
    demo_u64_round_to_multiple,
    demo_u64_round_to_multiple_assign,
    benchmark_u64_round_to_multiple,
    benchmark_u64_round_to_multiple_assign
);
unsigned!(
    usize,
    demo_usize_round_to_multiple,
    demo_usize_round_to_multiple_assign,
    benchmark_usize_round_to_multiple,
    benchmark_usize_round_to_multiple_assign
);

signed!(
    i8,
    demo_i8_round_to_multiple,
    demo_i8_round_to_multiple_assign,
    benchmark_i8_round_to_multiple,
    benchmark_i8_round_to_multiple_assign
);
signed!(
    i16,
    demo_i16_round_to_multiple,
    demo_i16_round_to_multiple_assign,
    benchmark_i16_round_to_multiple,
    benchmark_i16_round_to_multiple_assign
);
signed!(
    i32,
    demo_i32_round_to_multiple,
    demo_i32_round_to_multiple_assign,
    benchmark_i32_round_to_multiple,
    benchmark_i32_round_to_multiple_assign
);
signed!(
    i64,
    demo_i64_round_to_multiple,
    demo_i64_round_to_multiple_assign,
    benchmark_i64_round_to_multiple,
    benchmark_i64_round_to_multiple_assign
);
signed!(
    isize,
    demo_isize_round_to_multiple,
    demo_isize_round_to_multiple_assign,
    benchmark_isize_round_to_multiple,
    benchmark_isize_round_to_multiple_assign
);
