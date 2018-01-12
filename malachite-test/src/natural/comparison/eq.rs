use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

type It = Iterator<Item = (Natural, Natural)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs_from_single(random_naturals(
        &EXAMPLE_SEED,
        scale,
    )))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_natural_eq(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural == Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural == Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
