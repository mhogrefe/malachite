use common::GenerationMode;
use inputs::base::{pairs_of_signed_and_small_u64, pairs_of_unsigned_and_small_u64};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

fn demo_unsigned_get_bit<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_unsigned_and_small_u64::<T>(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn demo_signed_get_bit<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_signed_and_small_u64::<T>(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_unsigned_get_bit<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.get_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_unsigned_and_small_u64(gm),
        function_f: &mut (|(n, index): (T, u64)| n.get_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.get\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_signed_get_bit<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.get_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_signed_and_small_u64(gm),
        function_f: &mut (|(n, index): (T, u64)| n.get_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.get\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_get_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_get_bit::<$t>(gm, limit, file_name);
        }
    }
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_get_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_get_bit::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u8, demo_u8_get_bit, benchmark_u8_get_bit);
unsigned!(u16, demo_u16_get_bit, benchmark_u16_get_bit);
unsigned!(u32, demo_u32_get_bit, benchmark_u32_get_bit);
unsigned!(u64, demo_u64_get_bit, benchmark_u64_get_bit);

signed!(i8, demo_i8_get_bit, benchmark_i8_get_bit);
signed!(i16, demo_i16_get_bit, benchmark_i16_get_bit);
signed!(i32, demo_i32_get_bit, benchmark_i32_get_bit);
signed!(i64, demo_i64_get_bit, benchmark_i64_get_bit);
