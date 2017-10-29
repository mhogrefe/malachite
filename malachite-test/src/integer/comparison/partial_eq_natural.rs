use common::{gmp_integer_to_native, gmp_integer_to_rugint, gmp_natural_to_native,
             gmp_natural_to_rugint_integer};
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::max;

pub fn demo_exhaustive_integer_partial_eq_natural(limit: usize) {
    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn demo_random_integer_partial_eq_natural(limit: usize) {
    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
    ).take(limit)
    {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn demo_exhaustive_natural_partial_eq_integer(limit: usize) {
    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn demo_random_natural_partial_eq_integer(limit: usize) {
    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_eq_natural(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer == Natural");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer == Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_partial_eq_natural(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer == Natural");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        ),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer == Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_partial_eq_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural == Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural == Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_partial_eq_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural == Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural == Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
