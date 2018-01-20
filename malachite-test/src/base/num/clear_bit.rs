use common::GenerationMode;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::primitive_ints::{exhaustive_i, exhaustive_u};
use rust_wheels::iterators::tuples::{random_pairs, sqrt_pairs};

type It<T> = Iterator<Item = (T, u64)>;

pub fn exhaustive_inputs_u<T: 'static + PrimitiveUnsigned>() -> Box<It<T>> {
    Box::new(sqrt_pairs(exhaustive_u(), exhaustive_u()))
}

pub fn exhaustive_inputs_i<T: 'static + PrimitiveSigned>() -> Box<It<T>> {
    Box::new(
        sqrt_pairs(exhaustive_i(), exhaustive_u())
            .filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

pub fn random_inputs_u<T: 'static + PrimitiveInteger>(scale: u32) -> Box<It<T>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
    ))
}

pub fn random_inputs_i<T: 'static + PrimitiveInteger>(scale: u32) -> Box<It<T>> {
    Box::new(
        random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x(seed)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
        ).filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

pub fn select_inputs_u<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> Box<It<T>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_u(),
        GenerationMode::Random(scale) => random_inputs_u(scale),
    }
}

pub fn select_inputs_i<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> Box<It<T>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_i(),
        GenerationMode::Random(scale) => random_inputs_i(scale),
    }
}

fn demo_unsigned_clear_bit<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs_u::<T>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_clear_bit<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs_i::<T>(gm).take(limit) {
        let n_old = n;
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_unsigned_clear_bit<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.clear_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_u(gm),
        function_f: &(|(mut n, index): (T, u64)| n.clear_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.clear\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

fn benchmark_signed_clear_bit<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} {}.set_bit(u64)", gm.name(), T::NAME);
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_i(gm),
        function_f: &(|(mut n, index): (T, u64)| n.clear_bit(index)),
        x_cons: &(|&p| p),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        title: &format!("{}.clear\\\\_bit(u64)", T::NAME),
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_clear_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_clear_bit::<$t>(gm, limit, file_name);
        }
    }
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_clear_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_clear_bit::<$t>(gm, limit, file_name);
        }
    }
}

unsigned!(u8, demo_u8_clear_bit, benchmark_u8_clear_bit);
unsigned!(u16, demo_u16_clear_bit, benchmark_u16_clear_bit);
unsigned!(u32, demo_u32_clear_bit, benchmark_u32_clear_bit);
unsigned!(u64, demo_u64_clear_bit, benchmark_u64_clear_bit);

signed!(i8, demo_i8_clear_bit, benchmark_i8_clear_bit);
signed!(i16, demo_i16_clear_bit, benchmark_i16_clear_bit);
signed!(i32, demo_i32_clear_bit, benchmark_i32_clear_bit);
signed!(i64, demo_i64_clear_bit, benchmark_i64_clear_bit);
