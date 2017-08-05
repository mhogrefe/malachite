use common::{gmp_integer_to_native, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::cmp::max;

pub fn demo_exhaustive_integer_flip_bit(limit: usize) {
    for (mut n, index) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

pub fn demo_random_integer_flip_bit(limit: usize) {
    for (mut n, index) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_exhaustive_integer_flip_bit(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.flip_bit(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u64>()),
        function_f: &(|(mut n, index): (gmp::Integer, u64)| n.flip_bit(index)),
        function_g: &(|(mut n, index): (native::Integer, u64)| n.flip_bit(index)),
        function_h: &(|(mut n, index): (rugint::Integer, u64)| { n.invert_bit(index as u32); }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.flip\\\\_bit(u64)",
        x_axis_label: "max(n.significant\\\\_bits(), index)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_flip_bit(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.flip_bit(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i as u64)),
        ),
        function_f: &(|(mut n, index): (gmp::Integer, u64)| n.flip_bit(index)),
        function_g: &(|(mut n, index): (native::Integer, u64)| n.flip_bit(index)),
        function_h: &(|(mut n, index): (rugint::Integer, u64)| { n.invert_bit(index as u32); }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Integer.flip\\\\_bit(u64)",
        x_axis_label: "max(n.significant\\\\_bits(), index)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
