use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_significant_bits(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn demo_random_natural_significant_bits(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_exhaustive_natural_significant_bits(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.significant_bits()");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_naturals(),
                    function_f: &(|n: gmp::Natural| n.significant_bits()),
                    function_g: &(|n: native::Natural| n.significant_bits()),
                    function_h: &(|n: num::BigUint| n.bits()),
                    function_i: &(|n: rugint::Integer| n.significant_bits()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_natural_to_native(x)),
                    z_cons: &(|x| gmp_natural_to_num_biguint(x)),
                    w_cons: &(|x| gmp_natural_to_rugint_integer(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.significant\\\\_bits()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_significant_bits(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.significant_bits()");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_naturals(&EXAMPLE_SEED, scale),
                    function_f: &(|n: gmp::Natural| n.significant_bits()),
                    function_g: &(|n: native::Natural| n.significant_bits()),
                    function_h: &(|n: num::BigUint| n.bits()),
                    function_i: &(|n: rugint::Integer| n.significant_bits()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_natural_to_native(x)),
                    z_cons: &(|x| gmp_natural_to_num_biguint(x)),
                    w_cons: &(|x| gmp_natural_to_rugint_integer(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural.significant\\\\_bits()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
