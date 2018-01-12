use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_nz::integer::Integer;
use num::BigInt;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_partial_eq_i32(x: &BigInt, i: i32) -> bool {
    *x == BigInt::from(i)
}

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

pub fn demo_integer_partial_eq_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        if n == i {
            println!("{} = {}", n, i);
        } else {
            println!("{} ≠ {}", n, i);
        }
    }
}

pub fn demo_i32_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in select_inputs_2(gm).take(limit) {
        if i == n {
            println!("{} = {}", i, n);
        } else {
            println!("{} ≠ {}", i, n);
        }
    }
}

pub fn benchmark_integer_partial_eq_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == i32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i)| n == i),
        function_g: &(|(n, i): (BigInt, i32)| num_partial_eq_i32(&n, i)),
        function_h: &(|(n, i): (rugint::Integer, i32)| n == i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        z_cons: &(|&(ref n, i)| (integer_to_rugint_integer(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer == i32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_i32_partial_eq_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} i32 == Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(i, n)| i == n),
        function_g: &(|(i, n): (i32, rugint::Integer)| i == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, integer_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "i32 == Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
