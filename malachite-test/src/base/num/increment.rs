use malachite_base::conversion::WrappingFrom;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use rand::Rand;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{signeds_no_max, unsigneds_no_max};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_increment);
    register_demo!(registry, demo_u16_increment);
    register_demo!(registry, demo_u32_increment);
    register_demo!(registry, demo_u64_increment);
    register_demo!(registry, demo_i8_increment);
    register_demo!(registry, demo_i16_increment);
    register_demo!(registry, demo_i32_increment);
    register_demo!(registry, demo_i64_increment);
    register_bench!(registry, None, benchmark_u8_increment);
    register_bench!(registry, None, benchmark_u16_increment);
    register_bench!(registry, None, benchmark_u32_increment);
    register_bench!(registry, None, benchmark_u64_increment);
    register_bench!(registry, None, benchmark_i8_increment);
    register_bench!(registry, None, benchmark_i16_increment);
    register_bench!(registry, None, benchmark_i32_increment);
    register_bench!(registry, None, benchmark_i64_increment);
}

fn demo_unsigned_increment<T: PrimitiveUnsigned + Rand>(gm: GenerationMode, limit: usize) {
    for mut n in unsigneds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn demo_signed_increment<T: PrimitiveSigned + Rand>(gm: GenerationMode, limit: usize)
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    for mut n in signeds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn benchmark_unsigned_increment<T: PrimitiveUnsigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Single,
        unsigneds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}

fn benchmark_signed_increment<T: PrimitiveSigned + Rand>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Single,
        signeds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}

macro_rules! unsigned {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_increment::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_increment::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t:ident, $demo_name:ident, $bench_name:ident) => {
        fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_increment::<$t>(gm, limit);
        }

        fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_increment::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_increment, benchmark_u8_increment);
unsigned!(u16, demo_u16_increment, benchmark_u16_increment);
unsigned!(u32, demo_u32_increment, benchmark_u32_increment);
unsigned!(u64, demo_u64_increment, benchmark_u64_increment);

signed!(i8, demo_i8_increment, benchmark_i8_increment);
signed!(i16, demo_i16_increment, benchmark_i16_increment);
signed!(i32, demo_i32_increment, benchmark_i32_increment);
signed!(i64, demo_i64_increment, benchmark_i64_increment);
