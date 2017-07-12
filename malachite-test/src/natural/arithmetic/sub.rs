use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2, BenchmarkOptions3, benchmark_3,
                              BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

pub fn num_sub(mut x: num::BigUint, y: num::BigUint) -> Option<num::BigUint> {
    if x >= y {
        x = x - y;
        Some(x)
    } else {
        None
    }
}

pub fn rugint_sub(x: rugint::Integer, y: rugint::Integer) -> Option<rugint::Integer> {
    if x >= y { Some(x - y) } else { None }
}

//TODO use subset_pairs
pub fn demo_exhaustive_natural_sub_assign(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_naturals())
            .filter(|&(ref x, ref y)| x >= y)
            .take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_random_natural_sub_assign(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32))
            .filter(|&(ref x, ref y)| x >= y)
            .take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_exhaustive_natural_sub(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {:?}", x_old, y, x - &y);
    }
}

pub fn demo_random_natural_sub(limit: usize) {
    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {:?}", x_old, y, x - &y);
    }
}

pub fn demo_exhaustive_natural_sub_ref_ref(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        println!("&{} - &{} = {:?}", x, y, &x - &y);
    }
}

pub fn demo_random_natural_sub_ref_ref(limit: usize) {
    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        println!("&{} - &{} = {:?}", x, y, &x - &y);
    }
}

//TODO use subset_pairs
pub fn benchmark_exhaustive_natural_sub_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural -= &Natural");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()).filter(|&(ref x,
                                                                                      ref y)| {
                                                                                       x >= y
                                                                                   }),
                    function_f: &(|(mut x, y)| x -= &y),
                    function_g: &(|(mut x, y): (native::Natural, native::Natural)| x -= &y),
                    function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x -= &y),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Natural -= \\\\&Natural",
                    x_axis_label: "other",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_sub_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural -= &Natural");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale))
                        .filter(|&(ref x, ref y)| x >= y),
                    function_f: &(|(mut x, y)| x -= &y),
                    function_g: &(|(mut x, y): (native::Natural, native::Natural)| x -= &y),
                    function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x -= &y),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Natural -= \\\\&Natural",
                    x_axis_label: "other",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_natural_sub(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural - &Natural");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()),
                    function_f: &(|(x, y)| x - &y),
                    function_g: &(|(x, y)| x - &y),
                    function_h: &(|(x, y)| num_sub(x, y)),
                    function_i: &(|(x, y)| rugint_sub(x, y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural - \\\\&Natural",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_sub(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural - &Natural");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                    function_f: &(|(x, y)| x - &y),
                    function_g: &(|(x, y)| x - &y),
                    function_h: &(|(x, y)| num_sub(x, y)),
                    function_i: &(|(x, y)| rugint_sub(x, y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_rugint_integer(x),
                                   gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Natural - \\\\&Natural",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_natural_sub_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural - Natural evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_pairs_from_single(exhaustive_naturals()),
                    function_f: &(|(x, y)| x - &y),
                    function_g: &(|(x, y)| &x - &y),
                    x_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "Natural - \\\\&Natural",
                    g_name: "\\\\&Natural - \\\\&Natural",
                    title: "Natural + Natural evaluation strategy",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_sub_evaluation_strategy(limit: usize,
                                                        scale: u32,
                                                        file_name: &str) {
    println!("benchmarking random Natural - Natural evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                    function_f: &(|(x, y)| x - &y),
                    function_g: &(|(x, y)| &x - &y),
                    x_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_natural_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "Natural - \\\\&Natural",
                    g_name: "\\\\&Natural - \\\\&Natural",
                    title: "Natural + Natural evaluation strategy",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}