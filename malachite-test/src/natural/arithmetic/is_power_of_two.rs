use common::gmp_natural_to_native;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_is_power_of_two(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        if n.is_power_of_two() {
            println!("{} is a power of two", n);
        } else {
            println!("{} is not a power of two", n);
        }
    }
}

pub fn demo_random_natural_is_power_of_two(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        if n.is_power_of_two() {
            println!("{} is a power of two", n);
        } else {
            println!("{} is not a power of two", n);
        }
    }
}

pub fn benchmark_exhaustive_natural_is_power_of_two(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.is_power_of_two()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_naturals(),
        function_f: &(|n: gmp::Natural| n.is_power_of_two()),
        function_g: &(|n: native::Natural| n.is_power_of_two()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.is\\\\_power\\\\_of\\\\_two()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_is_power_of_two(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.is_power_of_two()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_naturals(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Natural| n.is_power_of_two()),
        function_g: &(|n: native::Natural| n.is_power_of_two()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.is\\\\_power\\\\_of\\\\_two()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
