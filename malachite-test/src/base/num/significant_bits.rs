use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use inputs::base::{signeds, unsigneds};

fn demo_unsigned_significant_bits<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) {
    for n in unsigneds::<T>(gm).take(limit) {
        println!("{}.significant_bits() = {}", n, n.significant_bits());
    }
}

fn demo_signed_significant_bits<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for n in signeds::<T>(gm).take(limit) {
        println!("{}.significant_bits() = {}", n, n.significant_bits());
    }
}

fn benchmark_unsigned_significant_bits<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.significant_bits()", T::NAME),
        BenchmarkType::Ordinary,
        unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|n| no_out!(n.significant_bits())))],
    );
}

fn benchmark_signed_significant_bits<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.significant_bits()", T::NAME),
        BenchmarkType::Ordinary,
        signeds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|n| no_out!(n.significant_bits())))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_significant_bits::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_significant_bits::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_significant_bits::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_significant_bits::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_significant_bits, benchmark_u8_significant_bits);
unsigned!(
    u16,
    demo_u16_significant_bits,
    benchmark_u16_significant_bits
);
unsigned!(
    u32,
    demo_u32_significant_bits,
    benchmark_u32_significant_bits
);
unsigned!(
    u64,
    demo_u64_significant_bits,
    benchmark_u64_significant_bits
);

signed!(i8, demo_i8_significant_bits, benchmark_i8_significant_bits);
signed!(
    i16,
    demo_i16_significant_bits,
    benchmark_i16_significant_bits
);
signed!(
    i32,
    demo_i32_significant_bits,
    benchmark_i32_significant_bits
);
signed!(
    i64,
    demo_i64_significant_bits,
    benchmark_i64_significant_bits
);
