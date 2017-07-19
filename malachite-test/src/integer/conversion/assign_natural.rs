use common::{gmp_integer_to_native, gmp_integer_to_rugint, gmp_natural_to_native,
             gmp_natural_to_rugint_integer};
use malachite_gmp as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use malachite_native as native;
use malachite_native::traits::Assign as native_assign;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2, BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::max;

pub fn demo_exhaustive_integer_assign_natural(limit: usize) {
    for (mut x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_random_integer_assign_natural(limit: usize) {
    for (mut x, y) in random_pairs(&EXAMPLE_SEED,
                                   &(|seed| random_integers(seed, 32)),
                                   &(|seed| random_naturals(seed, 32)))
                .take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_exhaustive_integer_assign_natural_ref(limit: usize) {
    for (mut x, y) in exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_integer_assign_natural_ref(limit: usize) {
    for (mut x, y) in random_pairs(&EXAMPLE_SEED,
                                   &(|seed| random_integers(seed, 32)),
                                   &(|seed| random_naturals(seed, 32)))
                .take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_exhaustive_integer_assign_natural(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(Natural)");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
                    function_f: &(|(mut x, y): (gmp::integer::Integer, gmp::natural::Natural)| {
                                      x.assign(y)
                                  }),
                    function_g: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(y)),
                    function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.assign(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_assign_natural(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.assign(Natural)");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_integers(seed, scale)),
                                     &(|seed| random_naturals(seed, scale))),
                    function_f: &(|(mut x, y): (gmp::integer::Integer, gmp::natural::Natural)| {
                                      x.assign(y)
                                  }),
                    function_g: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(y)),
                    function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    z_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_rugint(x), gmp_natural_to_rugint_integer(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.assign(Natural)",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_integer_assign_natural_evaluation_strategy(limit: usize,
                                                                       file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(Natural) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_pairs(exhaustive_integers(), exhaustive_naturals()),
                    function_f: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(y)),
                    function_g: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(&y)),
                    x_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "Integer.assign(Natural)",
                    g_name: "Integer.assign(\\\\&Natural)",
                    title: "Integer.assign(Natural) evaluation strategy",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_assign_natural_evaluation_strategy(limit: usize,
                                                                   scale: u32,
                                                                   file_name: &str) {
    println!("benchmarking random Integer.assign(Natural) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_integers(seed, scale)),
                                     &(|seed| random_naturals(seed, scale))),
                    function_f: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(y)),
                    function_g: &(|(mut x, y): (native::integer::Integer,
                                                native::natural::Natural)| x.assign(&y)),
                    x_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    y_cons: &(|&(ref x, ref y)| {
                                  (gmp_integer_to_native(x), gmp_natural_to_native(y))
                              }),
                    x_param: &(|&(ref x, ref y)| {
                                   max(x.significant_bits(), y.significant_bits()) as usize
                               }),
                    limit: limit,
                    f_name: "Integer.assign(Natural)",
                    g_name: "Integer.assign(\\\\&Natural)",
                    title: "Integer.assign(Natural) evaluation strategy",
                    x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
