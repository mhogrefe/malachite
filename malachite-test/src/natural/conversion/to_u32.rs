use common::{gmp_natural_to_native, gmp_natural_to_rugint_integer, GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

type It = Iterator<Item = gmp::Natural>;

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

pub fn demo_natural_to_u32(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_natural_to_u32_wrapping(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32_wrapping());
    }
}

pub fn benchmark_natural_to_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_u32()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Natural| n.to_u32()),
        function_g: &(|n: native::Natural| n.to_u32()),
        function_h: &(|n: rugint::Integer| n.to_u32()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        z_cons: &(|x| gmp_natural_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural.to\\\\_u32()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_to_u32_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_u32_wrapping()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Natural| n.to_u32_wrapping()),
        function_g: &(|n: native::Natural| n.to_u32_wrapping()),
        function_h: &(|n: rugint::Integer| n.to_u32_wrapping()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        z_cons: &(|x| gmp_natural_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural.to\\\\_u32\\\\_wrapping()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
