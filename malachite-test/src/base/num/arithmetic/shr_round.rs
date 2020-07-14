use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::conversion::traits::ExactFrom;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    triples_of_signed_small_signed_and_rounding_mode_var_1,
    triples_of_signed_small_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_small_signed_and_rounding_mode_var_1,
    triples_of_unsigned_small_unsigned_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_shr_round_assign_u8);
    register_demo!(registry, demo_u8_shr_round_assign_u16);
    register_demo!(registry, demo_u8_shr_round_assign_u32);
    register_demo!(registry, demo_u8_shr_round_assign_u64);
    register_demo!(registry, demo_u8_shr_round_assign_usize);
    register_demo!(registry, demo_u16_shr_round_assign_u8);
    register_demo!(registry, demo_u16_shr_round_assign_u16);
    register_demo!(registry, demo_u16_shr_round_assign_u32);
    register_demo!(registry, demo_u16_shr_round_assign_u64);
    register_demo!(registry, demo_u16_shr_round_assign_usize);
    register_demo!(registry, demo_u32_shr_round_assign_u8);
    register_demo!(registry, demo_u32_shr_round_assign_u16);
    register_demo!(registry, demo_u32_shr_round_assign_u32);
    register_demo!(registry, demo_u32_shr_round_assign_u64);
    register_demo!(registry, demo_u32_shr_round_assign_usize);
    register_demo!(registry, demo_u64_shr_round_assign_u8);
    register_demo!(registry, demo_u64_shr_round_assign_u16);
    register_demo!(registry, demo_u64_shr_round_assign_u32);
    register_demo!(registry, demo_u64_shr_round_assign_u64);
    register_demo!(registry, demo_u64_shr_round_assign_usize);
    register_demo!(registry, demo_usize_shr_round_assign_u8);
    register_demo!(registry, demo_usize_shr_round_assign_u16);
    register_demo!(registry, demo_usize_shr_round_assign_u32);
    register_demo!(registry, demo_usize_shr_round_assign_u64);
    register_demo!(registry, demo_usize_shr_round_assign_usize);
    register_demo!(registry, demo_u8_shr_round_assign_i8);
    register_demo!(registry, demo_u8_shr_round_assign_i16);
    register_demo!(registry, demo_u8_shr_round_assign_i32);
    register_demo!(registry, demo_u8_shr_round_assign_i64);
    register_demo!(registry, demo_u8_shr_round_assign_isize);
    register_demo!(registry, demo_u16_shr_round_assign_i8);
    register_demo!(registry, demo_u16_shr_round_assign_i16);
    register_demo!(registry, demo_u16_shr_round_assign_i32);
    register_demo!(registry, demo_u16_shr_round_assign_i64);
    register_demo!(registry, demo_u16_shr_round_assign_isize);
    register_demo!(registry, demo_u32_shr_round_assign_i8);
    register_demo!(registry, demo_u32_shr_round_assign_i16);
    register_demo!(registry, demo_u32_shr_round_assign_i32);
    register_demo!(registry, demo_u32_shr_round_assign_i64);
    register_demo!(registry, demo_u32_shr_round_assign_isize);
    register_demo!(registry, demo_u64_shr_round_assign_i8);
    register_demo!(registry, demo_u64_shr_round_assign_i16);
    register_demo!(registry, demo_u64_shr_round_assign_i32);
    register_demo!(registry, demo_u64_shr_round_assign_i64);
    register_demo!(registry, demo_u64_shr_round_assign_isize);
    register_demo!(registry, demo_usize_shr_round_assign_i8);
    register_demo!(registry, demo_usize_shr_round_assign_i16);
    register_demo!(registry, demo_usize_shr_round_assign_i32);
    register_demo!(registry, demo_usize_shr_round_assign_i64);
    register_demo!(registry, demo_usize_shr_round_assign_isize);
    register_demo!(registry, demo_i8_shr_round_assign_u8);
    register_demo!(registry, demo_i8_shr_round_assign_u16);
    register_demo!(registry, demo_i8_shr_round_assign_u32);
    register_demo!(registry, demo_i8_shr_round_assign_u64);
    register_demo!(registry, demo_i8_shr_round_assign_usize);
    register_demo!(registry, demo_i16_shr_round_assign_u8);
    register_demo!(registry, demo_i16_shr_round_assign_u16);
    register_demo!(registry, demo_i16_shr_round_assign_u32);
    register_demo!(registry, demo_i16_shr_round_assign_u64);
    register_demo!(registry, demo_i16_shr_round_assign_usize);
    register_demo!(registry, demo_i32_shr_round_assign_u8);
    register_demo!(registry, demo_i32_shr_round_assign_u16);
    register_demo!(registry, demo_i32_shr_round_assign_u32);
    register_demo!(registry, demo_i32_shr_round_assign_u64);
    register_demo!(registry, demo_i32_shr_round_assign_usize);
    register_demo!(registry, demo_i64_shr_round_assign_u8);
    register_demo!(registry, demo_i64_shr_round_assign_u16);
    register_demo!(registry, demo_i64_shr_round_assign_u32);
    register_demo!(registry, demo_i64_shr_round_assign_u64);
    register_demo!(registry, demo_i64_shr_round_assign_usize);
    register_demo!(registry, demo_isize_shr_round_assign_u8);
    register_demo!(registry, demo_isize_shr_round_assign_u16);
    register_demo!(registry, demo_isize_shr_round_assign_u32);
    register_demo!(registry, demo_isize_shr_round_assign_u64);
    register_demo!(registry, demo_isize_shr_round_assign_usize);
    register_demo!(registry, demo_i8_shr_round_assign_i8);
    register_demo!(registry, demo_i8_shr_round_assign_i16);
    register_demo!(registry, demo_i8_shr_round_assign_i32);
    register_demo!(registry, demo_i8_shr_round_assign_i64);
    register_demo!(registry, demo_i8_shr_round_assign_isize);
    register_demo!(registry, demo_i16_shr_round_assign_i8);
    register_demo!(registry, demo_i16_shr_round_assign_i16);
    register_demo!(registry, demo_i16_shr_round_assign_i32);
    register_demo!(registry, demo_i16_shr_round_assign_i64);
    register_demo!(registry, demo_i16_shr_round_assign_isize);
    register_demo!(registry, demo_i32_shr_round_assign_i8);
    register_demo!(registry, demo_i32_shr_round_assign_i16);
    register_demo!(registry, demo_i32_shr_round_assign_i32);
    register_demo!(registry, demo_i32_shr_round_assign_i64);
    register_demo!(registry, demo_i32_shr_round_assign_isize);
    register_demo!(registry, demo_i64_shr_round_assign_i8);
    register_demo!(registry, demo_i64_shr_round_assign_i16);
    register_demo!(registry, demo_i64_shr_round_assign_i32);
    register_demo!(registry, demo_i64_shr_round_assign_i64);
    register_demo!(registry, demo_i64_shr_round_assign_isize);
    register_demo!(registry, demo_isize_shr_round_assign_i8);
    register_demo!(registry, demo_isize_shr_round_assign_i16);
    register_demo!(registry, demo_isize_shr_round_assign_i32);
    register_demo!(registry, demo_isize_shr_round_assign_i64);
    register_demo!(registry, demo_isize_shr_round_assign_isize);

    register_demo!(registry, demo_u8_shr_round_u8);
    register_demo!(registry, demo_u8_shr_round_u16);
    register_demo!(registry, demo_u8_shr_round_u32);
    register_demo!(registry, demo_u8_shr_round_u64);
    register_demo!(registry, demo_u8_shr_round_usize);
    register_demo!(registry, demo_u16_shr_round_u8);
    register_demo!(registry, demo_u16_shr_round_u16);
    register_demo!(registry, demo_u16_shr_round_u32);
    register_demo!(registry, demo_u16_shr_round_u64);
    register_demo!(registry, demo_u16_shr_round_usize);
    register_demo!(registry, demo_u32_shr_round_u8);
    register_demo!(registry, demo_u32_shr_round_u16);
    register_demo!(registry, demo_u32_shr_round_u32);
    register_demo!(registry, demo_u32_shr_round_u64);
    register_demo!(registry, demo_u32_shr_round_usize);
    register_demo!(registry, demo_u64_shr_round_u8);
    register_demo!(registry, demo_u64_shr_round_u16);
    register_demo!(registry, demo_u64_shr_round_u32);
    register_demo!(registry, demo_u64_shr_round_u64);
    register_demo!(registry, demo_u64_shr_round_usize);
    register_demo!(registry, demo_usize_shr_round_u8);
    register_demo!(registry, demo_usize_shr_round_u16);
    register_demo!(registry, demo_usize_shr_round_u32);
    register_demo!(registry, demo_usize_shr_round_u64);
    register_demo!(registry, demo_usize_shr_round_usize);
    register_demo!(registry, demo_u8_shr_round_i8);
    register_demo!(registry, demo_u8_shr_round_i16);
    register_demo!(registry, demo_u8_shr_round_i32);
    register_demo!(registry, demo_u8_shr_round_i64);
    register_demo!(registry, demo_u8_shr_round_isize);
    register_demo!(registry, demo_u16_shr_round_i8);
    register_demo!(registry, demo_u16_shr_round_i16);
    register_demo!(registry, demo_u16_shr_round_i32);
    register_demo!(registry, demo_u16_shr_round_i64);
    register_demo!(registry, demo_u16_shr_round_isize);
    register_demo!(registry, demo_u32_shr_round_i8);
    register_demo!(registry, demo_u32_shr_round_i16);
    register_demo!(registry, demo_u32_shr_round_i32);
    register_demo!(registry, demo_u32_shr_round_i64);
    register_demo!(registry, demo_u32_shr_round_isize);
    register_demo!(registry, demo_u64_shr_round_i8);
    register_demo!(registry, demo_u64_shr_round_i16);
    register_demo!(registry, demo_u64_shr_round_i32);
    register_demo!(registry, demo_u64_shr_round_i64);
    register_demo!(registry, demo_u64_shr_round_isize);
    register_demo!(registry, demo_usize_shr_round_i8);
    register_demo!(registry, demo_usize_shr_round_i16);
    register_demo!(registry, demo_usize_shr_round_i32);
    register_demo!(registry, demo_usize_shr_round_i64);
    register_demo!(registry, demo_usize_shr_round_isize);
    register_demo!(registry, demo_i8_shr_round_u8);
    register_demo!(registry, demo_i8_shr_round_u16);
    register_demo!(registry, demo_i8_shr_round_u32);
    register_demo!(registry, demo_i8_shr_round_u64);
    register_demo!(registry, demo_i8_shr_round_usize);
    register_demo!(registry, demo_i16_shr_round_u8);
    register_demo!(registry, demo_i16_shr_round_u16);
    register_demo!(registry, demo_i16_shr_round_u32);
    register_demo!(registry, demo_i16_shr_round_u64);
    register_demo!(registry, demo_i16_shr_round_usize);
    register_demo!(registry, demo_i32_shr_round_u8);
    register_demo!(registry, demo_i32_shr_round_u16);
    register_demo!(registry, demo_i32_shr_round_u32);
    register_demo!(registry, demo_i32_shr_round_u64);
    register_demo!(registry, demo_i32_shr_round_usize);
    register_demo!(registry, demo_i64_shr_round_u8);
    register_demo!(registry, demo_i64_shr_round_u16);
    register_demo!(registry, demo_i64_shr_round_u32);
    register_demo!(registry, demo_i64_shr_round_u64);
    register_demo!(registry, demo_i64_shr_round_usize);
    register_demo!(registry, demo_isize_shr_round_u8);
    register_demo!(registry, demo_isize_shr_round_u16);
    register_demo!(registry, demo_isize_shr_round_u32);
    register_demo!(registry, demo_isize_shr_round_u64);
    register_demo!(registry, demo_isize_shr_round_usize);
    register_demo!(registry, demo_i8_shr_round_i8);
    register_demo!(registry, demo_i8_shr_round_i16);
    register_demo!(registry, demo_i8_shr_round_i32);
    register_demo!(registry, demo_i8_shr_round_i64);
    register_demo!(registry, demo_i8_shr_round_isize);
    register_demo!(registry, demo_i16_shr_round_i8);
    register_demo!(registry, demo_i16_shr_round_i16);
    register_demo!(registry, demo_i16_shr_round_i32);
    register_demo!(registry, demo_i16_shr_round_i64);
    register_demo!(registry, demo_i16_shr_round_isize);
    register_demo!(registry, demo_i32_shr_round_i8);
    register_demo!(registry, demo_i32_shr_round_i16);
    register_demo!(registry, demo_i32_shr_round_i32);
    register_demo!(registry, demo_i32_shr_round_i64);
    register_demo!(registry, demo_i32_shr_round_isize);
    register_demo!(registry, demo_i64_shr_round_i8);
    register_demo!(registry, demo_i64_shr_round_i16);
    register_demo!(registry, demo_i64_shr_round_i32);
    register_demo!(registry, demo_i64_shr_round_i64);
    register_demo!(registry, demo_i64_shr_round_isize);
    register_demo!(registry, demo_isize_shr_round_i8);
    register_demo!(registry, demo_isize_shr_round_i16);
    register_demo!(registry, demo_isize_shr_round_i32);
    register_demo!(registry, demo_isize_shr_round_i64);
    register_demo!(registry, demo_isize_shr_round_isize);

    register_bench!(registry, Large, benchmark_u8_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_u8_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_u16_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_u32_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_u64_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_usize_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_u8);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_u16);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_u32);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_u64);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_usize);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_i8_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_i16_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_i32_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_i64_shr_round_assign_isize);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_i8);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_i16);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_i32);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_i64);
    register_bench!(registry, Large, benchmark_isize_shr_round_assign_isize);

    register_bench!(registry, Large, benchmark_u8_shr_round_u8);
    register_bench!(registry, Large, benchmark_u8_shr_round_u16);
    register_bench!(registry, Large, benchmark_u8_shr_round_u32);
    register_bench!(registry, Large, benchmark_u8_shr_round_u64);
    register_bench!(registry, Large, benchmark_u8_shr_round_usize);
    register_bench!(registry, Large, benchmark_u16_shr_round_u8);
    register_bench!(registry, Large, benchmark_u16_shr_round_u16);
    register_bench!(registry, Large, benchmark_u16_shr_round_u32);
    register_bench!(registry, Large, benchmark_u16_shr_round_u64);
    register_bench!(registry, Large, benchmark_u16_shr_round_usize);
    register_bench!(registry, Large, benchmark_u32_shr_round_u8);
    register_bench!(registry, Large, benchmark_u32_shr_round_u16);
    register_bench!(registry, Large, benchmark_u32_shr_round_u32);
    register_bench!(registry, Large, benchmark_u32_shr_round_u64);
    register_bench!(registry, Large, benchmark_u32_shr_round_usize);
    register_bench!(registry, Large, benchmark_u64_shr_round_u8);
    register_bench!(registry, Large, benchmark_u64_shr_round_u16);
    register_bench!(registry, Large, benchmark_u64_shr_round_u32);
    register_bench!(registry, Large, benchmark_u64_shr_round_u64);
    register_bench!(registry, Large, benchmark_u64_shr_round_usize);
    register_bench!(registry, Large, benchmark_usize_shr_round_u8);
    register_bench!(registry, Large, benchmark_usize_shr_round_u16);
    register_bench!(registry, Large, benchmark_usize_shr_round_u32);
    register_bench!(registry, Large, benchmark_usize_shr_round_u64);
    register_bench!(registry, Large, benchmark_usize_shr_round_usize);
    register_bench!(registry, Large, benchmark_u8_shr_round_i8);
    register_bench!(registry, Large, benchmark_u8_shr_round_i16);
    register_bench!(registry, Large, benchmark_u8_shr_round_i32);
    register_bench!(registry, Large, benchmark_u8_shr_round_i64);
    register_bench!(registry, Large, benchmark_u8_shr_round_isize);
    register_bench!(registry, Large, benchmark_u16_shr_round_i8);
    register_bench!(registry, Large, benchmark_u16_shr_round_i16);
    register_bench!(registry, Large, benchmark_u16_shr_round_i32);
    register_bench!(registry, Large, benchmark_u16_shr_round_i64);
    register_bench!(registry, Large, benchmark_u16_shr_round_isize);
    register_bench!(registry, Large, benchmark_u32_shr_round_i8);
    register_bench!(registry, Large, benchmark_u32_shr_round_i16);
    register_bench!(registry, Large, benchmark_u32_shr_round_i32);
    register_bench!(registry, Large, benchmark_u32_shr_round_i64);
    register_bench!(registry, Large, benchmark_u32_shr_round_isize);
    register_bench!(registry, Large, benchmark_u64_shr_round_i8);
    register_bench!(registry, Large, benchmark_u64_shr_round_i16);
    register_bench!(registry, Large, benchmark_u64_shr_round_i32);
    register_bench!(registry, Large, benchmark_u64_shr_round_i64);
    register_bench!(registry, Large, benchmark_u64_shr_round_isize);
    register_bench!(registry, Large, benchmark_usize_shr_round_i8);
    register_bench!(registry, Large, benchmark_usize_shr_round_i16);
    register_bench!(registry, Large, benchmark_usize_shr_round_i32);
    register_bench!(registry, Large, benchmark_usize_shr_round_i64);
    register_bench!(registry, Large, benchmark_usize_shr_round_isize);
    register_bench!(registry, Large, benchmark_i8_shr_round_u8);
    register_bench!(registry, Large, benchmark_i8_shr_round_u16);
    register_bench!(registry, Large, benchmark_i8_shr_round_u32);
    register_bench!(registry, Large, benchmark_i8_shr_round_u64);
    register_bench!(registry, Large, benchmark_i8_shr_round_usize);
    register_bench!(registry, Large, benchmark_i16_shr_round_u8);
    register_bench!(registry, Large, benchmark_i16_shr_round_u16);
    register_bench!(registry, Large, benchmark_i16_shr_round_u32);
    register_bench!(registry, Large, benchmark_i16_shr_round_u64);
    register_bench!(registry, Large, benchmark_i16_shr_round_usize);
    register_bench!(registry, Large, benchmark_i32_shr_round_u8);
    register_bench!(registry, Large, benchmark_i32_shr_round_u16);
    register_bench!(registry, Large, benchmark_i32_shr_round_u32);
    register_bench!(registry, Large, benchmark_i32_shr_round_u64);
    register_bench!(registry, Large, benchmark_i32_shr_round_usize);
    register_bench!(registry, Large, benchmark_i64_shr_round_u8);
    register_bench!(registry, Large, benchmark_i64_shr_round_u16);
    register_bench!(registry, Large, benchmark_i64_shr_round_u32);
    register_bench!(registry, Large, benchmark_i64_shr_round_u64);
    register_bench!(registry, Large, benchmark_i64_shr_round_usize);
    register_bench!(registry, Large, benchmark_isize_shr_round_u8);
    register_bench!(registry, Large, benchmark_isize_shr_round_u16);
    register_bench!(registry, Large, benchmark_isize_shr_round_u32);
    register_bench!(registry, Large, benchmark_isize_shr_round_u64);
    register_bench!(registry, Large, benchmark_isize_shr_round_usize);
    register_bench!(registry, Large, benchmark_i8_shr_round_i8);
    register_bench!(registry, Large, benchmark_i8_shr_round_i16);
    register_bench!(registry, Large, benchmark_i8_shr_round_i32);
    register_bench!(registry, Large, benchmark_i8_shr_round_i64);
    register_bench!(registry, Large, benchmark_i8_shr_round_isize);
    register_bench!(registry, Large, benchmark_i16_shr_round_i8);
    register_bench!(registry, Large, benchmark_i16_shr_round_i16);
    register_bench!(registry, Large, benchmark_i16_shr_round_i32);
    register_bench!(registry, Large, benchmark_i16_shr_round_i64);
    register_bench!(registry, Large, benchmark_i16_shr_round_isize);
    register_bench!(registry, Large, benchmark_i32_shr_round_i8);
    register_bench!(registry, Large, benchmark_i32_shr_round_i16);
    register_bench!(registry, Large, benchmark_i32_shr_round_i32);
    register_bench!(registry, Large, benchmark_i32_shr_round_i64);
    register_bench!(registry, Large, benchmark_i32_shr_round_isize);
    register_bench!(registry, Large, benchmark_i64_shr_round_i8);
    register_bench!(registry, Large, benchmark_i64_shr_round_i16);
    register_bench!(registry, Large, benchmark_i64_shr_round_i32);
    register_bench!(registry, Large, benchmark_i64_shr_round_i64);
    register_bench!(registry, Large, benchmark_i64_shr_round_isize);
    register_bench!(registry, Large, benchmark_isize_shr_round_i8);
    register_bench!(registry, Large, benchmark_isize_shr_round_i16);
    register_bench!(registry, Large, benchmark_isize_shr_round_i32);
    register_bench!(registry, Large, benchmark_isize_shr_round_i64);
    register_bench!(registry, Large, benchmark_isize_shr_round_isize);
}

macro_rules! shr_round_u_u {
    (
        $t:ident,
        $u:ident,
        $demo_shr_round_assign:ident,
        $demo_shr_round:ident,
        $benchmark_shr_round_assign:ident,
        $benchmark_shr_round:ident
    ) => {
        fn $demo_shr_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.shr_round_assign(u, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    old_n, u, rm, n
                );
            }
        }

        fn $demo_shr_round(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                println!("{}.shr_round({}, {}) = {}", n, u, rm, n.shr_round(u, rm));
            }
        }

        fn $benchmark_shr_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))))],
            );
        }
    };
}
shr_round_u_u!(
    u8,
    u8,
    demo_u8_shr_round_assign_u8,
    demo_u8_shr_round_u8,
    benchmark_u8_shr_round_assign_u8,
    benchmark_u8_shr_round_u8
);
shr_round_u_u!(
    u8,
    u16,
    demo_u8_shr_round_assign_u16,
    demo_u8_shr_round_u16,
    benchmark_u8_shr_round_assign_u16,
    benchmark_u8_shr_round_u16
);
shr_round_u_u!(
    u8,
    u32,
    demo_u8_shr_round_assign_u32,
    demo_u8_shr_round_u32,
    benchmark_u8_shr_round_assign_u32,
    benchmark_u8_shr_round_u32
);
shr_round_u_u!(
    u8,
    u64,
    demo_u8_shr_round_assign_u64,
    demo_u8_shr_round_u64,
    benchmark_u8_shr_round_assign_u64,
    benchmark_u8_shr_round_u64
);
shr_round_u_u!(
    u8,
    usize,
    demo_u8_shr_round_assign_usize,
    demo_u8_shr_round_usize,
    benchmark_u8_shr_round_assign_usize,
    benchmark_u8_shr_round_usize
);

shr_round_u_u!(
    u16,
    u8,
    demo_u16_shr_round_assign_u8,
    demo_u16_shr_round_u8,
    benchmark_u16_shr_round_assign_u8,
    benchmark_u16_shr_round_u8
);
shr_round_u_u!(
    u16,
    u16,
    demo_u16_shr_round_assign_u16,
    demo_u16_shr_round_u16,
    benchmark_u16_shr_round_assign_u16,
    benchmark_u16_shr_round_u16
);
shr_round_u_u!(
    u16,
    u32,
    demo_u16_shr_round_assign_u32,
    demo_u16_shr_round_u32,
    benchmark_u16_shr_round_assign_u32,
    benchmark_u16_shr_round_u32
);
shr_round_u_u!(
    u16,
    u64,
    demo_u16_shr_round_assign_u64,
    demo_u16_shr_round_u64,
    benchmark_u16_shr_round_assign_u64,
    benchmark_u16_shr_round_u64
);
shr_round_u_u!(
    u16,
    usize,
    demo_u16_shr_round_assign_usize,
    demo_u16_shr_round_usize,
    benchmark_u16_shr_round_assign_usize,
    benchmark_u16_shr_round_usize
);

shr_round_u_u!(
    u32,
    u8,
    demo_u32_shr_round_assign_u8,
    demo_u32_shr_round_u8,
    benchmark_u32_shr_round_assign_u8,
    benchmark_u32_shr_round_u8
);
shr_round_u_u!(
    u32,
    u16,
    demo_u32_shr_round_assign_u16,
    demo_u32_shr_round_u16,
    benchmark_u32_shr_round_assign_u16,
    benchmark_u32_shr_round_u16
);
shr_round_u_u!(
    u32,
    u32,
    demo_u32_shr_round_assign_u32,
    demo_u32_shr_round_u32,
    benchmark_u32_shr_round_assign_u32,
    benchmark_u32_shr_round_u32
);
shr_round_u_u!(
    u32,
    u64,
    demo_u32_shr_round_assign_u64,
    demo_u32_shr_round_u64,
    benchmark_u32_shr_round_assign_u64,
    benchmark_u32_shr_round_u64
);
shr_round_u_u!(
    u32,
    usize,
    demo_u32_shr_round_assign_usize,
    demo_u32_shr_round_usize,
    benchmark_u32_shr_round_assign_usize,
    benchmark_u32_shr_round_usize
);

shr_round_u_u!(
    u64,
    u8,
    demo_u64_shr_round_assign_u8,
    demo_u64_shr_round_u8,
    benchmark_u64_shr_round_assign_u8,
    benchmark_u64_shr_round_u8
);
shr_round_u_u!(
    u64,
    u16,
    demo_u64_shr_round_assign_u16,
    demo_u64_shr_round_u16,
    benchmark_u64_shr_round_assign_u16,
    benchmark_u64_shr_round_u16
);
shr_round_u_u!(
    u64,
    u32,
    demo_u64_shr_round_assign_u32,
    demo_u64_shr_round_u32,
    benchmark_u64_shr_round_assign_u32,
    benchmark_u64_shr_round_u32
);
shr_round_u_u!(
    u64,
    u64,
    demo_u64_shr_round_assign_u64,
    demo_u64_shr_round_u64,
    benchmark_u64_shr_round_assign_u64,
    benchmark_u64_shr_round_u64
);
shr_round_u_u!(
    u64,
    usize,
    demo_u64_shr_round_assign_usize,
    demo_u64_shr_round_usize,
    benchmark_u64_shr_round_assign_usize,
    benchmark_u64_shr_round_usize
);

shr_round_u_u!(
    usize,
    u8,
    demo_usize_shr_round_assign_u8,
    demo_usize_shr_round_u8,
    benchmark_usize_shr_round_assign_u8,
    benchmark_usize_shr_round_u8
);
shr_round_u_u!(
    usize,
    u16,
    demo_usize_shr_round_assign_u16,
    demo_usize_shr_round_u16,
    benchmark_usize_shr_round_assign_u16,
    benchmark_usize_shr_round_u16
);
shr_round_u_u!(
    usize,
    u32,
    demo_usize_shr_round_assign_u32,
    demo_usize_shr_round_u32,
    benchmark_usize_shr_round_assign_u32,
    benchmark_usize_shr_round_u32
);
shr_round_u_u!(
    usize,
    u64,
    demo_usize_shr_round_assign_u64,
    demo_usize_shr_round_u64,
    benchmark_usize_shr_round_assign_u64,
    benchmark_usize_shr_round_u64
);
shr_round_u_u!(
    usize,
    usize,
    demo_usize_shr_round_assign_usize,
    demo_usize_shr_round_usize,
    benchmark_usize_shr_round_assign_usize,
    benchmark_usize_shr_round_usize
);

macro_rules! shr_round_u_i {
    (
        $t:ident,
        $u:ident,
        $demo_shr_round_assign:ident,
        $demo_shr_round:ident,
        $benchmark_shr_round_assign:ident,
        $benchmark_shr_round:ident
    ) => {
        fn $demo_shr_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.shr_round_assign(i, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    old_n, i, rm, n
                );
            }
        }

        fn $demo_shr_round(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                println!("{}.shr_round({}, {}) = {}", n, i, rm, n.shr_round(i, rm));
            }
        }

        fn $benchmark_shr_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_unsigned_small_signed_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))))],
            );
        }
    };
}
shr_round_u_i!(
    u8,
    i8,
    demo_u8_shr_round_assign_i8,
    demo_u8_shr_round_i8,
    benchmark_u8_shr_round_assign_i8,
    benchmark_u8_shr_round_i8
);
shr_round_u_i!(
    u8,
    i16,
    demo_u8_shr_round_assign_i16,
    demo_u8_shr_round_i16,
    benchmark_u8_shr_round_assign_i16,
    benchmark_u8_shr_round_i16
);
shr_round_u_i!(
    u8,
    i32,
    demo_u8_shr_round_assign_i32,
    demo_u8_shr_round_i32,
    benchmark_u8_shr_round_assign_i32,
    benchmark_u8_shr_round_i32
);
shr_round_u_i!(
    u8,
    i64,
    demo_u8_shr_round_assign_i64,
    demo_u8_shr_round_i64,
    benchmark_u8_shr_round_assign_i64,
    benchmark_u8_shr_round_i64
);
shr_round_u_i!(
    u8,
    isize,
    demo_u8_shr_round_assign_isize,
    demo_u8_shr_round_isize,
    benchmark_u8_shr_round_assign_isize,
    benchmark_u8_shr_round_isize
);

shr_round_u_i!(
    u16,
    i8,
    demo_u16_shr_round_assign_i8,
    demo_u16_shr_round_i8,
    benchmark_u16_shr_round_assign_i8,
    benchmark_u16_shr_round_i8
);
shr_round_u_i!(
    u16,
    i16,
    demo_u16_shr_round_assign_i16,
    demo_u16_shr_round_i16,
    benchmark_u16_shr_round_assign_i16,
    benchmark_u16_shr_round_i16
);
shr_round_u_i!(
    u16,
    i32,
    demo_u16_shr_round_assign_i32,
    demo_u16_shr_round_i32,
    benchmark_u16_shr_round_assign_i32,
    benchmark_u16_shr_round_i32
);
shr_round_u_i!(
    u16,
    i64,
    demo_u16_shr_round_assign_i64,
    demo_u16_shr_round_i64,
    benchmark_u16_shr_round_assign_i64,
    benchmark_u16_shr_round_i64
);
shr_round_u_i!(
    u16,
    isize,
    demo_u16_shr_round_assign_isize,
    demo_u16_shr_round_isize,
    benchmark_u16_shr_round_assign_isize,
    benchmark_u16_shr_round_isize
);

shr_round_u_i!(
    u32,
    i8,
    demo_u32_shr_round_assign_i8,
    demo_u32_shr_round_i8,
    benchmark_u32_shr_round_assign_i8,
    benchmark_u32_shr_round_i8
);
shr_round_u_i!(
    u32,
    i16,
    demo_u32_shr_round_assign_i16,
    demo_u32_shr_round_i16,
    benchmark_u32_shr_round_assign_i16,
    benchmark_u32_shr_round_i16
);
shr_round_u_i!(
    u32,
    i32,
    demo_u32_shr_round_assign_i32,
    demo_u32_shr_round_i32,
    benchmark_u32_shr_round_assign_i32,
    benchmark_u32_shr_round_i32
);
shr_round_u_i!(
    u32,
    i64,
    demo_u32_shr_round_assign_i64,
    demo_u32_shr_round_i64,
    benchmark_u32_shr_round_assign_i64,
    benchmark_u32_shr_round_i64
);
shr_round_u_i!(
    u32,
    isize,
    demo_u32_shr_round_assign_isize,
    demo_u32_shr_round_isize,
    benchmark_u32_shr_round_assign_isize,
    benchmark_u32_shr_round_isize
);

shr_round_u_i!(
    u64,
    i8,
    demo_u64_shr_round_assign_i8,
    demo_u64_shr_round_i8,
    benchmark_u64_shr_round_assign_i8,
    benchmark_u64_shr_round_i8
);
shr_round_u_i!(
    u64,
    i16,
    demo_u64_shr_round_assign_i16,
    demo_u64_shr_round_i16,
    benchmark_u64_shr_round_assign_i16,
    benchmark_u64_shr_round_i16
);
shr_round_u_i!(
    u64,
    i32,
    demo_u64_shr_round_assign_i32,
    demo_u64_shr_round_i32,
    benchmark_u64_shr_round_assign_i32,
    benchmark_u64_shr_round_i32
);
shr_round_u_i!(
    u64,
    i64,
    demo_u64_shr_round_assign_i64,
    demo_u64_shr_round_i64,
    benchmark_u64_shr_round_assign_i64,
    benchmark_u64_shr_round_i64
);
shr_round_u_i!(
    u64,
    isize,
    demo_u64_shr_round_assign_isize,
    demo_u64_shr_round_isize,
    benchmark_u64_shr_round_assign_isize,
    benchmark_u64_shr_round_isize
);

shr_round_u_i!(
    usize,
    i8,
    demo_usize_shr_round_assign_i8,
    demo_usize_shr_round_i8,
    benchmark_usize_shr_round_assign_i8,
    benchmark_usize_shr_round_i8
);
shr_round_u_i!(
    usize,
    i16,
    demo_usize_shr_round_assign_i16,
    demo_usize_shr_round_i16,
    benchmark_usize_shr_round_assign_i16,
    benchmark_usize_shr_round_i16
);
shr_round_u_i!(
    usize,
    i32,
    demo_usize_shr_round_assign_i32,
    demo_usize_shr_round_i32,
    benchmark_usize_shr_round_assign_i32,
    benchmark_usize_shr_round_i32
);
shr_round_u_i!(
    usize,
    i64,
    demo_usize_shr_round_assign_i64,
    demo_usize_shr_round_i64,
    benchmark_usize_shr_round_assign_i64,
    benchmark_usize_shr_round_i64
);
shr_round_u_i!(
    usize,
    isize,
    demo_usize_shr_round_assign_isize,
    demo_usize_shr_round_isize,
    benchmark_usize_shr_round_assign_isize,
    benchmark_usize_shr_round_isize
);

macro_rules! shr_round_i_u {
    (
        $t:ident,
        $u:ident,
        $demo_shr_round_assign:ident,
        $demo_shr_round:ident,
        $benchmark_shr_round_assign:ident,
        $benchmark_shr_round:ident
    ) => {
        fn $demo_shr_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, u, rm) in
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.shr_round_assign(u, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    old_n, u, rm, n
                );
            }
        }

        fn $demo_shr_round(gm: GenerationMode, limit: usize) {
            for (n, u, rm) in
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                println!("({}).shr_round({}, {}) = {}", n, u, rm, n.shr_round(u, rm));
            }
        }

        fn $benchmark_shr_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_unsigned_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))))],
            );
        }
    };
}
shr_round_i_u!(
    i8,
    u8,
    demo_i8_shr_round_assign_u8,
    demo_i8_shr_round_u8,
    benchmark_i8_shr_round_assign_u8,
    benchmark_i8_shr_round_u8
);
shr_round_i_u!(
    i8,
    u16,
    demo_i8_shr_round_assign_u16,
    demo_i8_shr_round_u16,
    benchmark_i8_shr_round_assign_u16,
    benchmark_i8_shr_round_u16
);
shr_round_i_u!(
    i8,
    u32,
    demo_i8_shr_round_assign_u32,
    demo_i8_shr_round_u32,
    benchmark_i8_shr_round_assign_u32,
    benchmark_i8_shr_round_u32
);
shr_round_i_u!(
    i8,
    u64,
    demo_i8_shr_round_assign_u64,
    demo_i8_shr_round_u64,
    benchmark_i8_shr_round_assign_u64,
    benchmark_i8_shr_round_u64
);
shr_round_i_u!(
    i8,
    usize,
    demo_i8_shr_round_assign_usize,
    demo_i8_shr_round_usize,
    benchmark_i8_shr_round_assign_usize,
    benchmark_i8_shr_round_usize
);

shr_round_i_u!(
    i16,
    u8,
    demo_i16_shr_round_assign_u8,
    demo_i16_shr_round_u8,
    benchmark_i16_shr_round_assign_u8,
    benchmark_i16_shr_round_u8
);
shr_round_i_u!(
    i16,
    u16,
    demo_i16_shr_round_assign_u16,
    demo_i16_shr_round_u16,
    benchmark_i16_shr_round_assign_u16,
    benchmark_i16_shr_round_u16
);
shr_round_i_u!(
    i16,
    u32,
    demo_i16_shr_round_assign_u32,
    demo_i16_shr_round_u32,
    benchmark_i16_shr_round_assign_u32,
    benchmark_i16_shr_round_u32
);
shr_round_i_u!(
    i16,
    u64,
    demo_i16_shr_round_assign_u64,
    demo_i16_shr_round_u64,
    benchmark_i16_shr_round_assign_u64,
    benchmark_i16_shr_round_u64
);
shr_round_i_u!(
    i16,
    usize,
    demo_i16_shr_round_assign_usize,
    demo_i16_shr_round_usize,
    benchmark_i16_shr_round_assign_usize,
    benchmark_i16_shr_round_usize
);

shr_round_i_u!(
    i32,
    u8,
    demo_i32_shr_round_assign_u8,
    demo_i32_shr_round_u8,
    benchmark_i32_shr_round_assign_u8,
    benchmark_i32_shr_round_u8
);
shr_round_i_u!(
    i32,
    u16,
    demo_i32_shr_round_assign_u16,
    demo_i32_shr_round_u16,
    benchmark_i32_shr_round_assign_u16,
    benchmark_i32_shr_round_u16
);
shr_round_i_u!(
    i32,
    u32,
    demo_i32_shr_round_assign_u32,
    demo_i32_shr_round_u32,
    benchmark_i32_shr_round_assign_u32,
    benchmark_i32_shr_round_u32
);
shr_round_i_u!(
    i32,
    u64,
    demo_i32_shr_round_assign_u64,
    demo_i32_shr_round_u64,
    benchmark_i32_shr_round_assign_u64,
    benchmark_i32_shr_round_u64
);
shr_round_i_u!(
    i32,
    usize,
    demo_i32_shr_round_assign_usize,
    demo_i32_shr_round_usize,
    benchmark_i32_shr_round_assign_usize,
    benchmark_i32_shr_round_usize
);

shr_round_i_u!(
    i64,
    u8,
    demo_i64_shr_round_assign_u8,
    demo_i64_shr_round_u8,
    benchmark_i64_shr_round_assign_u8,
    benchmark_i64_shr_round_u8
);
shr_round_i_u!(
    i64,
    u16,
    demo_i64_shr_round_assign_u16,
    demo_i64_shr_round_u16,
    benchmark_i64_shr_round_assign_u16,
    benchmark_i64_shr_round_u16
);
shr_round_i_u!(
    i64,
    u32,
    demo_i64_shr_round_assign_u32,
    demo_i64_shr_round_u32,
    benchmark_i64_shr_round_assign_u32,
    benchmark_i64_shr_round_u32
);
shr_round_i_u!(
    i64,
    u64,
    demo_i64_shr_round_assign_u64,
    demo_i64_shr_round_u64,
    benchmark_i64_shr_round_assign_u64,
    benchmark_i64_shr_round_u64
);
shr_round_i_u!(
    i64,
    usize,
    demo_i64_shr_round_assign_usize,
    demo_i64_shr_round_usize,
    benchmark_i64_shr_round_assign_usize,
    benchmark_i64_shr_round_usize
);

shr_round_i_u!(
    isize,
    u8,
    demo_isize_shr_round_assign_u8,
    demo_isize_shr_round_u8,
    benchmark_isize_shr_round_assign_u8,
    benchmark_isize_shr_round_u8
);
shr_round_i_u!(
    isize,
    u16,
    demo_isize_shr_round_assign_u16,
    demo_isize_shr_round_u16,
    benchmark_isize_shr_round_assign_u16,
    benchmark_isize_shr_round_u16
);
shr_round_i_u!(
    isize,
    u32,
    demo_isize_shr_round_assign_u32,
    demo_isize_shr_round_u32,
    benchmark_isize_shr_round_assign_u32,
    benchmark_isize_shr_round_u32
);
shr_round_i_u!(
    isize,
    u64,
    demo_isize_shr_round_assign_u64,
    demo_isize_shr_round_u64,
    benchmark_isize_shr_round_assign_u64,
    benchmark_isize_shr_round_u64
);
shr_round_i_u!(
    isize,
    usize,
    demo_isize_shr_round_assign_usize,
    demo_isize_shr_round_usize,
    benchmark_isize_shr_round_assign_usize,
    benchmark_isize_shr_round_usize
);

macro_rules! shr_round_i_i {
    (
        $t:ident,
        $u:ident,
        $demo_shr_round_assign:ident,
        $demo_shr_round:ident,
        $benchmark_shr_round_assign:ident,
        $benchmark_shr_round:ident
    ) => {
        fn $demo_shr_round_assign(gm: GenerationMode, limit: usize) {
            for (mut n, i, rm) in
                triples_of_signed_small_signed_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                let old_n = n;
                n.shr_round_assign(i, rm);
                println!(
                    "x := {}; x.shr_round_assign({}, {}); x = {}",
                    old_n, i, rm, n
                );
            }
        }

        fn $demo_shr_round(gm: GenerationMode, limit: usize) {
            for (n, i, rm) in
                triples_of_signed_small_signed_and_rounding_mode_var_1::<$t, $u>(gm).take(limit)
            {
                println!("({}).shr_round({}, {}) = {}", n, i, rm, n.shr_round(i, rm));
            }
        }

        fn $benchmark_shr_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round_assign({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_signed_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [(
                    "malachite",
                    &mut (|(mut x, y, rm)| x.shr_round_assign(y, rm)),
                )],
            );
        }

        fn $benchmark_shr_round(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}.shr_round({}, RoundingMode)", $t::NAME, $u::NAME),
                BenchmarkType::Single,
                triples_of_signed_small_signed_and_rounding_mode_var_1::<$t, $u>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other, _)| usize::exact_from(other)),
                "other",
                &mut [("malachite", &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm))))],
            );
        }
    };
}
shr_round_i_i!(
    i8,
    i8,
    demo_i8_shr_round_assign_i8,
    demo_i8_shr_round_i8,
    benchmark_i8_shr_round_assign_i8,
    benchmark_i8_shr_round_i8
);
shr_round_i_i!(
    i8,
    i16,
    demo_i8_shr_round_assign_i16,
    demo_i8_shr_round_i16,
    benchmark_i8_shr_round_assign_i16,
    benchmark_i8_shr_round_i16
);
shr_round_i_i!(
    i8,
    i32,
    demo_i8_shr_round_assign_i32,
    demo_i8_shr_round_i32,
    benchmark_i8_shr_round_assign_i32,
    benchmark_i8_shr_round_i32
);
shr_round_i_i!(
    i8,
    i64,
    demo_i8_shr_round_assign_i64,
    demo_i8_shr_round_i64,
    benchmark_i8_shr_round_assign_i64,
    benchmark_i8_shr_round_i64
);
shr_round_i_i!(
    i8,
    isize,
    demo_i8_shr_round_assign_isize,
    demo_i8_shr_round_isize,
    benchmark_i8_shr_round_assign_isize,
    benchmark_i8_shr_round_isize
);

shr_round_i_i!(
    i16,
    i8,
    demo_i16_shr_round_assign_i8,
    demo_i16_shr_round_i8,
    benchmark_i16_shr_round_assign_i8,
    benchmark_i16_shr_round_i8
);
shr_round_i_i!(
    i16,
    i16,
    demo_i16_shr_round_assign_i16,
    demo_i16_shr_round_i16,
    benchmark_i16_shr_round_assign_i16,
    benchmark_i16_shr_round_i16
);
shr_round_i_i!(
    i16,
    i32,
    demo_i16_shr_round_assign_i32,
    demo_i16_shr_round_i32,
    benchmark_i16_shr_round_assign_i32,
    benchmark_i16_shr_round_i32
);
shr_round_i_i!(
    i16,
    i64,
    demo_i16_shr_round_assign_i64,
    demo_i16_shr_round_i64,
    benchmark_i16_shr_round_assign_i64,
    benchmark_i16_shr_round_i64
);
shr_round_i_i!(
    i16,
    isize,
    demo_i16_shr_round_assign_isize,
    demo_i16_shr_round_isize,
    benchmark_i16_shr_round_assign_isize,
    benchmark_i16_shr_round_isize
);

shr_round_i_i!(
    i32,
    i8,
    demo_i32_shr_round_assign_i8,
    demo_i32_shr_round_i8,
    benchmark_i32_shr_round_assign_i8,
    benchmark_i32_shr_round_i8
);
shr_round_i_i!(
    i32,
    i16,
    demo_i32_shr_round_assign_i16,
    demo_i32_shr_round_i16,
    benchmark_i32_shr_round_assign_i16,
    benchmark_i32_shr_round_i16
);
shr_round_i_i!(
    i32,
    i32,
    demo_i32_shr_round_assign_i32,
    demo_i32_shr_round_i32,
    benchmark_i32_shr_round_assign_i32,
    benchmark_i32_shr_round_i32
);
shr_round_i_i!(
    i32,
    i64,
    demo_i32_shr_round_assign_i64,
    demo_i32_shr_round_i64,
    benchmark_i32_shr_round_assign_i64,
    benchmark_i32_shr_round_i64
);
shr_round_i_i!(
    i32,
    isize,
    demo_i32_shr_round_assign_isize,
    demo_i32_shr_round_isize,
    benchmark_i32_shr_round_assign_isize,
    benchmark_i32_shr_round_isize
);

shr_round_i_i!(
    i64,
    i8,
    demo_i64_shr_round_assign_i8,
    demo_i64_shr_round_i8,
    benchmark_i64_shr_round_assign_i8,
    benchmark_i64_shr_round_i8
);
shr_round_i_i!(
    i64,
    i16,
    demo_i64_shr_round_assign_i16,
    demo_i64_shr_round_i16,
    benchmark_i64_shr_round_assign_i16,
    benchmark_i64_shr_round_i16
);
shr_round_i_i!(
    i64,
    i32,
    demo_i64_shr_round_assign_i32,
    demo_i64_shr_round_i32,
    benchmark_i64_shr_round_assign_i32,
    benchmark_i64_shr_round_i32
);
shr_round_i_i!(
    i64,
    i64,
    demo_i64_shr_round_assign_i64,
    demo_i64_shr_round_i64,
    benchmark_i64_shr_round_assign_i64,
    benchmark_i64_shr_round_i64
);
shr_round_i_i!(
    i64,
    isize,
    demo_i64_shr_round_assign_isize,
    demo_i64_shr_round_isize,
    benchmark_i64_shr_round_assign_isize,
    benchmark_i64_shr_round_isize
);

shr_round_i_i!(
    isize,
    i8,
    demo_isize_shr_round_assign_i8,
    demo_isize_shr_round_i8,
    benchmark_isize_shr_round_assign_i8,
    benchmark_isize_shr_round_i8
);
shr_round_i_i!(
    isize,
    i16,
    demo_isize_shr_round_assign_i16,
    demo_isize_shr_round_i16,
    benchmark_isize_shr_round_assign_i16,
    benchmark_isize_shr_round_i16
);
shr_round_i_i!(
    isize,
    i32,
    demo_isize_shr_round_assign_i32,
    demo_isize_shr_round_i32,
    benchmark_isize_shr_round_assign_i32,
    benchmark_isize_shr_round_i32
);
shr_round_i_i!(
    isize,
    i64,
    demo_isize_shr_round_assign_i64,
    demo_isize_shr_round_i64,
    benchmark_isize_shr_round_assign_i64,
    benchmark_isize_shr_round_i64
);
shr_round_i_i!(
    isize,
    isize,
    demo_isize_shr_round_assign_isize,
    demo_isize_shr_round_isize,
    benchmark_isize_shr_round_assign_isize,
    benchmark_isize_shr_round_isize
);
