use common::GenerationMode;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

type It = Iterator<Item = u32>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_u())
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

pub fn demo_natural_from_u32(gm: GenerationMode, limit: usize) {
    for u in select_inputs(gm).take(limit) {
        println!("from({}) = {}", u, gmp::Natural::from(u));
    }
}

pub fn benchmark_natural_from_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural::from(u32)", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|u| gmp::Natural::from(u)),
        function_g: &(|u| native::Natural::from(u)),
        function_h: &(|u| num::BigUint::from(u)),
        function_i: &(|u| rugint::Integer::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        z_cons: &(|&u| u),
        w_cons: &(|&u| u),
        x_param: &(|&u| (32 - u.leading_zeros()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural::from(u32)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
