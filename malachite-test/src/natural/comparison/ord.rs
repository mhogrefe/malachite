use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::{max, Ordering};

pub fn demo_exhaustive_natural_cmp(limit: usize) {
    for (x, y) in exhaustive_pairs_from_single(exhaustive_naturals()).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn demo_random_natural_cmp(limit: usize) {
    for (x, y) in random_pairs_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_exhaustive_natural_cmp(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.cmp(&Natural)");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs_from_single(exhaustive_naturals()),
        function_f: &(|(x, y): (gmp::Natural, gmp::Natural)| x.cmp(&y)),
        function_g: &(|(x, y): (native::Natural, native::Natural)| x.cmp(&y)),
        function_h: &(|(x, y): (num::BigUint, num::BigUint)| x.cmp(&y)),
        function_i: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))),
        w_cons: &(|&(ref x, ref y)| {
            (
                gmp_natural_to_rugint_integer(x),
                gmp_natural_to_rugint_integer(y),
            )
        }),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural.cmp(\\\\&Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_cmp(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.cmp(&Natural)");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(x, y): (gmp::Natural, gmp::Natural)| x.cmp(&y)),
        function_g: &(|(x, y): (native::Natural, native::Natural)| x.cmp(&y)),
        function_h: &(|(x, y): (num::BigUint, num::BigUint)| x.cmp(&y)),
        function_i: &(|(x, y): (rugint::Integer, rugint::Integer)| x.cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))),
        w_cons: &(|&(ref x, ref y)| {
            (
                gmp_natural_to_rugint_integer(x),
                gmp_natural_to_rugint_integer(y),
            )
        }),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural.cmp(\\\\&Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
