use common::{integer_to_rugint_integer, natural_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::max;

type It = Iterator<Item = (Integer, Natural)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs(
        exhaustive_integers(),
        exhaustive_naturals(),
    ))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_naturals(seed, 32)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_integer_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_integer_assign_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(Natural)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
        function_f: &(|(mut x, y): (Integer, Natural)| x.assign(y)),
        function_g: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer.assign(Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.assign(Natural) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
        function_f: &(|(mut x, y): (Integer, Natural)| x.assign(y)),
        function_g: &(|(mut x, y): (Integer, Natural)| x.assign(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Integer.assign(Natural)",
        g_name: "Integer.assign(\\\\&Natural)",
        title: "Integer.assign(Natural) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
