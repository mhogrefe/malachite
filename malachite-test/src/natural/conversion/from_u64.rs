use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

pub fn demo_exhaustive_natural_from_u64(limit: usize) {
    for u in exhaustive_u::<u64>().take(limit) {
        println!("from({}) = {}", u, gmp::Natural::from(u));
    }
}

pub fn demo_random_natural_from_u64(limit: usize) {
    for u in random_x::<u64>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", u, gmp::Natural::from(u));
    }
}

pub fn benchmark_exhaustive_natural_from_u64(limit: usize, file_name: &str) {
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_u::<u64>(),
                    function_f: &(|u| gmp::Natural::from(u)),
                    function_g: &(|u| native::Natural::from(u)),
                    function_h: &(|u| num::BigUint::from(u)),
                    x_to_y: &(|&u| u),
                    x_to_z: &(|&u| u),
                    x_param: &(|&u| (64 - u.leading_zeros()) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural::from(u64)",
                    x_axis_label: "u.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_from_u64(limit: usize, file_name: &str) {
    benchmark_3(BenchmarkOptions3 {
                    xs: random_x::<u64>(&EXAMPLE_SEED),
                    function_f: &(|u| gmp::Natural::from(u)),
                    function_g: &(|u| native::Natural::from(u)),
                    function_h: &(|u| num::BigUint::from(u)),
                    x_to_y: &(|&u| u),
                    x_to_z: &(|&u| u),
                    x_param: &(|&u| (64 - u.leading_zeros()) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural::from(u64)",
                    x_axis_label: "u.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
