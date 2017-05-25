use common::{gmp_to_native, gmp_to_num};
use rust_wheels::benchmarks::benchmark_3;
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
    benchmark_3(exhaustive_pairs_from_single(exhaustive_naturals()),
                &(|(x, y)| x + y),
                &(|(x, y)| x + y),
                &(|(x, y)| x + y),
                &(|&(ref x, ref y)| (gmp_to_native(x), gmp_to_native(y))),
                &(|&(ref x, ref y)| (gmp_to_num(x), gmp_to_num(y))),
                &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
                limit,
                "malachite-gmp",
                "malachite-native",
                "num",
                "Natural + Natural",
                "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                "time (ns)",
                &format!("benchmarks/{}", file_name));
}

pub fn benchmark_random_natural_add(limit: usize, scale: u32, file_name: &str) {
    benchmark_3(random_pairs_from_single(random_naturals(&EXAMPLE_SEED, scale)),
                &(|(x, y)| x + y),
                &(|(x, y)| x + y),
                &(|(x, y)| x + y),
                &(|&(ref x, ref y)| (gmp_to_native(x), gmp_to_native(y))),
                &(|&(ref x, ref y)| (gmp_to_num(x), gmp_to_num(y))),
                &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
                limit,
                "malachite-gmp",
                "malachite-native",
                "num",
                "Natural + Natural",
                "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
                "time (ns)",
                &format!("benchmarks/{}", file_name));
}
