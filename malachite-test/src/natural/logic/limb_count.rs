use common::gmp_natural_to_native;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_limb_count(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

pub fn demo_random_natural_limb_count(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

pub fn benchmark_exhaustive_natural_limb_count(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.limb_count()");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_naturals(),
                    function_f: &(|n| n.limb_count()),
                    function_g: &(|n: native::Natural| n.limb_count()),
                    x_to_y: &(|x| gmp_natural_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural.limb\\\\_count()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_limb_count(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.limb_count()");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_naturals(&EXAMPLE_SEED, scale),
                    function_f: &(|n| n.limb_count()),
                    function_g: &(|n: native::Natural| n.limb_count()),
                    x_to_y: &(|x| gmp_natural_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Natural.limb\\\\_count()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
