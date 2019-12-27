use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_signed_and_u64_width_range_var_2, pairs_of_unsigned_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_clear_bit);
    register_demo!(registry, demo_u16_clear_bit);
    register_demo!(registry, demo_u32_clear_bit);
    register_demo!(registry, demo_u64_clear_bit);
    register_demo!(registry, demo_i8_clear_bit);
    register_demo!(registry, demo_i16_clear_bit);
    register_demo!(registry, demo_i32_clear_bit);
    register_demo!(registry, demo_i64_clear_bit);
    register_bench!(registry, None, benchmark_u8_clear_bit);
    register_bench!(registry, None, benchmark_u16_clear_bit);
    register_bench!(registry, None, benchmark_u32_clear_bit);
    register_bench!(registry, None, benchmark_u64_clear_bit);
    register_bench!(registry, None, benchmark_i8_clear_bit);
    register_bench!(registry, None, benchmark_i16_clear_bit);
    register_bench!(registry, None, benchmark_i32_clear_bit);
    register_bench!(registry, None, benchmark_i64_clear_bit);
}

fn demo_unsigned_clear_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_clear_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index) in pairs_of_signed_and_u64_width_range_var_2::<T>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_unsigned_clear_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.clear_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}

fn benchmark_signed_clear_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.clear_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_u64_width_range_var_2::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_clear_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_clear_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_clear_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_clear_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_clear_bit, benchmark_u8_clear_bit);
unsigned!(u16, demo_u16_clear_bit, benchmark_u16_clear_bit);
unsigned!(u32, demo_u32_clear_bit, benchmark_u32_clear_bit);
unsigned!(u64, demo_u64_clear_bit, benchmark_u64_clear_bit);

signed!(i8, demo_i8_clear_bit, benchmark_i8_clear_bit);
signed!(i16, demo_i16_clear_bit, benchmark_i16_clear_bit);
signed!(i32, demo_i32_clear_bit, benchmark_i32_clear_bit);
signed!(i64, demo_i64_clear_bit, benchmark_i64_clear_bit);
