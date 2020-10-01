use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_signed_nonzero_signed_and_rounding_mode_var_1,
    triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_div_round);
    register_demo!(registry, demo_u16_div_round);
    register_demo!(registry, demo_u32_div_round);
    register_demo!(registry, demo_u64_div_round);
    register_demo!(registry, demo_usize_div_round);
    register_demo!(registry, demo_i8_div_round);
    register_demo!(registry, demo_i16_div_round);
    register_demo!(registry, demo_i32_div_round);
    register_demo!(registry, demo_i64_div_round);
    register_demo!(registry, demo_isize_div_round);

    register_demo!(registry, demo_u8_div_round_assign);
    register_demo!(registry, demo_u16_div_round_assign);
    register_demo!(registry, demo_u32_div_round_assign);
    register_demo!(registry, demo_u64_div_round_assign);
    register_demo!(registry, demo_usize_div_round_assign);
    register_demo!(registry, demo_i8_div_round_assign);
    register_demo!(registry, demo_i16_div_round_assign);
    register_demo!(registry, demo_i32_div_round_assign);
    register_demo!(registry, demo_i64_div_round_assign);
    register_demo!(registry, demo_isize_div_round_assign);

    register_bench!(registry, None, benchmark_u8_div_round);
    register_bench!(registry, None, benchmark_u16_div_round);
    register_bench!(registry, None, benchmark_u32_div_round);
    register_bench!(registry, None, benchmark_u64_div_round);
    register_bench!(registry, None, benchmark_usize_div_round);
    register_bench!(registry, None, benchmark_i8_div_round);
    register_bench!(registry, None, benchmark_i16_div_round);
    register_bench!(registry, None, benchmark_i32_div_round);
    register_bench!(registry, None, benchmark_i64_div_round);
    register_bench!(registry, None, benchmark_isize_div_round);

    register_bench!(registry, None, benchmark_u8_div_round_assign);
    register_bench!(registry, None, benchmark_u16_div_round_assign);
    register_bench!(registry, None, benchmark_u32_div_round_assign);
    register_bench!(registry, None, benchmark_u64_div_round_assign);
    register_bench!(registry, None, benchmark_usize_div_round_assign);
    register_bench!(registry, None, benchmark_i8_div_round_assign);
    register_bench!(registry, None, benchmark_i16_div_round_assign);
    register_bench!(registry, None, benchmark_i32_div_round_assign);
    register_bench!(registry, None, benchmark_i64_div_round_assign);
    register_bench!(registry, None, benchmark_isize_div_round_assign);
}

fn demo_unsigned_div_round<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<T>(gm).take(limit)
    {
        println!("{}.div_round({}, {}) = {}", x, y, rm, x.div_round(y, rm));
    }
}

fn demo_signed_div_round<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, rm) in triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<T>(gm).take(limit)
    {
        println!("{}.div_round({}, {}) = {}", x, y, rm, x.div_round(y, rm));
    }
}

fn demo_unsigned_div_round_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y, rm) in
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<T>(gm).take(limit)
    {
        let old_x = x;
        x.div_round_assign(y, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            old_x, y, rm, x
        );
    }
}

fn demo_signed_div_round_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y, rm) in
        triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<T>(gm).take(limit)
    {
        let old_x = x;
        x.div_round_assign(y, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            old_x, y, rm, x
        );
    }
}

fn benchmark_unsigned_div_round<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.div_round({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))))],
    );
}

fn benchmark_signed_div_round<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.div_round({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("Malachite", &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))))],
    );
}

fn benchmark_unsigned_div_round_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.div_round_assign({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
        )],
    );
}

fn benchmark_signed_div_round_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    run_benchmark_old(
        &format!("{}.div_round_assign({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "Malachite",
            &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
        )],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_div_round::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_div_round_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_div_round::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_div_round_assign::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_div_round::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_signed_div_round_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_div_round::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_div_round_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_div_round,
    demo_u8_div_round_assign,
    benchmark_u8_div_round,
    benchmark_u8_div_round_assign
);
unsigned!(
    u16,
    demo_u16_div_round,
    demo_u16_div_round_assign,
    benchmark_u16_div_round,
    benchmark_u16_div_round_assign
);
unsigned!(
    u32,
    demo_u32_div_round,
    demo_u32_div_round_assign,
    benchmark_u32_div_round,
    benchmark_u32_div_round_assign
);
unsigned!(
    u64,
    demo_u64_div_round,
    demo_u64_div_round_assign,
    benchmark_u64_div_round,
    benchmark_u64_div_round_assign
);
unsigned!(
    usize,
    demo_usize_div_round,
    demo_usize_div_round_assign,
    benchmark_usize_div_round,
    benchmark_usize_div_round_assign
);

signed!(
    i8,
    demo_i8_div_round,
    demo_i8_div_round_assign,
    benchmark_i8_div_round,
    benchmark_i8_div_round_assign
);
signed!(
    i16,
    demo_i16_div_round,
    demo_i16_div_round_assign,
    benchmark_i16_div_round,
    benchmark_i16_div_round_assign
);
signed!(
    i32,
    demo_i32_div_round,
    demo_i32_div_round_assign,
    benchmark_i32_div_round,
    benchmark_i32_div_round_assign
);
signed!(
    i64,
    demo_i64_div_round,
    demo_i64_div_round_assign,
    benchmark_i64_div_round,
    benchmark_i64_div_round_assign
);
signed!(
    isize,
    demo_isize_div_round,
    demo_isize_div_round_assign,
    benchmark_isize_div_round,
    benchmark_isize_div_round_assign
);
