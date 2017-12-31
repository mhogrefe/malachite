use common::GenerationMode;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_i;

type It = Iterator<Item = i32>;

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

pub fn demo_integer_from_i32(gm: GenerationMode, limit: usize) {
    for i in select_inputs(gm).take(limit) {
        println!("from({}) = {}", i, gmp::Integer::from(i));
    }
}

pub fn benchmark_integer_from_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer::from(i32)", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|i| gmp::Integer::from(i)),
        function_g: &(|i| native::Integer::from(i)),
        function_h: &(|i| num::BigInt::from(i)),
        function_i: &(|i| rugint::Integer::from(i)),
        x_cons: &(|&i| i),
        y_cons: &(|&i| i),
        z_cons: &(|&i| i),
        w_cons: &(|&i| i),
        x_param: &(|&i| (32 - i.leading_zeros()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer::from(i32)",
        x_axis_label: "i.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
