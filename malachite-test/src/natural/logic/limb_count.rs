use common::GenerationMode;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
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

pub fn demo_natural_limb_count(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

pub fn benchmark_natural_limb_count(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.limb_count()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| n.limb_count()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.limb\\\\_count()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
