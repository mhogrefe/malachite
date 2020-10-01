use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{signeds_var_2, unsigneds_var_8};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_u8_square);
    register_ns_demo!(registry, demo_u16_square);
    register_ns_demo!(registry, demo_u32_square);
    register_ns_demo!(registry, demo_u64_square);
    register_ns_demo!(registry, demo_usize_square);
    register_ns_demo!(registry, demo_i8_square);
    register_ns_demo!(registry, demo_i16_square);
    register_ns_demo!(registry, demo_i32_square);
    register_ns_demo!(registry, demo_i64_square);
    register_ns_demo!(registry, demo_isize_square);

    register_ns_demo!(registry, demo_u8_square_assign);
    register_ns_demo!(registry, demo_u16_square_assign);
    register_ns_demo!(registry, demo_u32_square_assign);
    register_ns_demo!(registry, demo_u64_square_assign);
    register_ns_demo!(registry, demo_usize_square_assign);
    register_ns_demo!(registry, demo_i8_square_assign);
    register_ns_demo!(registry, demo_i16_square_assign);
    register_ns_demo!(registry, demo_i32_square_assign);
    register_ns_demo!(registry, demo_i64_square_assign);
    register_ns_demo!(registry, demo_isize_square_assign);

    register_ns_bench!(registry, None, benchmark_u8_square);
    register_ns_bench!(registry, None, benchmark_u16_square);
    register_ns_bench!(registry, None, benchmark_u32_square);
    register_ns_bench!(registry, None, benchmark_u64_square);
    register_ns_bench!(registry, None, benchmark_usize_square);
    register_ns_bench!(registry, None, benchmark_i8_square);
    register_ns_bench!(registry, None, benchmark_i16_square);
    register_ns_bench!(registry, None, benchmark_i32_square);
    register_ns_bench!(registry, None, benchmark_i64_square);
    register_ns_bench!(registry, None, benchmark_isize_square);

    register_ns_bench!(registry, None, benchmark_u8_square_assign);
    register_ns_bench!(registry, None, benchmark_u16_square_assign);
    register_ns_bench!(registry, None, benchmark_u32_square_assign);
    register_ns_bench!(registry, None, benchmark_u64_square_assign);
    register_ns_bench!(registry, None, benchmark_usize_square_assign);
    register_ns_bench!(registry, None, benchmark_i8_square_assign);
    register_ns_bench!(registry, None, benchmark_i16_square_assign);
    register_ns_bench!(registry, None, benchmark_i32_square_assign);
    register_ns_bench!(registry, None, benchmark_i64_square_assign);
    register_ns_bench!(registry, None, benchmark_isize_square_assign);
}

fn demo_unsigned_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for x in unsigneds_var_8::<T>(gm).take(limit) {
        println!("{}.square() = {}", x, x.square());
    }
}

fn demo_unsigned_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for mut x in unsigneds_var_8::<T>(gm).take(limit) {
        let old_x = x;
        x.square_assign();
        println!("x := {}; x.square_assign(); x = {}", old_x, x);
    }
}

fn demo_signed_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for x in signeds_var_2::<T>(gm).take(limit) {
        println!("{}.square() = {}", x, x.square());
    }
}

fn demo_signed_square_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
) {
    for mut x in signeds_var_2::<T>(gm).take(limit) {
        let old_x = x;
        x.square_assign();
        println!("x := {}; x.square_assign(); x = {}", old_x, x);
    }
}

fn benchmark_unsigned_square<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.square()", T::NAME),
        BenchmarkType::Single,
        unsigneds_var_8::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|x| no_out!(x.square())))],
    );
}

fn benchmark_unsigned_square_assign<T: PrimitiveUnsigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.square_assign()", T::NAME),
        BenchmarkType::Single,
        unsigneds_var_8::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|mut x| x.square_assign()))],
    );
}

fn benchmark_signed_square<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.square()", T::NAME),
        BenchmarkType::Single,
        signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|x| no_out!(x.square())))],
    );
}

fn benchmark_signed_square_assign<T: PrimitiveSigned + Rand + SampleRange>(
    gm: NoSpecialGenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        &format!("{}.square_assign()", T::NAME),
        BenchmarkType::Single,
        signeds_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|x| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [("Malachite", &mut (|mut x| x.square_assign()))],
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
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_unsigned_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_unsigned_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_square_assign::<$t>(gm, limit, file_name);
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
        fn $demo_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_signed_square::<$t>(gm, limit);
        }

        fn $demo_assign_name(gm: NoSpecialGenerationMode, limit: usize) {
            demo_signed_square_assign::<$t>(gm, limit);
        }

        fn $bench_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_square::<$t>(gm, limit, file_name);
        }

        fn $bench_assign_name(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_square_assign::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(
    u8,
    demo_u8_square,
    demo_u8_square_assign,
    benchmark_u8_square,
    benchmark_u8_square_assign
);
unsigned!(
    u16,
    demo_u16_square,
    demo_u16_square_assign,
    benchmark_u16_square,
    benchmark_u16_square_assign
);
unsigned!(
    u32,
    demo_u32_square,
    demo_u32_square_assign,
    benchmark_u32_square,
    benchmark_u32_square_assign
);
unsigned!(
    u64,
    demo_u64_square,
    demo_u64_square_assign,
    benchmark_u64_square,
    benchmark_u64_square_assign
);
unsigned!(
    usize,
    demo_usize_square,
    demo_usize_square_assign,
    benchmark_usize_square,
    benchmark_usize_square_assign
);

signed!(
    i8,
    demo_i8_square,
    demo_i8_square_assign,
    benchmark_i8_square,
    benchmark_i8_square_assign
);
signed!(
    i16,
    demo_i16_square,
    demo_i16_square_assign,
    benchmark_i16_square,
    benchmark_i16_square_assign
);
signed!(
    i32,
    demo_i32_square,
    demo_i32_square_assign,
    benchmark_i32_square,
    benchmark_i32_square_assign
);
signed!(
    i64,
    demo_i64_square,
    demo_i64_square_assign,
    benchmark_i64_square,
    benchmark_i64_square_assign
);
signed!(
    isize,
    demo_isize_square,
    demo_isize_square_assign,
    benchmark_isize_square,
    benchmark_isize_square_assign
);
