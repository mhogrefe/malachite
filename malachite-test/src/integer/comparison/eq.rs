use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

type It = Iterator<Item = (Integer, Integer)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs_from_single(exhaustive_integers()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs_from_single(random_integers(
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

pub fn demo_integer_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_integer_eq(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == Integer", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_bigint(x), integer_to_bigint(y))),
        z_cons: &(|&(ref x, ref y)| (integer_to_rugint_integer(x), integer_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer == Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
