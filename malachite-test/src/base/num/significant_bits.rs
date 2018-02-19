use common::GenerationMode;
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use inputs::base::{signeds, unsigneds};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

fn demo_unsigned_significant_bits<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
) {
    for n in unsigneds::<T>(gm).take(limit) {
        println!("n.significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn demo_signed_significant_bits<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for n in signeds::<T>(gm).take(limit) {
        println!("n.significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn benchmark_unsigned_significant_bits<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.significant_bits()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: unsigneds(gm),
        function_f: &mut (|n: T| n.significant_bits()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.significant\\\\_bits()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_signed_significant_bits<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.significant_bits()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: signeds(gm),
        function_f: &mut (|n: T| n.significant_bits()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.significant\\\\_bits()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_significant_bits::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_significant_bits::<$t>(gm, limit, file_name);
        }
    }
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_significant_bits::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_significant_bits::<$t>(gm, limit, file_name);
        }
    }
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
