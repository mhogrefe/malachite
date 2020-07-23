use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::ArithmeticCheckedShl;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_signed, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_small_signed, pairs_of_unsigned_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_u8_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_u16_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_u32_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_u64_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_usize_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_u8);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_u16);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_u32);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_u64);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_usize);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_i8_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_i16_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_i32_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_i64_arithmetic_checked_shl_isize);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_i8);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_i16);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_i32);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_i64);
    register_demo!(registry, demo_isize_arithmetic_checked_shl_isize);

    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_u64);
    register_bench!(
        registry,
        Large,
        benchmark_usize_arithmetic_checked_shl_usize
    );
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_u8_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_u16_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_u32_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_u64_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_usize_arithmetic_checked_shl_i64);
    register_bench!(
        registry,
        Large,
        benchmark_usize_arithmetic_checked_shl_isize
    );
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_u64);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_usize);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_u8);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_u16);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_u32);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_u64);
    register_bench!(
        registry,
        Large,
        benchmark_isize_arithmetic_checked_shl_usize
    );
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_i8_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_i16_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_i32_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_i64);
    register_bench!(registry, Large, benchmark_i64_arithmetic_checked_shl_isize);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_i8);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_i16);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_i32);
    register_bench!(registry, Large, benchmark_isize_arithmetic_checked_shl_i64);
    register_bench!(
        registry,
        Large,
        benchmark_isize_arithmetic_checked_shl_isize
    );
}

macro_rules! arithmetic_checked_shl_u_u {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shl:ident,
        $benchmark_arithmetic_checked_shl:ident
    ) => {
        fn $demo_arithmetic_checked_shl(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_unsigned_and_small_unsigned::<$t, $u>(gm).take(limit) {
                println!(
                    "{}.arithmetic_checked_shl({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shl(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark(
                &format!("{}.arithmetic_checked_shl({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_unsigned_and_small_unsigned::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shl(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shl_u_u!(
    u8,
    u8,
    demo_u8_arithmetic_checked_shl_u8,
    benchmark_u8_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_u_u!(
    u8,
    u16,
    demo_u8_arithmetic_checked_shl_u16,
    benchmark_u8_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_u_u!(
    u8,
    u32,
    demo_u8_arithmetic_checked_shl_u32,
    benchmark_u8_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_u_u!(
    u8,
    u64,
    demo_u8_arithmetic_checked_shl_u64,
    benchmark_u8_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_u_u!(
    u8,
    usize,
    demo_u8_arithmetic_checked_shl_usize,
    benchmark_u8_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_u_u!(
    u16,
    u8,
    demo_u16_arithmetic_checked_shl_u8,
    benchmark_u16_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_u_u!(
    u16,
    u16,
    demo_u16_arithmetic_checked_shl_u16,
    benchmark_u16_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_u_u!(
    u16,
    u32,
    demo_u16_arithmetic_checked_shl_u32,
    benchmark_u16_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_u_u!(
    u16,
    u64,
    demo_u16_arithmetic_checked_shl_u64,
    benchmark_u16_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_u_u!(
    u16,
    usize,
    demo_u16_arithmetic_checked_shl_usize,
    benchmark_u16_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_u_u!(
    u32,
    u8,
    demo_u32_arithmetic_checked_shl_u8,
    benchmark_u32_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_u_u!(
    u32,
    u16,
    demo_u32_arithmetic_checked_shl_u16,
    benchmark_u32_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_u_u!(
    u32,
    u32,
    demo_u32_arithmetic_checked_shl_u32,
    benchmark_u32_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_u_u!(
    u32,
    u64,
    demo_u32_arithmetic_checked_shl_u64,
    benchmark_u32_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_u_u!(
    u32,
    usize,
    demo_u32_arithmetic_checked_shl_usize,
    benchmark_u32_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_u_u!(
    u64,
    u8,
    demo_u64_arithmetic_checked_shl_u8,
    benchmark_u64_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_u_u!(
    u64,
    u16,
    demo_u64_arithmetic_checked_shl_u16,
    benchmark_u64_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_u_u!(
    u64,
    u32,
    demo_u64_arithmetic_checked_shl_u32,
    benchmark_u64_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_u_u!(
    u64,
    u64,
    demo_u64_arithmetic_checked_shl_u64,
    benchmark_u64_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_u_u!(
    u64,
    usize,
    demo_u64_arithmetic_checked_shl_usize,
    benchmark_u64_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_u_u!(
    usize,
    u8,
    demo_usize_arithmetic_checked_shl_u8,
    benchmark_usize_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_u_u!(
    usize,
    u16,
    demo_usize_arithmetic_checked_shl_u16,
    benchmark_usize_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_u_u!(
    usize,
    u32,
    demo_usize_arithmetic_checked_shl_u32,
    benchmark_usize_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_u_u!(
    usize,
    u64,
    demo_usize_arithmetic_checked_shl_u64,
    benchmark_usize_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_u_u!(
    usize,
    usize,
    demo_usize_arithmetic_checked_shl_usize,
    benchmark_usize_arithmetic_checked_shl_usize
);

macro_rules! arithmetic_checked_shl_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shl:ident,
        $benchmark_arithmetic_checked_shl:ident
    ) => {
        fn $demo_arithmetic_checked_shl(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_unsigned_and_small_signed::<$t, $u>(gm).take(limit) {
                println!(
                    "{}.arithmetic_checked_shl({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shl(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark(
                &format!("{}.arithmetic_checked_shl({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_unsigned_and_small_signed::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shl(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shl_u_i!(
    u8,
    i8,
    demo_u8_arithmetic_checked_shl_i8,
    benchmark_u8_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_u_i!(
    u8,
    i16,
    demo_u8_arithmetic_checked_shl_i16,
    benchmark_u8_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_u_i!(
    u8,
    i32,
    demo_u8_arithmetic_checked_shl_i32,
    benchmark_u8_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_u_i!(
    u8,
    i64,
    demo_u8_arithmetic_checked_shl_i64,
    benchmark_u8_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_u_i!(
    u8,
    isize,
    demo_u8_arithmetic_checked_shl_isize,
    benchmark_u8_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_u_i!(
    u16,
    i8,
    demo_u16_arithmetic_checked_shl_i8,
    benchmark_u16_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_u_i!(
    u16,
    i16,
    demo_u16_arithmetic_checked_shl_i16,
    benchmark_u16_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_u_i!(
    u16,
    i32,
    demo_u16_arithmetic_checked_shl_i32,
    benchmark_u16_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_u_i!(
    u16,
    i64,
    demo_u16_arithmetic_checked_shl_i64,
    benchmark_u16_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_u_i!(
    u16,
    isize,
    demo_u16_arithmetic_checked_shl_isize,
    benchmark_u16_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_u_i!(
    u32,
    i8,
    demo_u32_arithmetic_checked_shl_i8,
    benchmark_u32_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_u_i!(
    u32,
    i16,
    demo_u32_arithmetic_checked_shl_i16,
    benchmark_u32_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_u_i!(
    u32,
    i32,
    demo_u32_arithmetic_checked_shl_i32,
    benchmark_u32_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_u_i!(
    u32,
    i64,
    demo_u32_arithmetic_checked_shl_i64,
    benchmark_u32_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_u_i!(
    u32,
    isize,
    demo_u32_arithmetic_checked_shl_isize,
    benchmark_u32_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_u_i!(
    u64,
    i8,
    demo_u64_arithmetic_checked_shl_i8,
    benchmark_u64_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_u_i!(
    u64,
    i16,
    demo_u64_arithmetic_checked_shl_i16,
    benchmark_u64_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_u_i!(
    u64,
    i32,
    demo_u64_arithmetic_checked_shl_i32,
    benchmark_u64_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_u_i!(
    u64,
    i64,
    demo_u64_arithmetic_checked_shl_i64,
    benchmark_u64_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_u_i!(
    u64,
    isize,
    demo_u64_arithmetic_checked_shl_isize,
    benchmark_u64_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_u_i!(
    usize,
    i8,
    demo_usize_arithmetic_checked_shl_i8,
    benchmark_usize_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_u_i!(
    usize,
    i16,
    demo_usize_arithmetic_checked_shl_i16,
    benchmark_usize_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_u_i!(
    usize,
    i32,
    demo_usize_arithmetic_checked_shl_i32,
    benchmark_usize_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_u_i!(
    usize,
    i64,
    demo_usize_arithmetic_checked_shl_i64,
    benchmark_usize_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_u_i!(
    usize,
    isize,
    demo_usize_arithmetic_checked_shl_isize,
    benchmark_usize_arithmetic_checked_shl_isize
);

macro_rules! arithmetic_checked_shl_i_u {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shl:ident,
        $benchmark_arithmetic_checked_shl:ident
    ) => {
        fn $demo_arithmetic_checked_shl(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_signed_and_small_unsigned::<$t, $u>(gm).take(limit) {
                println!(
                    "({}).arithmetic_checked_shl({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shl(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark(
                &format!("{}.arithmetic_checked_shl({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_signed_and_small_unsigned::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shl(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shl_i_u!(
    i8,
    u8,
    demo_i8_arithmetic_checked_shl_u8,
    benchmark_i8_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_i_u!(
    i8,
    u16,
    demo_i8_arithmetic_checked_shl_u16,
    benchmark_i8_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_i_u!(
    i8,
    u32,
    demo_i8_arithmetic_checked_shl_u32,
    benchmark_i8_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_i_u!(
    i8,
    u64,
    demo_i8_arithmetic_checked_shl_u64,
    benchmark_i8_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_i_u!(
    i8,
    usize,
    demo_i8_arithmetic_checked_shl_usize,
    benchmark_i8_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_i_u!(
    i16,
    u8,
    demo_i16_arithmetic_checked_shl_u8,
    benchmark_i16_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_i_u!(
    i16,
    u16,
    demo_i16_arithmetic_checked_shl_u16,
    benchmark_i16_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_i_u!(
    i16,
    u32,
    demo_i16_arithmetic_checked_shl_u32,
    benchmark_i16_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_i_u!(
    i16,
    u64,
    demo_i16_arithmetic_checked_shl_u64,
    benchmark_i16_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_i_u!(
    i16,
    usize,
    demo_i16_arithmetic_checked_shl_usize,
    benchmark_i16_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_i_u!(
    i32,
    u8,
    demo_i32_arithmetic_checked_shl_u8,
    benchmark_i32_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_i_u!(
    i32,
    u16,
    demo_i32_arithmetic_checked_shl_u16,
    benchmark_i32_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_i_u!(
    i32,
    u32,
    demo_i32_arithmetic_checked_shl_u32,
    benchmark_i32_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_i_u!(
    i32,
    u64,
    demo_i32_arithmetic_checked_shl_u64,
    benchmark_i32_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_i_u!(
    i32,
    usize,
    demo_i32_arithmetic_checked_shl_usize,
    benchmark_i32_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_i_u!(
    i64,
    u8,
    demo_i64_arithmetic_checked_shl_u8,
    benchmark_i64_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_i_u!(
    i64,
    u16,
    demo_i64_arithmetic_checked_shl_u16,
    benchmark_i64_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_i_u!(
    i64,
    u32,
    demo_i64_arithmetic_checked_shl_u32,
    benchmark_i64_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_i_u!(
    i64,
    u64,
    demo_i64_arithmetic_checked_shl_u64,
    benchmark_i64_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_i_u!(
    i64,
    usize,
    demo_i64_arithmetic_checked_shl_usize,
    benchmark_i64_arithmetic_checked_shl_usize
);
arithmetic_checked_shl_i_u!(
    isize,
    u8,
    demo_isize_arithmetic_checked_shl_u8,
    benchmark_isize_arithmetic_checked_shl_u8
);
arithmetic_checked_shl_i_u!(
    isize,
    u16,
    demo_isize_arithmetic_checked_shl_u16,
    benchmark_isize_arithmetic_checked_shl_u16
);
arithmetic_checked_shl_i_u!(
    isize,
    u32,
    demo_isize_arithmetic_checked_shl_u32,
    benchmark_isize_arithmetic_checked_shl_u32
);
arithmetic_checked_shl_i_u!(
    isize,
    u64,
    demo_isize_arithmetic_checked_shl_u64,
    benchmark_isize_arithmetic_checked_shl_u64
);
arithmetic_checked_shl_i_u!(
    isize,
    usize,
    demo_isize_arithmetic_checked_shl_usize,
    benchmark_isize_arithmetic_checked_shl_usize
);

macro_rules! arithmetic_checked_shl_i_i {
    (
        $t:ident,
        $u:ident,
        $demo_arithmetic_checked_shl:ident,
        $benchmark_arithmetic_checked_shl:ident
    ) => {
        fn $demo_arithmetic_checked_shl(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_signed_and_small_signed::<$t, $u>(gm).take(limit) {
                println!(
                    "({}).arithmetic_checked_shl({}) = {:?}",
                    n,
                    u,
                    n.arithmetic_checked_shl(u)
                );
            }
        }

        fn $benchmark_arithmetic_checked_shl(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark(
                &format!("{}.arithmetic_checked_shl({})", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                pairs_of_signed_and_small_signed::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(x, y)| no_out!(x.arithmetic_checked_shl(y))),
                )],
            );
        }
    };
}
arithmetic_checked_shl_i_i!(
    i8,
    i8,
    demo_i8_arithmetic_checked_shl_i8,
    benchmark_i8_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_i_i!(
    i8,
    i16,
    demo_i8_arithmetic_checked_shl_i16,
    benchmark_i8_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_i_i!(
    i8,
    i32,
    demo_i8_arithmetic_checked_shl_i32,
    benchmark_i8_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_i_i!(
    i8,
    i64,
    demo_i8_arithmetic_checked_shl_i64,
    benchmark_i8_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_i_i!(
    i8,
    isize,
    demo_i8_arithmetic_checked_shl_isize,
    benchmark_i8_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_i_i!(
    i16,
    i8,
    demo_i16_arithmetic_checked_shl_i8,
    benchmark_i16_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_i_i!(
    i16,
    i16,
    demo_i16_arithmetic_checked_shl_i16,
    benchmark_i16_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_i_i!(
    i16,
    i32,
    demo_i16_arithmetic_checked_shl_i32,
    benchmark_i16_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_i_i!(
    i16,
    i64,
    demo_i16_arithmetic_checked_shl_i64,
    benchmark_i16_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_i_i!(
    i16,
    isize,
    demo_i16_arithmetic_checked_shl_isize,
    benchmark_i16_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_i_i!(
    i32,
    i8,
    demo_i32_arithmetic_checked_shl_i8,
    benchmark_i32_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_i_i!(
    i32,
    i16,
    demo_i32_arithmetic_checked_shl_i16,
    benchmark_i32_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_i_i!(
    i32,
    i32,
    demo_i32_arithmetic_checked_shl_i32,
    benchmark_i32_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_i_i!(
    i32,
    i64,
    demo_i32_arithmetic_checked_shl_i64,
    benchmark_i32_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_i_i!(
    i32,
    isize,
    demo_i32_arithmetic_checked_shl_isize,
    benchmark_i32_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_i_i!(
    i64,
    i8,
    demo_i64_arithmetic_checked_shl_i8,
    benchmark_i64_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_i_i!(
    i64,
    i16,
    demo_i64_arithmetic_checked_shl_i16,
    benchmark_i64_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_i_i!(
    i64,
    i32,
    demo_i64_arithmetic_checked_shl_i32,
    benchmark_i64_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_i_i!(
    i64,
    i64,
    demo_i64_arithmetic_checked_shl_i64,
    benchmark_i64_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_i_i!(
    i64,
    isize,
    demo_i64_arithmetic_checked_shl_isize,
    benchmark_i64_arithmetic_checked_shl_isize
);
arithmetic_checked_shl_i_i!(
    isize,
    i8,
    demo_isize_arithmetic_checked_shl_i8,
    benchmark_isize_arithmetic_checked_shl_i8
);
arithmetic_checked_shl_i_i!(
    isize,
    i16,
    demo_isize_arithmetic_checked_shl_i16,
    benchmark_isize_arithmetic_checked_shl_i16
);
arithmetic_checked_shl_i_i!(
    isize,
    i32,
    demo_isize_arithmetic_checked_shl_i32,
    benchmark_isize_arithmetic_checked_shl_i32
);
arithmetic_checked_shl_i_i!(
    isize,
    i64,
    demo_isize_arithmetic_checked_shl_i64,
    benchmark_isize_arithmetic_checked_shl_i64
);
arithmetic_checked_shl_i_i!(
    isize,
    isize,
    demo_isize_arithmetic_checked_shl_isize,
    benchmark_isize_arithmetic_checked_shl_isize
);
