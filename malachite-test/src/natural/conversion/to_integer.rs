use common::GenerationMode;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
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

pub fn demo_natural_into_integer(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        let n_clone = n.clone();
        println!("into_integer({}) = {}", n_clone, n.into_integer());
    }
}

pub fn demo_natural_to_integer(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("to_integer(&{}) = {}", n, n.to_integer());
    }
}

pub fn benchmark_natural_to_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_integer()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| n.into_integer()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.to\\\\_integer()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_to_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.to_integer() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: Natural| n.into_integer()),
        function_g: &(|n: Natural| n.to_integer()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "into\\\\_integer (by value)",
        g_name: "to\\\\_integer (by reference)",
        title: "Natural.to\\\\_integer()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
