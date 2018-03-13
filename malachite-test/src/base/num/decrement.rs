use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use inputs::base::{positive_unsigneds, signeds_no_min};

fn demo_unsigned_decrement<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for mut n in positive_unsigneds::<T>(gm).take(limit) {
        let n_old = n;
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

fn demo_signed_decrement<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for mut n in signeds_no_min::<T>(gm).take(limit) {
        let n_old = n;
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

fn benchmark_unsigned_decrement<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.decrement()", T::NAME),
        BenchmarkType::Single,
        positive_unsigneds::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|mut n| n.decrement()))],
    );
}

fn benchmark_signed_decrement<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.decrement()", T::NAME),
        BenchmarkType::Single,
        signeds_no_min::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&n| n.significant_bits() as usize),
        "index",
        &[("malachite", &mut (|mut n| n.decrement()))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_decrement::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_decrement::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_decrement::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_decrement::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_decrement, benchmark_u8_decrement);
unsigned!(u16, demo_u16_decrement, benchmark_u16_decrement);
unsigned!(u32, demo_u32_decrement, benchmark_u32_decrement);
unsigned!(u64, demo_u64_decrement, benchmark_u64_decrement);

signed!(i8, demo_i8_decrement, benchmark_i8_decrement);
signed!(i16, demo_i16_decrement, benchmark_i16_decrement);
signed!(i32, demo_i32_decrement, benchmark_i32_decrement);
signed!(i64, demo_i64_decrement, benchmark_i64_decrement);
