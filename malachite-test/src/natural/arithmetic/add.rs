use common::{gmp_natural_to_native, gmp_natural_to_num_biguint};
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

pub fn demo_exhaustive_natural_add(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

pub fn demo_random_natural_add(limit: usize) {
    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

pub fn benchmark_exhaustive_natural_add(limit: usize, file_name: &str) {
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()),
                    function_f: &(|(x, y)| x + y),
                    function_g: &(|(x, y)| x + y),
                    function_h: &(|(x, y)| x + y),
                    x_to_y: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_to_z: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    x_size: &(|&(ref x, ref y)| {
                                  max(x.significant_bits(), y.significant_bits()) as usize
                              }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural + Natural",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_add(limit: usize, scale: u32, file_name: &str) {
    benchmark_3(BenchmarkOptions3 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                    function_f: &(|(x, y)| x + y),
                    function_g: &(|(x, y)| x + y),
                    function_h: &(|(x, y)| x + y),
                    x_to_y: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_to_z: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    x_size: &(|&(ref x, ref y)| {
                                  max(x.significant_bits(), y.significant_bits()) as usize
                              }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural + Natural",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
