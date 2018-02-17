use common::GenerationMode;
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use inputs::base::{signeds_no_max, unsigneds_no_max};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

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
    println!("benchmarking {} {}.increment()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: unsigneds_no_max(gm),
        function_f: &(|mut n: T| n.increment()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.increment()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_signed_increment<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.increment()", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: signeds_no_max(gm),
        function_f: &(|mut n: T| n.increment()),
        x_cons: &(|&n| n),
        x_param: &(|&n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.increment()", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_increment::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_increment::<$t>(gm, limit, file_name);
        }
    }
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_increment::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_increment::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u8, demo_u8_increment, benchmark_u8_increment);
unsigned!(u16, demo_u16_increment, benchmark_u16_increment);
unsigned!(u32, demo_u32_increment, benchmark_u32_increment);
unsigned!(u64, demo_u64_increment, benchmark_u64_increment);

signed!(i8, demo_i8_increment, benchmark_i8_increment);
signed!(i16, demo_i16_increment, benchmark_i16_increment);
signed!(i32, demo_i32_increment, benchmark_i32_increment);
signed!(i64, demo_i64_increment, benchmark_i64_increment);
