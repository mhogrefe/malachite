use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use inputs::base::{signeds_no_max, unsigneds_no_max};

fn demo_unsigned_increment<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for mut n in unsigneds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn demo_signed_increment<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for mut n in signeds_no_max::<T>(gm).take(limit) {
        let n_old = n;
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn benchmark_unsigned_increment<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Ordinary,
        unsigneds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|mut n| n.increment()))],
    );
}

fn benchmark_signed_increment<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.increment()", T::NAME),
        BenchmarkType::Ordinary,
        signeds_no_max::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|mut n| n.increment()))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_increment::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_increment::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_increment::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
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
