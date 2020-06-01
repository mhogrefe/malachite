use std::cmp::max;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_signed_and_nonzero_signed, pairs_of_signeds_var_2,
    pairs_of_unsigned_and_positive_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_mod);
    register_demo!(registry, demo_u16_mod);
    register_demo!(registry, demo_u32_mod);
    register_demo!(registry, demo_u64_mod);
    register_demo!(registry, demo_usize_mod);
    register_demo!(registry, demo_i8_mod);
    register_demo!(registry, demo_i16_mod);
    register_demo!(registry, demo_i32_mod);
    register_demo!(registry, demo_i64_mod);
    register_demo!(registry, demo_isize_mod);

    register_demo!(registry, demo_u8_mod_assign);
    register_demo!(registry, demo_u16_mod_assign);
    register_demo!(registry, demo_u32_mod_assign);
    register_demo!(registry, demo_u64_mod_assign);
    register_demo!(registry, demo_usize_mod_assign);
    register_demo!(registry, demo_i8_mod_assign);
    register_demo!(registry, demo_i16_mod_assign);
    register_demo!(registry, demo_i32_mod_assign);
    register_demo!(registry, demo_i64_mod_assign);
    register_demo!(registry, demo_isize_mod_assign);

    register_demo!(registry, demo_u8_neg_mod);
    register_demo!(registry, demo_u16_neg_mod);
    register_demo!(registry, demo_u32_neg_mod);
    register_demo!(registry, demo_u64_neg_mod);
    register_demo!(registry, demo_usize_neg_mod);

    register_demo!(registry, demo_u8_neg_mod_assign);
    register_demo!(registry, demo_u16_neg_mod_assign);
    register_demo!(registry, demo_u32_neg_mod_assign);
    register_demo!(registry, demo_u64_neg_mod_assign);
    register_demo!(registry, demo_usize_neg_mod_assign);

    register_demo!(registry, demo_i8_ceiling_mod);
    register_demo!(registry, demo_i16_ceiling_mod);
    register_demo!(registry, demo_i32_ceiling_mod);
    register_demo!(registry, demo_i64_ceiling_mod);
    register_demo!(registry, demo_isize_ceiling_mod);

    register_demo!(registry, demo_i8_ceiling_mod_assign);
    register_demo!(registry, demo_i16_ceiling_mod_assign);
    register_demo!(registry, demo_i32_ceiling_mod_assign);
    register_demo!(registry, demo_i64_ceiling_mod_assign);
    register_demo!(registry, demo_isize_ceiling_mod_assign);

    register_bench!(registry, None, benchmark_u8_mod_algorithms);
    register_bench!(registry, None, benchmark_u16_mod_algorithms);
    register_bench!(registry, None, benchmark_u32_mod_algorithms);
    register_bench!(registry, None, benchmark_u64_mod_algorithms);
    register_bench!(registry, None, benchmark_usize_mod_algorithms);
    register_bench!(registry, None, benchmark_i8_mod_algorithms);
    register_bench!(registry, None, benchmark_i16_mod_algorithms);
    register_bench!(registry, None, benchmark_i32_mod_algorithms);
    register_bench!(registry, None, benchmark_i64_mod_algorithms);
    register_bench!(registry, None, benchmark_isize_mod_algorithms);

    register_bench!(registry, None, benchmark_u8_mod_assign);
    register_bench!(registry, None, benchmark_u16_mod_assign);
    register_bench!(registry, None, benchmark_u32_mod_assign);
    register_bench!(registry, None, benchmark_u64_mod_assign);
    register_bench!(registry, None, benchmark_usize_mod_assign);
    register_bench!(registry, None, benchmark_i8_mod_assign);
    register_bench!(registry, None, benchmark_i16_mod_assign);
    register_bench!(registry, None, benchmark_i32_mod_assign);
    register_bench!(registry, None, benchmark_i64_mod_assign);
    register_bench!(registry, None, benchmark_isize_mod_assign);

    register_bench!(registry, None, benchmark_u8_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u16_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u32_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_u64_neg_mod_algorithms);
    register_bench!(registry, None, benchmark_usize_neg_mod_algorithms);

    register_bench!(registry, None, benchmark_u8_neg_mod_assign);
    register_bench!(registry, None, benchmark_u16_neg_mod_assign);
    register_bench!(registry, None, benchmark_u32_neg_mod_assign);
    register_bench!(registry, None, benchmark_u64_neg_mod_assign);
    register_bench!(registry, None, benchmark_usize_neg_mod_assign);

    register_bench!(registry, None, benchmark_i8_ceiling_mod_algorithms);
    register_bench!(registry, None, benchmark_i16_ceiling_mod_algorithms);
    register_bench!(registry, None, benchmark_i32_ceiling_mod_algorithms);
    register_bench!(registry, None, benchmark_i64_ceiling_mod_algorithms);
    register_bench!(registry, None, benchmark_isize_ceiling_mod_algorithms);

    register_bench!(registry, None, benchmark_i8_ceiling_mod_assign);
    register_bench!(registry, None, benchmark_i16_ceiling_mod_assign);
    register_bench!(registry, None, benchmark_i32_ceiling_mod_assign);
    register_bench!(registry, None, benchmark_i64_ceiling_mod_assign);
    register_bench!(registry, None, benchmark_isize_ceiling_mod_assign);
}

fn demo_mod_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        println!("{}.mod_op({}) = {}", x, y, x.mod_op(y));
    }
}

fn demo_mod_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_nonzero_signed::<T, T>(gm).take(limit) {
        println!("({}).mod({}) = {}", x, y, x.mod_op(y));
    }
}

fn demo_mod_assign_unsigned<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        x.mod_assign(y);
        println!("x := {}; x.mod_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_mod_assign_signed<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signed_and_nonzero_signed::<T, T>(gm).take(limit) {
        let old_x = x;
        x.mod_assign(y);
        println!("x := {}; x.mod_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_neg_mod<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        println!("{}.neg_mod({}) = {}", x, y, x.neg_mod(y));
    }
}

fn demo_neg_mod_assign<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_unsigned_and_positive_unsigned::<T, T>(gm).take(limit) {
        let old_x = x;
        x.neg_mod_assign(y);
        println!("x := {}; x.neg_mod_assign({}); x = {}", old_x, y, x);
    }
}

fn demo_ceiling_mod<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y) in pairs_of_signed_and_nonzero_signed::<T, T>(gm).take(limit) {
        println!("({}).ceiling_mod({}) = {}", x, y, x.ceiling_mod(y));
    }
}

fn demo_ceiling_mod_assign<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut x, y) in pairs_of_signed_and_nonzero_signed::<T, T>(gm).take(limit) {
        let old_x = x;
        x.ceiling_mod_assign(y);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", old_x, y, x);
    }
}

fn benchmark_mod_unsigned_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_op({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using mod", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_mod_signed_algorithms<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.mod_op({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using mod", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_mod_assign_unsigned<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| no_out!(x.mod_assign(y))))],
    );
}

fn benchmark_mod_assign_signed<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_nonzero_signed::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [("malachite", &mut (|(mut x, y)| no_out!(x.mod_assign(y))))],
    );
}

fn benchmark_neg_mod_algorithms<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("using neg_mod", &mut (|(x, y)| no_out!(x.neg_mod(y)))),
            (
                "using ceiling_div_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_ceiling_mod_algorithms<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.ceiling_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        pairs_of_signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "using ceiling_mod",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "using ceiling_div_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_neg_mod_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.neg_mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_positive_unsigned::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.neg_mod_assign(y))),
        )],
    );
}

fn benchmark_ceiling_mod_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.ceiling_mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_nonzero_signed::<T, T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x, y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.ceiling_mod_assign(y))),
        )],
    );
}

macro_rules! mod_unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_unsigned::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_assign_unsigned::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_unsigned_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_assign_unsigned::<$t>(gm, limit, file_name);
        }
    };
}

mod_unsigned!(
    u8,
    demo_u8_mod,
    demo_u8_mod_assign,
    benchmark_u8_mod_algorithms,
    benchmark_u8_mod_assign
);
mod_unsigned!(
    u16,
    demo_u16_mod,
    demo_u16_mod_assign,
    benchmark_u16_mod_algorithms,
    benchmark_u16_mod_assign
);
mod_unsigned!(
    u32,
    demo_u32_mod,
    demo_u32_mod_assign,
    benchmark_u32_mod_algorithms,
    benchmark_u32_mod_assign
);
mod_unsigned!(
    u64,
    demo_u64_mod,
    demo_u64_mod_assign,
    benchmark_u64_mod_algorithms,
    benchmark_u64_mod_assign
);
mod_unsigned!(
    usize,
    demo_usize_mod,
    demo_usize_mod_assign,
    benchmark_usize_mod_algorithms,
    benchmark_usize_mod_assign
);

macro_rules! mod_signed {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_mod_signed::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_mod_assign_signed::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_signed_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_mod_assign_signed::<$t>(gm, limit, file_name);
        }
    };
}

mod_signed!(
    i8,
    demo_i8_mod,
    demo_i8_mod_assign,
    benchmark_i8_mod_algorithms,
    benchmark_i8_mod_assign
);
mod_signed!(
    i16,
    demo_i16_mod,
    demo_i16_mod_assign,
    benchmark_i16_mod_algorithms,
    benchmark_i16_mod_assign
);
mod_signed!(
    i32,
    demo_i32_mod,
    demo_i32_mod_assign,
    benchmark_i32_mod_algorithms,
    benchmark_i32_mod_assign
);
mod_signed!(
    i64,
    demo_i64_mod,
    demo_i64_mod_assign,
    benchmark_i64_mod_algorithms,
    benchmark_i64_mod_assign
);
mod_signed!(
    isize,
    demo_isize_mod,
    demo_isize_mod_assign,
    benchmark_isize_mod_algorithms,
    benchmark_isize_mod_assign
);

macro_rules! neg_mod {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_neg_mod::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_neg_mod_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_neg_mod_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_neg_mod_assign::<$t>(gm, limit, file_name);
        }
    };
}

neg_mod!(
    u8,
    demo_u8_neg_mod,
    demo_u8_neg_mod_assign,
    benchmark_u8_neg_mod_algorithms,
    benchmark_u8_neg_mod_assign
);
neg_mod!(
    u16,
    demo_u16_neg_mod,
    demo_u16_neg_mod_assign,
    benchmark_u16_neg_mod_algorithms,
    benchmark_u16_neg_mod_assign
);
neg_mod!(
    u32,
    demo_u32_neg_mod,
    demo_u32_neg_mod_assign,
    benchmark_u32_neg_mod_algorithms,
    benchmark_u32_neg_mod_assign
);
neg_mod!(
    u64,
    demo_u64_neg_mod,
    demo_u64_neg_mod_assign,
    benchmark_u64_neg_mod_algorithms,
    benchmark_u64_neg_mod_assign
);
neg_mod!(
    usize,
    demo_usize_neg_mod,
    demo_usize_neg_mod_assign,
    benchmark_usize_neg_mod_algorithms,
    benchmark_usize_neg_mod_assign
);

macro_rules! ceiling_mod {
    (
        $t:ident,
        $demo_name:ident,
        $demo_assign_name:ident,
        $bench_name:ident,
        $bench_assign_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_mod::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: GenerationMode, limit: usize) {
            demo_ceiling_mod_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_mod_algorithms::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_ceiling_mod_assign::<$t>(gm, limit, file_name);
        }
    };
}

ceiling_mod!(
    i8,
    demo_i8_ceiling_mod,
    demo_i8_ceiling_mod_assign,
    benchmark_i8_ceiling_mod_algorithms,
    benchmark_i8_ceiling_mod_assign
);
ceiling_mod!(
    i16,
    demo_i16_ceiling_mod,
    demo_i16_ceiling_mod_assign,
    benchmark_i16_ceiling_mod_algorithms,
    benchmark_i16_ceiling_mod_assign
);
ceiling_mod!(
    i32,
    demo_i32_ceiling_mod,
    demo_i32_ceiling_mod_assign,
    benchmark_i32_ceiling_mod_algorithms,
    benchmark_i32_ceiling_mod_assign
);
ceiling_mod!(
    i64,
    demo_i64_ceiling_mod,
    demo_i64_ceiling_mod_assign,
    benchmark_i64_ceiling_mod_algorithms,
    benchmark_i64_ceiling_mod_assign
);
ceiling_mod!(
    isize,
    demo_isize_ceiling_mod,
    demo_isize_ceiling_mod_assign,
    benchmark_isize_ceiling_mod_algorithms,
    benchmark_isize_ceiling_mod_assign
);
