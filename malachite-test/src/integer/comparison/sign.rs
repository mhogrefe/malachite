use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use num::bigint::Sign;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}

type It = Iterator<Item = Integer>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_integers())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_integers(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_sign(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

pub fn benchmark_integer_sign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|n: Integer| n.sign()),
        function_g: &(|n: BigInt| num_sign(&n)),
        function_h: &(|n: rugint::Integer| n.sign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer.sign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
