use common::GenerationMode;
use malachite_nz::integer::Integer;
use num::BigInt;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_i;

type It = Iterator<Item = i64>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_i())
}

pub fn random_inputs() -> Box<It> {
    Box::new(random_x(&EXAMPLE_SEED))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(_) => random_inputs(),
    }
}

pub fn demo_integer_from_i64(gm: GenerationMode, limit: usize) {
    for i in select_inputs(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

pub fn benchmark_integer_from_i64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer::from(i64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|i| Integer::from(i)),
        function_g: &(|i| BigInt::from(i)),
        x_cons: &(|&i| i),
        y_cons: &(|&i| i),
        x_param: &(|&i| (64 - i.leading_zeros()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer::from(i64)",
        x_axis_label: "i.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
