use common::gmp_natural_to_native;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_into_integer(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        let n_clone = n.clone();
        println!("into_integer({}) = {}", n_clone, n.into_integer());
    }
}

pub fn demo_random_natural_into_integer(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        let n_clone = n.clone();
        println!("into_integer({}) = {}", n_clone, n.into_integer());
    }
}

pub fn demo_exhaustive_natural_to_integer(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("to_integer(&{}) = {}", n, n.to_integer());
    }
}

pub fn demo_random_natural_to_integer(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_integer(&{}) = {}", n, n.to_integer());
    }
}

pub fn benchmark_exhaustive_natural_to_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.to_integer()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_naturals(),
        function_f: &(|n: gmp::Natural| n.into_integer()),
        function_g: &(|n: native::Natural| n.into_integer()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.to\\\\_integer()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_to_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.to_integer()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_naturals(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Natural| n.into_integer()),
        function_g: &(|n: native::Natural| n.into_integer()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.to\\\\_integer()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_to_integer_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.to_integer() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_naturals(),
        function_f: &(|n: native::Natural| n.into_integer()),
        function_g: &(|n: native::Natural| n.to_integer()),
        x_cons: &(|x| gmp_natural_to_native(x)),
        y_cons: &(|x| gmp_natural_to_native(x)),
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

pub fn benchmark_random_natural_to_integer_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.to_integer() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_naturals(&EXAMPLE_SEED, scale),
        function_f: &(|n: native::Natural| n.into_integer()),
        function_g: &(|n: native::Natural| n.to_integer()),
        x_cons: &(|x| gmp_natural_to_native(x)),
        y_cons: &(|x| gmp_natural_to_native(x)),
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
