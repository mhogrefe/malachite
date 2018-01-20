use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

type It = Iterator<Item = Natural>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_naturals())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_naturals(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_significant_bits(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_natural_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.significant_bits()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| n.significant_bits()),
        function_g: &(|n: BigUint| n.bits()),
        function_h: &(|n: rugint::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        z_cons: &(|x| natural_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural.significant\\\\_bits()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
