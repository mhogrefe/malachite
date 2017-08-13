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

pub fn demo_exhaustive_integer_get_bit(limit: usize) {
    for (n, index) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

pub fn demo_random_integer_get_bit(limit: usize) {
    for (n, index) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)),
    ).take(limit)
    {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

pub fn benchmark_exhaustive_integer_get_bit(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.get_bit(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u64>()),
        function_f: &(|(n, index): (gmp::Integer, u64)| n.get_bit(index)),
        function_g: &(|(n, index): (native::Integer, u64)| n.get_bit(index)),
        function_h: &(|(n, index): (rugint::Integer, u64)| n.get_bit(index as u32)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.get\\\\_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_get_bit(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.get_bit(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i as u64)),
        ),
        function_f: &(|(n, index): (gmp::Integer, u64)| n.get_bit(index)),
        function_g: &(|(n, index): (native::Integer, u64)| n.get_bit(index)),
        function_h: &(|(n, index): (rugint::Integer, u64)| n.get_bit(index as u32)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.get\\\\_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}