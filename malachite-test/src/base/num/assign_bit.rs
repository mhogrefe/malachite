use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_signed_unsigned_width_range_and_bool_var_1,
    triples_of_unsigned_unsigned_width_range_and_bool_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_assign_bit);
    register_demo!(registry, demo_u16_assign_bit);
    register_demo!(registry, demo_u32_assign_bit);
    register_demo!(registry, demo_u64_assign_bit);
    register_demo!(registry, demo_i8_assign_bit);
    register_demo!(registry, demo_i16_assign_bit);
    register_demo!(registry, demo_i32_assign_bit);
    register_demo!(registry, demo_i64_assign_bit);
    register_bench!(registry, None, benchmark_u8_assign_bit);
    register_bench!(registry, None, benchmark_u16_assign_bit);
    register_bench!(registry, None, benchmark_u32_assign_bit);
    register_bench!(registry, None, benchmark_u64_assign_bit);
    register_bench!(registry, None, benchmark_i8_assign_bit);
    register_bench!(registry, None, benchmark_i16_assign_bit);
    register_bench!(registry, None, benchmark_i32_assign_bit);
    register_bench!(registry, None, benchmark_i64_assign_bit);
}

fn demo_unsigned_assign_bit<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in
        triples_of_unsigned_unsigned_width_range_and_bool_var_1::<T, u64>(gm).take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn demo_signed_assign_bit<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (mut n, index, bit) in
        triples_of_signed_unsigned_width_range_and_bool_var_1::<T, u64>(gm).take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn benchmark_unsigned_assign_bit<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.assign_bit(u64)", T::NAME),
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_width_range_and_bool_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index, _)| usize::checked_from(index).unwrap()),
        "index",
        &mut [(
            "malachite",
            &mut (|(mut n, index, bit)| n.assign_bit(index, bit)),
        )],
    );
}

fn benchmark_signed_assign_bit<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.assign_bit(u64)", T::NAME),
        BenchmarkType::Single,
        triples_of_signed_unsigned_width_range_and_bool_var_1::<T, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index, _)| usize::checked_from(index).unwrap()),
        "index",
        &mut [(
            "malachite",
            &mut (|(mut n, index, bit)| n.assign_bit(index, bit)),
        )],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_assign_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_assign_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_assign_bit::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_assign_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_assign_bit, benchmark_u8_assign_bit);
unsigned!(u16, demo_u16_assign_bit, benchmark_u16_assign_bit);
unsigned!(u32, demo_u32_assign_bit, benchmark_u32_assign_bit);
unsigned!(u64, demo_u64_assign_bit, benchmark_u64_assign_bit);

signed!(i8, demo_i8_assign_bit, benchmark_i8_assign_bit);
signed!(i16, demo_i16_assign_bit, benchmark_i16_assign_bit);
signed!(i32, demo_i32_assign_bit, benchmark_i32_assign_bit);
signed!(i64, demo_i64_assign_bit, benchmark_i64_assign_bit);
