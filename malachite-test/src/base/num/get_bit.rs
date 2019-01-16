use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use rand::Rand;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_get_bit);
    register_demo!(registry, demo_u16_get_bit);
    register_demo!(registry, demo_u32_get_bit);
    register_demo!(registry, demo_u64_get_bit);
    register_demo!(registry, demo_i8_get_bit);
    register_demo!(registry, demo_i16_get_bit);
    register_demo!(registry, demo_i32_get_bit);
    register_demo!(registry, demo_i64_get_bit);
    register_bench!(registry, None, benchmark_u8_get_bit);
    register_bench!(registry, None, benchmark_u16_get_bit);
    register_bench!(registry, None, benchmark_u32_get_bit);
    register_bench!(registry, None, benchmark_u64_get_bit);
    register_bench!(registry, None, benchmark_i8_get_bit);
    register_bench!(registry, None, benchmark_i16_get_bit);
    register_bench!(registry, None, benchmark_i32_get_bit);
    register_bench!(registry, None, benchmark_i64_get_bit);
}

fn demo_unsigned_get_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_unsigned_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn demo_signed_get_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
{
    for (n, index) in pairs_of_signed_and_small_unsigned::<T, u64>(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_unsigned_get_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.get_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_unsigned_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [("malachite", &mut (|(n, index)| no_out!(n.get_bit(index))))],
    );
}

fn benchmark_signed_get_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
{
    m_run_benchmark(
        &format!("{}.get_bit(u64)", T::NAME),
        BenchmarkType::Single,
        pairs_of_signed_and_small_unsigned::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [("malachite", &mut (|(n, index)| no_out!(n.get_bit(index))))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_get_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_get_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_get_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_get_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_get_bit, benchmark_u8_get_bit);
unsigned!(u16, demo_u16_get_bit, benchmark_u16_get_bit);
unsigned!(u32, demo_u32_get_bit, benchmark_u32_get_bit);
unsigned!(u64, demo_u64_get_bit, benchmark_u64_get_bit);

signed!(i8, demo_i8_get_bit, benchmark_i8_get_bit);
signed!(i16, demo_i16_get_bit, benchmark_i16_get_bit);
signed!(i32, demo_i32_get_bit, benchmark_i32_get_bit);
signed!(i64, demo_i64_get_bit, benchmark_i64_get_bit);
