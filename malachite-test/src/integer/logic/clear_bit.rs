use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn demo_exhaustive_integer_clear_bit(limit: usize) {
    for (mut n, index) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

pub fn demo_random_integer_clear_bit(limit: usize) {
    for (mut n, index) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_exhaustive_integer_clear_bit(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.clear_bit(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u64>()),
        function_f: &(|(mut n, index): (gmp::Integer, u64)| n.clear_bit(index)),
        function_g: &(|(mut n, index): (native::Integer, u64)| n.clear_bit(index)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.clear\\\\_bit(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_clear_bit(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.clear_bit(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i as u64)),
        ),
        function_f: &(|(mut n, index): (gmp::Integer, u64)| n.clear_bit(index)),
        function_g: &(|(mut n, index): (native::Integer, u64)| n.clear_bit(index)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.clear\\\\_bit(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
