use common::GenerationMode;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random_x, range_increasing_x};
use rust_wheels::iterators::primitive_ints::{exhaustive_i, exhaustive_u, random_range};
use rust_wheels::iterators::tuples::{lex_pairs, random_pairs};

type It<T> = Iterator<Item = (T, u64)>;

pub fn exhaustive_inputs_u<T: 'static + PrimitiveUnsigned>() -> Box<It<T>> {
    Box::new(lex_pairs(
        exhaustive_u(),
        range_increasing_x(0, u64::from(T::WIDTH) - 1),
    ))
}

pub fn exhaustive_inputs_i<T: 'static + PrimitiveSigned>() -> Box<It<T>> {
    Box::new(lex_pairs(
        exhaustive_i(),
        range_increasing_x(0, u64::from(T::WIDTH) - 1),
    ))
}

pub fn random_inputs<T: 'static + PrimitiveInteger>() -> Box<It<T>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_range(seed, 0, T::WIDTH - 1).map(|i| i.into())),
    ))
}

pub fn select_inputs_u<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> Box<It<T>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_u(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

pub fn select_inputs_i<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> Box<It<T>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_i(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

fn demo_unsigned_flip_bit<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs_u::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_flip_bit<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs_i::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_unsigned_flip_bit<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.flip_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_u(gm),
        function_f: &(|(mut n, index): (T, u64)| n.flip_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.flip\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_signed_flip_bit<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.flip_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_i(gm),
        function_f: &(|(mut n, index): (T, u64)| n.flip_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.flip\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_flip_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_flip_bit::<$t>(gm, limit, file_name);
        }
    }
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_flip_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_flip_bit::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u8, demo_u8_flip_bit, benchmark_u8_flip_bit);
unsigned!(u16, demo_u16_flip_bit, benchmark_u16_flip_bit);
unsigned!(u32, demo_u32_flip_bit, benchmark_u32_flip_bit);
unsigned!(u64, demo_u64_flip_bit, benchmark_u64_flip_bit);

signed!(i8, demo_i8_flip_bit, benchmark_i8_flip_bit);
signed!(i16, demo_i16_flip_bit, benchmark_i16_flip_bit);
signed!(i32, demo_i32_flip_bit, benchmark_i32_flip_bit);
signed!(i64, demo_i64_flip_bit, benchmark_i64_flip_bit);
