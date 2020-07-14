use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{pairs_of_signeds_var_4, pairs_of_unsigneds_var_7};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_div_exact);
    register_demo!(registry, demo_u16_div_exact);
    register_demo!(registry, demo_u32_div_exact);
    register_demo!(registry, demo_u64_div_exact);
    register_demo!(registry, demo_usize_div_exact);
    register_demo!(registry, demo_i8_div_exact);
    register_demo!(registry, demo_i16_div_exact);
    register_demo!(registry, demo_i32_div_exact);
    register_demo!(registry, demo_i64_div_exact);
    register_demo!(registry, demo_isize_div_exact);

    register_demo!(registry, demo_u8_div_exact_assign);
    register_demo!(registry, demo_u16_div_exact_assign);
    register_demo!(registry, demo_u32_div_exact_assign);
    register_demo!(registry, demo_u64_div_exact_assign);
    register_demo!(registry, demo_usize_div_exact_assign);
    register_demo!(registry, demo_i8_div_exact_assign);
    register_demo!(registry, demo_i16_div_exact_assign);
    register_demo!(registry, demo_i32_div_exact_assign);
    register_demo!(registry, demo_i64_div_exact_assign);
    register_demo!(registry, demo_isize_div_exact_assign);

    register_bench!(registry, None, benchmark_u8_div_exact);
    register_bench!(registry, None, benchmark_u16_div_exact);
    register_bench!(registry, None, benchmark_u32_div_exact);
    register_bench!(registry, None, benchmark_u64_div_exact);
    register_bench!(registry, None, benchmark_usize_div_exact);
    register_bench!(registry, None, benchmark_i8_div_exact);
    register_bench!(registry, None, benchmark_i16_div_exact);
    register_bench!(registry, None, benchmark_i32_div_exact);
    register_bench!(registry, None, benchmark_i64_div_exact);
    register_bench!(registry, None, benchmark_isize_div_exact);

    register_bench!(registry, None, benchmark_u8_div_exact_assign);
    register_bench!(registry, None, benchmark_u16_div_exact_assign);
    register_bench!(registry, None, benchmark_u32_div_exact_assign);
    register_bench!(registry, None, benchmark_u64_div_exact_assign);
    register_bench!(registry, None, benchmark_usize_div_exact_assign);
    register_bench!(registry, None, benchmark_i8_div_exact_assign);
    register_bench!(registry, None, benchmark_i16_div_exact_assign);
    register_bench!(registry, None, benchmark_i32_div_exact_assign);
    register_bench!(registry, None, benchmark_i64_div_exact_assign);
    register_bench!(registry, None, benchmark_isize_div_exact_assign);
}

fn demo_unsigned_div_exact<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigneds_var_7::<T>(gm).take(limit) {
        println!("{}.div_exact({}) = {}", x, y, x.div_exact(y));
    }
}

fn demo_signed_div_exact<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signeds_var_4::<T>(gm).take(limit) {
        println!("{}.div_exact({}) = {}", x, y, x.div_exact(y));
    }
}

fn demo_unsigned_div_exact_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigneds_var_7::<T>(gm).take(limit) {
        let old_x = x;
        x.div_exact_assign(y);
        println!("x := {}; x.div_exact_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_signed_div_exact_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signeds_var_4::<T>(gm).take(limit) {
        let old_x = x;
        x.div_exact_assign(y);
        println!("x := {}; x.div_exact_assign({}); x = {}", old_x, y, x);
    }
}

fn benchmark_unsigned_div_exact<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.div_exact({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_7::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.div_exact(y))))],
    );
}

fn benchmark_signed_div_exact<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.div_exact({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(x, y)| no_out!(x.div_exact(y))))],
    );
}

fn benchmark_unsigned_div_exact_assign<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.div_exact_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigneds_var_7::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.div_exact_assign(y)))],
    );
}

fn benchmark_signed_div_exact_assign<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.div_exact_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signeds_var_4::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| x.div_exact_assign(y)))],
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
            demo_unsigned_div_exact::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_div_exact_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_div_exact::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_div_exact_assign::<$t>(gm, limit, file_name);
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
            demo_signed_div_exact::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_signed_div_exact_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_div_exact::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_div_exact_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_div_exact,
    demo_u8_div_exact_assign,
    benchmark_u8_div_exact,
    benchmark_u8_div_exact_assign
);
unsigned!(
    u16,
    demo_u16_div_exact,
    demo_u16_div_exact_assign,
    benchmark_u16_div_exact,
    benchmark_u16_div_exact_assign
);
unsigned!(
    u32,
    demo_u32_div_exact,
    demo_u32_div_exact_assign,
    benchmark_u32_div_exact,
    benchmark_u32_div_exact_assign
);
unsigned!(
    u64,
    demo_u64_div_exact,
    demo_u64_div_exact_assign,
    benchmark_u64_div_exact,
    benchmark_u64_div_exact_assign
);
unsigned!(
    usize,
    demo_usize_div_exact,
    demo_usize_div_exact_assign,
    benchmark_usize_div_exact,
    benchmark_usize_div_exact_assign
);

signed!(
    i8,
    demo_i8_div_exact,
    demo_i8_div_exact_assign,
    benchmark_i8_div_exact,
    benchmark_i8_div_exact_assign
);
signed!(
    i16,
    demo_i16_div_exact,
    demo_i16_div_exact_assign,
    benchmark_i16_div_exact,
    benchmark_i16_div_exact_assign
);
signed!(
    i32,
    demo_i32_div_exact,
    demo_i32_div_exact_assign,
    benchmark_i32_div_exact,
    benchmark_i32_div_exact_assign
);
signed!(
    i64,
    demo_i64_div_exact,
    demo_i64_div_exact_assign,
    benchmark_i64_div_exact,
    benchmark_i64_div_exact_assign
);
signed!(
    isize,
    demo_isize_div_exact,
    demo_isize_div_exact_assign,
    benchmark_isize_div_exact,
    benchmark_isize_div_exact_assign
);
