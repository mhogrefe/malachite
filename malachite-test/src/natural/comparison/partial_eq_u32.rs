use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_partial_eq_u32(x: &BigUint, u: u32) -> bool {
    *x == BigUint::from(u)
}

type It1 = Iterator<Item = (Natural, u32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (u32, Natural)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs(exhaustive_u(), exhaustive_naturals()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_natural_partial_eq_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_1(gm).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

pub fn demo_u32_partial_eq_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in select_inputs_2(gm).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

pub fn benchmark_natural_partial_eq_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural == u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, u)| n == u),
        function_g: &(|(n, u): (BigUint, u32)| num_partial_eq_u32(&n, u)),
        function_h: &(|(n, u): (rugint::Integer, u32)| n == u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        z_cons: &(|&(ref n, u)| (natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural == u32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_partial_eq_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 == Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(u, n)| u == n),
        function_g: &(|(u, n): (u32, rugint::Integer)| u == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, natural_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "u32 == Natural",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
