use common::GenerationMode;
use malachite_base::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::Ordering;

type It1 = Iterator<Item = (Integer, i32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_i()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (i32, Integer)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs(exhaustive_i(), exhaustive_integers()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

pub fn demo_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in select_inputs_1(gm).take(limit) {
        match PartialOrdAbs::partial_cmp_abs(&i, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

pub fn benchmark_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.partial_cmp_abs(&i32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i): (Integer, i32)| n.partial_cmp_abs(&i)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} i32.partial_cmp_abs(&Integer)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs_2(gm),
        function_f: &(|(i, n): (i32, Integer)| PartialOrdAbs::partial_cmp_abs(&i, &n)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "i32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
