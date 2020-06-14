use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{triples_of_signeds, triples_of_unsigneds};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_eq_mod);
    register_demo!(registry, demo_u16_eq_mod);
    register_demo!(registry, demo_u32_eq_mod);
    register_demo!(registry, demo_u64_eq_mod);
    register_demo!(registry, demo_usize_eq_mod);
    register_demo!(registry, demo_i8_eq_mod);
    register_demo!(registry, demo_i16_eq_mod);
    register_demo!(registry, demo_i32_eq_mod);
    register_demo!(registry, demo_i64_eq_mod);
    register_demo!(registry, demo_isize_eq_mod);

    register_bench!(registry, None, benchmark_u8_eq_mod);
    register_bench!(registry, None, benchmark_u16_eq_mod);
    register_bench!(registry, None, benchmark_u32_eq_mod);
    register_bench!(registry, None, benchmark_u64_eq_mod);
    register_bench!(registry, None, benchmark_usize_eq_mod);
    register_bench!(registry, None, benchmark_i8_eq_mod);
    register_bench!(registry, None, benchmark_i16_eq_mod);
    register_bench!(registry, None, benchmark_i32_eq_mod);
    register_bench!(registry, None, benchmark_i64_eq_mod);
    register_bench!(registry, None, benchmark_isize_eq_mod);
}

fn demo_unsigned_eq_mod<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for (x, y, z) in triples_of_unsigneds::<T>(gm).take(limit) {
        if x.eq_mod(y, z) {
            println!("{} is equal to {} mod {}", x, y, z);
        } else {
            println!("{} is not equal to {} mod {}", x, y, z);
        }
    }
}

fn demo_signed_eq_mod<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for (x, y, z) in triples_of_signeds::<T>(gm).take(limit) {
        if x.eq_mod(y, z) {
            println!("{} is equal to {} mod {}", x, y, z);
        } else {
            println!("{} is not equal to {} mod {}", x, y, z);
        }
    }
}

fn bucketing_function<T: PrimitiveInteger>(t: &(T, T, T)) -> usize {
    usize::exact_from(max!(
        t.0.significant_bits(),
        t.1.significant_bits(),
        t.2.significant_bits()
    ))
}

fn benchmark_unsigned_eq_mod<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.eq_mod({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(x.eq_mod(y, z))))],
    );
}

fn benchmark_signed_eq_mod<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.eq_mod({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        triples_of_signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [("malachite", &mut (|(x, y, z)| no_out!(x.eq_mod(y, z))))],
    );
}

macro_rules! unsigned {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_eq_mod::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_eq_mod::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    (
        $t:ident,
        $demo_name:ident,
        $bench_name:ident
    ) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_eq_mod::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_eq_mod::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_eq_mod, benchmark_u8_eq_mod);
unsigned!(u16, demo_u16_eq_mod, benchmark_u16_eq_mod);
unsigned!(u32, demo_u32_eq_mod, benchmark_u32_eq_mod);
unsigned!(u64, demo_u64_eq_mod, benchmark_u64_eq_mod);
unsigned!(usize, demo_usize_eq_mod, benchmark_usize_eq_mod);

signed!(i8, demo_i8_eq_mod, benchmark_i8_eq_mod);
signed!(i16, demo_i16_eq_mod, benchmark_i16_eq_mod);
signed!(i32, demo_i32_eq_mod, benchmark_i32_eq_mod);
signed!(i64, demo_i64_eq_mod, benchmark_i64_eq_mod);
signed!(isize, demo_isize_eq_mod, benchmark_isize_eq_mod);
