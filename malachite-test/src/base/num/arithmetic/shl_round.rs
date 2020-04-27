use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign};
use malachite_base::num::conversion::traits::ExactFrom;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_signed_small_signed_and_rounding_mode_var_2,
    triples_of_unsigned_small_signed_and_rounding_mode_var_2,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_shl_round_assign_i8);
    register_demo!(registry, demo_u8_shl_round_assign_i16);
    register_demo!(registry, demo_u8_shl_round_assign_i32);
    register_demo!(registry, demo_u8_shl_round_assign_i64);
    register_demo!(registry, demo_u8_shl_round_assign_isize);
    register_demo!(registry, demo_u16_shl_round_assign_i8);
    register_demo!(registry, demo_u16_shl_round_assign_i16);
    register_demo!(registry, demo_u16_shl_round_assign_i32);
    register_demo!(registry, demo_u16_shl_round_assign_i64);
    register_demo!(registry, demo_u16_shl_round_assign_isize);
    register_demo!(registry, demo_u32_shl_round_assign_i8);
    register_demo!(registry, demo_u32_shl_round_assign_i16);
    register_demo!(registry, demo_u32_shl_round_assign_i32);
    register_demo!(registry, demo_u32_shl_round_assign_i64);
    register_demo!(registry, demo_u32_shl_round_assign_isize);
    register_demo!(registry, demo_u64_shl_round_assign_i8);
    register_demo!(registry, demo_u64_shl_round_assign_i16);
    register_demo!(registry, demo_u64_shl_round_assign_i32);
    register_demo!(registry, demo_u64_shl_round_assign_i64);
    register_demo!(registry, demo_u64_shl_round_assign_isize);
    register_demo!(registry, demo_usize_shl_round_assign_i8);
    register_demo!(registry, demo_usize_shl_round_assign_i16);
    register_demo!(registry, demo_usize_shl_round_assign_i32);
    register_demo!(registry, demo_usize_shl_round_assign_i64);
    register_demo!(registry, demo_usize_shl_round_assign_isize);
    register_demo!(registry, demo_i8_shl_round_assign_i8);
    register_demo!(registry, demo_i8_shl_round_assign_i16);
    register_demo!(registry, demo_i8_shl_round_assign_i32);
    register_demo!(registry, demo_i8_shl_round_assign_i64);
    register_demo!(registry, demo_i8_shl_round_assign_isize);
    register_demo!(registry, demo_i16_shl_round_assign_i8);
    register_demo!(registry, demo_i16_shl_round_assign_i16);
    register_demo!(registry, demo_i16_shl_round_assign_i32);
    register_demo!(registry, demo_i16_shl_round_assign_i64);
    register_demo!(registry, demo_i16_shl_round_assign_isize);
    register_demo!(registry, demo_i32_shl_round_assign_i8);
    register_demo!(registry, demo_i32_shl_round_assign_i16);
    register_demo!(registry, demo_i32_shl_round_assign_i32);
    register_demo!(registry, demo_i32_shl_round_assign_i64);
    register_demo!(registry, demo_i32_shl_round_assign_isize);
    register_demo!(registry, demo_i64_shl_round_assign_i8);
    register_demo!(registry, demo_i64_shl_round_assign_i16);
    register_demo!(registry, demo_i64_shl_round_assign_i32);
    register_demo!(registry, demo_i64_shl_round_assign_i64);
    register_demo!(registry, demo_i64_shl_round_assign_isize);
    register_demo!(registry, demo_isize_shl_round_assign_i8);
    register_demo!(registry, demo_isize_shl_round_assign_i16);
    register_demo!(registry, demo_isize_shl_round_assign_i32);
    register_demo!(registry, demo_isize_shl_round_assign_i64);
    register_demo!(registry, demo_isize_shl_round_assign_isize);

    register_demo!(registry, demo_u8_shl_round_i8);
    register_demo!(registry, demo_u8_shl_round_i16);
    register_demo!(registry, demo_u8_shl_round_i32);
    register_demo!(registry, demo_u8_shl_round_i64);
    register_demo!(registry, demo_u8_shl_round_isize);
    register_demo!(registry, demo_u16_shl_round_i8);
    register_demo!(registry, demo_u16_shl_round_i16);
    register_demo!(registry, demo_u16_shl_round_i32);
    register_demo!(registry, demo_u16_shl_round_i64);
    register_demo!(registry, demo_u16_shl_round_isize);
    register_demo!(registry, demo_u32_shl_round_i8);
    register_demo!(registry, demo_u32_shl_round_i16);
    register_demo!(registry, demo_u32_shl_round_i32);
    register_demo!(registry, demo_u32_shl_round_i64);
    register_demo!(registry, demo_u32_shl_round_isize);
    register_demo!(registry, demo_u64_shl_round_i8);
    register_demo!(registry, demo_u64_shl_round_i16);
    register_demo!(registry, demo_u64_shl_round_i32);
    register_demo!(registry, demo_u64_shl_round_i64);
    register_demo!(registry, demo_u64_shl_round_isize);
    register_demo!(registry, demo_usize_shl_round_i8);
    register_demo!(registry, demo_usize_shl_round_i16);
    register_demo!(registry, demo_usize_shl_round_i32);
    register_demo!(registry, demo_usize_shl_round_i64);
    register_demo!(registry, demo_usize_shl_round_isize);
    register_demo!(registry, demo_i8_shl_round_i8);
    register_demo!(registry, demo_i8_shl_round_i16);
    register_demo!(registry, demo_i8_shl_round_i32);
    register_demo!(registry, demo_i8_shl_round_i64);
    register_demo!(registry, demo_i8_shl_round_isize);
    register_demo!(registry, demo_i16_shl_round_i8);
    register_demo!(registry, demo_i16_shl_round_i16);
    register_demo!(registry, demo_i16_shl_round_i32);
    register_demo!(registry, demo_i16_shl_round_i64);
    register_demo!(registry, demo_i16_shl_round_isize);
    register_demo!(registry, demo_i32_shl_round_i8);
    register_demo!(registry, demo_i32_shl_round_i16);
    register_demo!(registry, demo_i32_shl_round_i32);
    register_demo!(registry, demo_i32_shl_round_i64);
    register_demo!(registry, demo_i32_shl_round_isize);
    register_demo!(registry, demo_i64_shl_round_i8);
    register_demo!(registry, demo_i64_shl_round_i16);
    register_demo!(registry, demo_i64_shl_round_i32);
    register_demo!(registry, demo_i64_shl_round_i64);
    register_demo!(registry, demo_i64_shl_round_isize);
    register_demo!(registry, demo_isize_shl_round_i8);
    register_demo!(registry, demo_isize_shl_round_i16);
    register_demo!(registry, demo_isize_shl_round_i32);
    register_demo!(registry, demo_isize_shl_round_i64);
    register_demo!(registry, demo_isize_shl_round_isize);

    register_bench!(registry, Large, benchmark_u8_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_u8_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_u8_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_u8_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_u8_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_u16_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_u16_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_u16_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_u16_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_u16_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_u32_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_u32_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_u32_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_u32_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_u32_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_u64_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_u64_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_u64_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_u64_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_u64_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_usize_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_usize_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_usize_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_usize_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_usize_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_i8_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_i8_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_i8_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_i8_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_i8_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_i16_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_i16_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_i16_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_i16_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_i16_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_i32_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_i32_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_i32_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_i32_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_i32_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_i64_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_i64_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_i64_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_i64_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_i64_shl_round_assign_isize);
    register_bench!(registry, Large, benchmark_isize_shl_round_assign_i8);
    register_bench!(registry, Large, benchmark_isize_shl_round_assign_i16);
    register_bench!(registry, Large, benchmark_isize_shl_round_assign_i32);
    register_bench!(registry, Large, benchmark_isize_shl_round_assign_i64);
    register_bench!(registry, Large, benchmark_isize_shl_round_assign_isize);

    register_bench!(registry, Large, benchmark_u8_shl_round_i8);
    register_bench!(registry, Large, benchmark_u8_shl_round_i16);
    register_bench!(registry, Large, benchmark_u8_shl_round_i32);
    register_bench!(registry, Large, benchmark_u8_shl_round_i64);
    register_bench!(registry, Large, benchmark_u8_shl_round_isize);
    register_bench!(registry, Large, benchmark_u16_shl_round_i8);
    register_bench!(registry, Large, benchmark_u16_shl_round_i16);
    register_bench!(registry, Large, benchmark_u16_shl_round_i32);
    register_bench!(registry, Large, benchmark_u16_shl_round_i64);
    register_bench!(registry, Large, benchmark_u16_shl_round_isize);
    register_bench!(registry, Large, benchmark_u32_shl_round_i8);
    register_bench!(registry, Large, benchmark_u32_shl_round_i16);
    register_bench!(registry, Large, benchmark_u32_shl_round_i32);
    register_bench!(registry, Large, benchmark_u32_shl_round_i64);
    register_bench!(registry, Large, benchmark_u32_shl_round_isize);
    register_bench!(registry, Large, benchmark_u64_shl_round_i8);
    register_bench!(registry, Large, benchmark_u64_shl_round_i16);
    register_bench!(registry, Large, benchmark_u64_shl_round_i32);
    register_bench!(registry, Large, benchmark_u64_shl_round_i64);
    register_bench!(registry, Large, benchmark_u64_shl_round_isize);
    register_bench!(registry, Large, benchmark_usize_shl_round_i8);
    register_bench!(registry, Large, benchmark_usize_shl_round_i16);
    register_bench!(registry, Large, benchmark_usize_shl_round_i32);
    register_bench!(registry, Large, benchmark_usize_shl_round_i64);
    register_bench!(registry, Large, benchmark_usize_shl_round_isize);
    register_bench!(registry, Large, benchmark_i8_shl_round_i8);
    register_bench!(registry, Large, benchmark_i8_shl_round_i16);
    register_bench!(registry, Large, benchmark_i8_shl_round_i32);
    register_bench!(registry, Large, benchmark_i8_shl_round_i64);
    register_bench!(registry, Large, benchmark_i8_shl_round_isize);
    register_bench!(registry, Large, benchmark_i16_shl_round_i8);
    register_bench!(registry, Large, benchmark_i16_shl_round_i16);
    register_bench!(registry, Large, benchmark_i16_shl_round_i32);
    register_bench!(registry, Large, benchmark_i16_shl_round_i64);
    register_bench!(registry, Large, benchmark_i16_shl_round_isize);
    register_bench!(registry, Large, benchmark_i32_shl_round_i8);
    register_bench!(registry, Large, benchmark_i32_shl_round_i16);
    register_bench!(registry, Large, benchmark_i32_shl_round_i32);
    register_bench!(registry, Large, benchmark_i32_shl_round_i64);
    register_bench!(registry, Large, benchmark_i32_shl_round_isize);
    register_bench!(registry, Large, benchmark_i64_shl_round_i8);
    register_bench!(registry, Large, benchmark_i64_shl_round_i16);
    register_bench!(registry, Large, benchmark_i64_shl_round_i32);
    register_bench!(registry, Large, benchmark_i64_shl_round_i64);
    register_bench!(registry, Large, benchmark_i64_shl_round_isize);
    register_bench!(registry, Large, benchmark_isize_shl_round_i8);
    register_bench!(registry, Large, benchmark_isize_shl_round_i16);
    register_bench!(registry, Large, benchmark_isize_shl_round_i32);
    register_bench!(registry, Large, benchmark_isize_shl_round_i64);
    register_bench!(registry, Large, benchmark_isize_shl_round_isize);
}

macro_rules! shl_round_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_shl_round_assign:ident,
        $demo_shl_round:ident,
        $benchmark_shl_round_assign:ident,
        $benchmark_shl_round:ident
    ) => {
        fn $demo_shl_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_unsigned_small_signed_and_rounding_mode_var_2::<$t, $u>(gm).take(limit)
            {
                n.shl_round_assign(u, rm);
                println!("x := {}; x.shl_round_assign({}, {}); x = {}", n, u, rm, n);
            }
        }

        fn $demo_shl_round(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_unsigned_small_signed_and_rounding_mode_var_2::<$t, $u>(gm).take(limit)
            {
                println!("{}.shl_round({}, {}) = {}", n, u, rm, n.shl_round(u, rm));
            }
        }

        fn $benchmark_shl_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shl_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_rounding_mode_var_2::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shl_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shl_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shl_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_rounding_mode_var_2::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shl_round(y, rm))))],
            );
        }
    };
}
shl_round_u_i!(
    u8,
    i8,
    demo_u8_shl_round_assign_i8,
    demo_u8_shl_round_i8,
    benchmark_u8_shl_round_assign_i8,
    benchmark_u8_shl_round_i8
);
shl_round_u_i!(
    u8,
    i16,
    demo_u8_shl_round_assign_i16,
    demo_u8_shl_round_i16,
    benchmark_u8_shl_round_assign_i16,
    benchmark_u8_shl_round_i16
);
shl_round_u_i!(
    u8,
    i32,
    demo_u8_shl_round_assign_i32,
    demo_u8_shl_round_i32,
    benchmark_u8_shl_round_assign_i32,
    benchmark_u8_shl_round_i32
);
shl_round_u_i!(
    u8,
    i64,
    demo_u8_shl_round_assign_i64,
    demo_u8_shl_round_i64,
    benchmark_u8_shl_round_assign_i64,
    benchmark_u8_shl_round_i64
);
shl_round_u_i!(
    u8,
    isize,
    demo_u8_shl_round_assign_isize,
    demo_u8_shl_round_isize,
    benchmark_u8_shl_round_assign_isize,
    benchmark_u8_shl_round_isize
);

shl_round_u_i!(
    u16,
    i8,
    demo_u16_shl_round_assign_i8,
    demo_u16_shl_round_i8,
    benchmark_u16_shl_round_assign_i8,
    benchmark_u16_shl_round_i8
);
shl_round_u_i!(
    u16,
    i16,
    demo_u16_shl_round_assign_i16,
    demo_u16_shl_round_i16,
    benchmark_u16_shl_round_assign_i16,
    benchmark_u16_shl_round_i16
);
shl_round_u_i!(
    u16,
    i32,
    demo_u16_shl_round_assign_i32,
    demo_u16_shl_round_i32,
    benchmark_u16_shl_round_assign_i32,
    benchmark_u16_shl_round_i32
);
shl_round_u_i!(
    u16,
    i64,
    demo_u16_shl_round_assign_i64,
    demo_u16_shl_round_i64,
    benchmark_u16_shl_round_assign_i64,
    benchmark_u16_shl_round_i64
);
shl_round_u_i!(
    u16,
    isize,
    demo_u16_shl_round_assign_isize,
    demo_u16_shl_round_isize,
    benchmark_u16_shl_round_assign_isize,
    benchmark_u16_shl_round_isize
);

shl_round_u_i!(
    u32,
    i8,
    demo_u32_shl_round_assign_i8,
    demo_u32_shl_round_i8,
    benchmark_u32_shl_round_assign_i8,
    benchmark_u32_shl_round_i8
);
shl_round_u_i!(
    u32,
    i16,
    demo_u32_shl_round_assign_i16,
    demo_u32_shl_round_i16,
    benchmark_u32_shl_round_assign_i16,
    benchmark_u32_shl_round_i16
);
shl_round_u_i!(
    u32,
    i32,
    demo_u32_shl_round_assign_i32,
    demo_u32_shl_round_i32,
    benchmark_u32_shl_round_assign_i32,
    benchmark_u32_shl_round_i32
);
shl_round_u_i!(
    u32,
    i64,
    demo_u32_shl_round_assign_i64,
    demo_u32_shl_round_i64,
    benchmark_u32_shl_round_assign_i64,
    benchmark_u32_shl_round_i64
);
shl_round_u_i!(
    u32,
    isize,
    demo_u32_shl_round_assign_isize,
    demo_u32_shl_round_isize,
    benchmark_u32_shl_round_assign_isize,
    benchmark_u32_shl_round_isize
);

shl_round_u_i!(
    u64,
    i8,
    demo_u64_shl_round_assign_i8,
    demo_u64_shl_round_i8,
    benchmark_u64_shl_round_assign_i8,
    benchmark_u64_shl_round_i8
);
shl_round_u_i!(
    u64,
    i16,
    demo_u64_shl_round_assign_i16,
    demo_u64_shl_round_i16,
    benchmark_u64_shl_round_assign_i16,
    benchmark_u64_shl_round_i16
);
shl_round_u_i!(
    u64,
    i32,
    demo_u64_shl_round_assign_i32,
    demo_u64_shl_round_i32,
    benchmark_u64_shl_round_assign_i32,
    benchmark_u64_shl_round_i32
);
shl_round_u_i!(
    u64,
    i64,
    demo_u64_shl_round_assign_i64,
    demo_u64_shl_round_i64,
    benchmark_u64_shl_round_assign_i64,
    benchmark_u64_shl_round_i64
);
shl_round_u_i!(
    u64,
    isize,
    demo_u64_shl_round_assign_isize,
    demo_u64_shl_round_isize,
    benchmark_u64_shl_round_assign_isize,
    benchmark_u64_shl_round_isize
);

shl_round_u_i!(
    usize,
    i8,
    demo_usize_shl_round_assign_i8,
    demo_usize_shl_round_i8,
    benchmark_usize_shl_round_assign_i8,
    benchmark_usize_shl_round_i8
);
shl_round_u_i!(
    usize,
    i16,
    demo_usize_shl_round_assign_i16,
    demo_usize_shl_round_i16,
    benchmark_usize_shl_round_assign_i16,
    benchmark_usize_shl_round_i16
);
shl_round_u_i!(
    usize,
    i32,
    demo_usize_shl_round_assign_i32,
    demo_usize_shl_round_i32,
    benchmark_usize_shl_round_assign_i32,
    benchmark_usize_shl_round_i32
);
shl_round_u_i!(
    usize,
    i64,
    demo_usize_shl_round_assign_i64,
    demo_usize_shl_round_i64,
    benchmark_usize_shl_round_assign_i64,
    benchmark_usize_shl_round_i64
);
shl_round_u_i!(
    usize,
    isize,
    demo_usize_shl_round_assign_isize,
    demo_usize_shl_round_isize,
    benchmark_usize_shl_round_assign_isize,
    benchmark_usize_shl_round_isize
);

macro_rules! shl_round_i_i {
    (
        $t:ident,
        $u:ident,
        $demo_shl_round_assign:ident,
        $demo_shl_round:ident,
        $benchmark_shl_round_assign:ident,
        $benchmark_shl_round:ident
    ) => {
        fn $demo_shl_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_signed_small_signed_and_rounding_mode_var_2::<$t, $u>(gm).take(limit)
            {
                n.shl_round_assign(u, rm);
                println!("x := {}; x.shl_round_assign({}, {}); x = {}", n, u, rm, n);
            }
        }

        fn $demo_shl_round(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_signed_small_signed_and_rounding_mode_var_2::<$t, $u>(gm).take(limit)
            {
                println!("({}).shl_round({}, {}) = {}", n, u, rm, n.shl_round(u, rm));
            }
        }

        fn $benchmark_shl_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shl_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_signed_and_rounding_mode_var_2::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shl_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shl_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shl_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_signed_and_rounding_mode_var_2::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shl_round(y, rm))))],
            );
        }
    };
}
shl_round_i_i!(
    i8,
    i8,
    demo_i8_shl_round_assign_i8,
    demo_i8_shl_round_i8,
    benchmark_i8_shl_round_assign_i8,
    benchmark_i8_shl_round_i8
);
shl_round_i_i!(
    i8,
    i16,
    demo_i8_shl_round_assign_i16,
    demo_i8_shl_round_i16,
    benchmark_i8_shl_round_assign_i16,
    benchmark_i8_shl_round_i16
);
shl_round_i_i!(
    i8,
    i32,
    demo_i8_shl_round_assign_i32,
    demo_i8_shl_round_i32,
    benchmark_i8_shl_round_assign_i32,
    benchmark_i8_shl_round_i32
);
shl_round_i_i!(
    i8,
    i64,
    demo_i8_shl_round_assign_i64,
    demo_i8_shl_round_i64,
    benchmark_i8_shl_round_assign_i64,
    benchmark_i8_shl_round_i64
);
shl_round_i_i!(
    i8,
    isize,
    demo_i8_shl_round_assign_isize,
    demo_i8_shl_round_isize,
    benchmark_i8_shl_round_assign_isize,
    benchmark_i8_shl_round_isize
);

shl_round_i_i!(
    i16,
    i8,
    demo_i16_shl_round_assign_i8,
    demo_i16_shl_round_i8,
    benchmark_i16_shl_round_assign_i8,
    benchmark_i16_shl_round_i8
);
shl_round_i_i!(
    i16,
    i16,
    demo_i16_shl_round_assign_i16,
    demo_i16_shl_round_i16,
    benchmark_i16_shl_round_assign_i16,
    benchmark_i16_shl_round_i16
);
shl_round_i_i!(
    i16,
    i32,
    demo_i16_shl_round_assign_i32,
    demo_i16_shl_round_i32,
    benchmark_i16_shl_round_assign_i32,
    benchmark_i16_shl_round_i32
);
shl_round_i_i!(
    i16,
    i64,
    demo_i16_shl_round_assign_i64,
    demo_i16_shl_round_i64,
    benchmark_i16_shl_round_assign_i64,
    benchmark_i16_shl_round_i64
);
shl_round_i_i!(
    i16,
    isize,
    demo_i16_shl_round_assign_isize,
    demo_i16_shl_round_isize,
    benchmark_i16_shl_round_assign_isize,
    benchmark_i16_shl_round_isize
);

shl_round_i_i!(
    i32,
    i8,
    demo_i32_shl_round_assign_i8,
    demo_i32_shl_round_i8,
    benchmark_i32_shl_round_assign_i8,
    benchmark_i32_shl_round_i8
);
shl_round_i_i!(
    i32,
    i16,
    demo_i32_shl_round_assign_i16,
    demo_i32_shl_round_i16,
    benchmark_i32_shl_round_assign_i16,
    benchmark_i32_shl_round_i16
);
shl_round_i_i!(
    i32,
    i32,
    demo_i32_shl_round_assign_i32,
    demo_i32_shl_round_i32,
    benchmark_i32_shl_round_assign_i32,
    benchmark_i32_shl_round_i32
);
shl_round_i_i!(
    i32,
    i64,
    demo_i32_shl_round_assign_i64,
    demo_i32_shl_round_i64,
    benchmark_i32_shl_round_assign_i64,
    benchmark_i32_shl_round_i64
);
shl_round_i_i!(
    i32,
    isize,
    demo_i32_shl_round_assign_isize,
    demo_i32_shl_round_isize,
    benchmark_i32_shl_round_assign_isize,
    benchmark_i32_shl_round_isize
);

shl_round_i_i!(
    i64,
    i8,
    demo_i64_shl_round_assign_i8,
    demo_i64_shl_round_i8,
    benchmark_i64_shl_round_assign_i8,
    benchmark_i64_shl_round_i8
);
shl_round_i_i!(
    i64,
    i16,
    demo_i64_shl_round_assign_i16,
    demo_i64_shl_round_i16,
    benchmark_i64_shl_round_assign_i16,
    benchmark_i64_shl_round_i16
);
shl_round_i_i!(
    i64,
    i32,
    demo_i64_shl_round_assign_i32,
    demo_i64_shl_round_i32,
    benchmark_i64_shl_round_assign_i32,
    benchmark_i64_shl_round_i32
);
shl_round_i_i!(
    i64,
    i64,
    demo_i64_shl_round_assign_i64,
    demo_i64_shl_round_i64,
    benchmark_i64_shl_round_assign_i64,
    benchmark_i64_shl_round_i64
);
shl_round_i_i!(
    i64,
    isize,
    demo_i64_shl_round_assign_isize,
    demo_i64_shl_round_isize,
    benchmark_i64_shl_round_assign_isize,
    benchmark_i64_shl_round_isize
);

shl_round_i_i!(
    isize,
    i8,
    demo_isize_shl_round_assign_i8,
    demo_isize_shl_round_i8,
    benchmark_isize_shl_round_assign_i8,
    benchmark_isize_shl_round_i8
);
shl_round_i_i!(
    isize,
    i16,
    demo_isize_shl_round_assign_i16,
    demo_isize_shl_round_i16,
    benchmark_isize_shl_round_assign_i16,
    benchmark_isize_shl_round_i16
);
shl_round_i_i!(
    isize,
    i32,
    demo_isize_shl_round_assign_i32,
    demo_isize_shl_round_i32,
    benchmark_isize_shl_round_assign_i32,
    benchmark_isize_shl_round_i32
);
shl_round_i_i!(
    isize,
    i64,
    demo_isize_shl_round_assign_i64,
    demo_isize_shl_round_i64,
    benchmark_isize_shl_round_assign_i64,
    benchmark_isize_shl_round_i64
);
shl_round_i_i!(
    isize,
    isize,
    demo_isize_shl_round_assign_isize,
    demo_isize_shl_round_isize,
    benchmark_isize_shl_round_assign_isize,
    benchmark_isize_shl_round_isize
);
