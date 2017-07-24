use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::{max, Ordering};

pub fn demo_exhaustive_integer_cmp(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn demo_random_integer_cmp(limit: usize) {
    for (x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_exhaustive_integer_cmp(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.cmp(&Integer)");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_pairs_from_single(exhaustive_integers()),
                    function_f: &(|(x, y): (gmp::Integer, gmp::Integer)| x.cmp(&y)),
                    function_g: &(|(x, y): (native::Integer, native::Integer)| x.cmp(&y)),
                    function_h: &(|(x, y): (num::BigInt, num::BigInt)| x.cmp(&y)),
                    function_i: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp(&y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_integer_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Integer.cmp(\\\\&Integer)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_cmp(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.cmp(&Integer)");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
                    function_f: &(|(x, y): (gmp::Integer, gmp::Integer)| x.cmp(&y)),
                    function_g: &(|(x, y): (native::Integer, native::Integer)| x.cmp(&y)),
                    function_h: &(|(x, y): (num::BigInt, num::BigInt)| x.cmp(&y)),
                    function_i: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp(&y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_integer_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))
                              }),
                    w_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Integer.cmp(\\\\&Integer)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
