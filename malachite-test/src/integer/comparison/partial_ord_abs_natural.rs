use common::{gmp_integer_to_native, gmp_integer_to_rugint, gmp_natural_to_native,
             gmp_natural_to_rugint_integer};
use malachite_gmp as gmp;
use malachite_gmp::traits::PartialOrdAbs as gmp_partial_ord_abs;
use malachite_native as native;
use malachite_native::traits::PartialOrdAbs as native_partial_ord_abs;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::{max, Ordering};

pub fn demo_exhaustive_integer_partial_cmp_abs_natural(limit: usize) {
    for (x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

pub fn demo_random_integer_partial_cmp_abs_natural(limit: usize) {
    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_naturals(seed, 32)),
    ).take(limit)
    {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

pub fn demo_exhaustive_natural_partial_cmp_abs_integer(limit: usize) {
    for (x, y) in exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()).take(limit) {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

pub fn demo_random_natural_partial_cmp_abs_integer(limit: usize) {
    for (x, y) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        match x.partial_cmp_abs(&y).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_cmp_abs_natural(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.partial_cmp_abs(&Natural)");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
        function_f: &(|(x, y): (gmp::integer::Integer, gmp::natural::Natural)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_g: &(|(x, y): (native::integer::Integer, native::natural::Natural)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp_abs(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_partial_cmp_abs_natural(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.partial_cmp_abs(&Natural)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_naturals(seed, scale)),
        ),
        function_f: &(|(x, y): (gmp::integer::Integer, gmp::natural::Natural)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_g: &(|(x, y): (native::integer::Integer, native::natural::Natural)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp_abs(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_partial_cmp_abs_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.partial_cmp_abs(&Integer)");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_naturals(), exhaustive_integers()),
        function_f: &(|(x, y): (gmp::natural::Natural, gmp::integer::Integer)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_g: &(|(x, y): (native::natural::Natural, native::integer::Integer)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp_abs(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_partial_cmp_abs_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.partial_cmp_abs(&Integer)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(x, y): (gmp::natural::Natural, gmp::integer::Integer)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_g: &(|(x, y): (native::natural::Natural, native::integer::Integer)| {
                          x.partial_cmp_abs(&y)
                      }),
        function_h: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp_abs(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
