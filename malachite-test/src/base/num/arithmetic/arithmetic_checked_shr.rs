use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::ArithmeticCheckedShr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_signed, pairs_of_unsigned_and_small_signed,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_u8_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_u8_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_u8_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_u8_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_u16_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_u16_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_u16_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_u16_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_u16_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_u32_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_u32_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_u32_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_u32_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_u32_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_u64_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_u64_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_u64_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_u64_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_u64_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_usize_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_usize_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_usize_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_usize_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_usize_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_i8_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_i8_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_i8_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_i8_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_i8_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_i16_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_i16_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_i16_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_i16_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_i16_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_i32_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_i32_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_i32_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_i32_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_i32_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_i64_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_i64_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_i64_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_i64_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_i64_arithmetic_checked_shr_isize);
    register_demo!(registry, demo_isize_arithmetic_checked_shr_i8);
    register_demo!(registry, demo_isize_arithmetic_checked_shr_i16);
    register_demo!(registry, demo_isize_arithmetic_checked_shr_i32);
    register_demo!(registry, demo_isize_arithmetic_checked_shr_i64);
    register_demo!(registry, demo_isize_arithmetic_checked_shr_isize);

    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shr_i64);
    register_bench!(
        registry,
        Large,
        benchmark_usize_arithmetic_checked_shr_isize
    );
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shr_i64);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shr_isize);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shr_i8);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shr_i16);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shr_i32);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shr_i64);
    register_bench!(
        registry,
        Large,
        benchmark_isize_arithmetic_checked_shr_isize
    );
}

macro_rules! arithmetic_checked_shr_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shr:ident,
        $benchmark_arithmetic_checked_shr:ident
    ) => {
        fn $demo_arithmetic_checked_shr(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_unsigned_and_small_signed::<$t, $u>(gm).take(limit) {
                println!(
                    "{}.arithmetic_checked_shr({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shr(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shr(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}.arithmetic_checked_shr({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_unsigned_and_small_signed::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "Malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shr(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shr_u_i!(
    u8,
    i8,
    demo_u8_arithmetic_checked_shr_i8,
    benchmark_u8_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_u_i!(
    u8,
    i16,
    demo_u8_arithmetic_checked_shr_i16,
    benchmark_u8_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_u_i!(
    u8,
    i32,
    demo_u8_arithmetic_checked_shr_i32,
    benchmark_u8_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_u_i!(
    u8,
    i64,
    demo_u8_arithmetic_checked_shr_i64,
    benchmark_u8_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_u_i!(
    u8,
    isize,
    demo_u8_arithmetic_checked_shr_isize,
    benchmark_u8_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_u_i!(
    u16,
    i8,
    demo_u16_arithmetic_checked_shr_i8,
    benchmark_u16_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_u_i!(
    u16,
    i16,
    demo_u16_arithmetic_checked_shr_i16,
    benchmark_u16_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_u_i!(
    u16,
    i32,
    demo_u16_arithmetic_checked_shr_i32,
    benchmark_u16_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_u_i!(
    u16,
    i64,
    demo_u16_arithmetic_checked_shr_i64,
    benchmark_u16_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_u_i!(
    u16,
    isize,
    demo_u16_arithmetic_checked_shr_isize,
    benchmark_u16_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_u_i!(
    u32,
    i8,
    demo_u32_arithmetic_checked_shr_i8,
    benchmark_u32_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_u_i!(
    u32,
    i16,
    demo_u32_arithmetic_checked_shr_i16,
    benchmark_u32_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_u_i!(
    u32,
    i32,
    demo_u32_arithmetic_checked_shr_i32,
    benchmark_u32_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_u_i!(
    u32,
    i64,
    demo_u32_arithmetic_checked_shr_i64,
    benchmark_u32_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_u_i!(
    u32,
    isize,
    demo_u32_arithmetic_checked_shr_isize,
    benchmark_u32_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_u_i!(
    u64,
    i8,
    demo_u64_arithmetic_checked_shr_i8,
    benchmark_u64_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_u_i!(
    u64,
    i16,
    demo_u64_arithmetic_checked_shr_i16,
    benchmark_u64_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_u_i!(
    u64,
    i32,
    demo_u64_arithmetic_checked_shr_i32,
    benchmark_u64_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_u_i!(
    u64,
    i64,
    demo_u64_arithmetic_checked_shr_i64,
    benchmark_u64_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_u_i!(
    u64,
    isize,
    demo_u64_arithmetic_checked_shr_isize,
    benchmark_u64_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_u_i!(
    usize,
    i8,
    demo_usize_arithmetic_checked_shr_i8,
    benchmark_usize_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_u_i!(
    usize,
    i16,
    demo_usize_arithmetic_checked_shr_i16,
    benchmark_usize_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_u_i!(
    usize,
    i32,
    demo_usize_arithmetic_checked_shr_i32,
    benchmark_usize_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_u_i!(
    usize,
    i64,
    demo_usize_arithmetic_checked_shr_i64,
    benchmark_usize_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_u_i!(
    usize,
    isize,
    demo_usize_arithmetic_checked_shr_isize,
    benchmark_usize_arithmetic_checked_shr_isize
);

macro_rules! arithmetic_checked_shr_i_i {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shr:ident,
        $benchmark_arithmetic_checked_shr:ident
    ) => {
        fn $demo_arithmetic_checked_shr(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_signed_and_small_signed::<$t, $u>(gm).take(limit) {
                println!(
                    "({}).arithmetic_checked_shr({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shr(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shr(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}.arithmetic_checked_shr({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_signed_and_small_signed::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "Malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shr(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shr_i_i!(
    i8,
    i8,
    demo_i8_arithmetic_checked_shr_i8,
    benchmark_i8_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_i_i!(
    i8,
    i16,
    demo_i8_arithmetic_checked_shr_i16,
    benchmark_i8_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_i_i!(
    i8,
    i32,
    demo_i8_arithmetic_checked_shr_i32,
    benchmark_i8_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_i_i!(
    i8,
    i64,
    demo_i8_arithmetic_checked_shr_i64,
    benchmark_i8_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_i_i!(
    i8,
    isize,
    demo_i8_arithmetic_checked_shr_isize,
    benchmark_i8_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_i_i!(
    i16,
    i8,
    demo_i16_arithmetic_checked_shr_i8,
    benchmark_i16_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_i_i!(
    i16,
    i16,
    demo_i16_arithmetic_checked_shr_i16,
    benchmark_i16_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_i_i!(
    i16,
    i32,
    demo_i16_arithmetic_checked_shr_i32,
    benchmark_i16_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_i_i!(
    i16,
    i64,
    demo_i16_arithmetic_checked_shr_i64,
    benchmark_i16_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_i_i!(
    i16,
    isize,
    demo_i16_arithmetic_checked_shr_isize,
    benchmark_i16_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_i_i!(
    i32,
    i8,
    demo_i32_arithmetic_checked_shr_i8,
    benchmark_i32_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_i_i!(
    i32,
    i16,
    demo_i32_arithmetic_checked_shr_i16,
    benchmark_i32_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_i_i!(
    i32,
    i32,
    demo_i32_arithmetic_checked_shr_i32,
    benchmark_i32_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_i_i!(
    i32,
    i64,
    demo_i32_arithmetic_checked_shr_i64,
    benchmark_i32_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_i_i!(
    i32,
    isize,
    demo_i32_arithmetic_checked_shr_isize,
    benchmark_i32_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_i_i!(
    i64,
    i8,
    demo_i64_arithmetic_checked_shr_i8,
    benchmark_i64_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_i_i!(
    i64,
    i16,
    demo_i64_arithmetic_checked_shr_i16,
    benchmark_i64_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_i_i!(
    i64,
    i32,
    demo_i64_arithmetic_checked_shr_i32,
    benchmark_i64_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_i_i!(
    i64,
    i64,
    demo_i64_arithmetic_checked_shr_i64,
    benchmark_i64_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_i_i!(
    i64,
    isize,
    demo_i64_arithmetic_checked_shr_isize,
    benchmark_i64_arithmetic_checked_shr_isize
);
arithmetic_checked_shr_i_i!(
    isize,
    i8,
    demo_isize_arithmetic_checked_shr_i8,
    benchmark_isize_arithmetic_checked_shr_i8
);
arithmetic_checked_shr_i_i!(
    isize,
    i16,
    demo_isize_arithmetic_checked_shr_i16,
    benchmark_isize_arithmetic_checked_shr_i16
);
arithmetic_checked_shr_i_i!(
    isize,
    i32,
    demo_isize_arithmetic_checked_shr_i32,
    benchmark_isize_arithmetic_checked_shr_i32
);
arithmetic_checked_shr_i_i!(
    isize,
    i64,
    demo_isize_arithmetic_checked_shr_i64,
    benchmark_isize_arithmetic_checked_shr_i64
);
arithmetic_checked_shr_i_i!(
    isize,
    isize,
    demo_isize_arithmetic_checked_shr_isize,
    benchmark_isize_arithmetic_checked_shr_isize
);
