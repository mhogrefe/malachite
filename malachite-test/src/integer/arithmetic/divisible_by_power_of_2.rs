use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn demo_exhaustive_integer_divisible_by_power_of_2(limit: usize) {
    for (n, pow) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(limit) {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn demo_random_integer_divisible_by_power_of_2(limit: usize) {
    for (n, pow) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32).map(|u| u as u64)),
    ).take(limit)
    {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn benchmark_exhaustive_integer_divisible_by_power_of_2(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.divisible_by_power_of_2(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u64>()),
        function_f: &(|(n, pow): (gmp::Integer, u64)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Integer, u64)| n.divisible_by_power_of_2(pow)),
        x_cons: &(|&(ref n, pow)| (n.clone(), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_divisible_by_power_of_2(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.divisible_by_power_of_2(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|u| u as u64)),
        ),
        function_f: &(|(n, pow): (gmp::Integer, u64)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Integer, u64)| n.divisible_by_power_of_2(pow)),
        x_cons: &(|&(ref n, pow)| (n.clone(), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_divisible_by_power_of_2_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.divisible_by_power_of_2(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u64>()),
        function_f: &(|(n, pow): (native::Integer, u64)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Integer, u64)| {
                          n.trailing_zeros().map_or(true, |z| z >= pow)
                      }),
        x_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        g_name: "Integer.trailing\\\\_zeros().map\\\\_or(true, |z| z >= u64)",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_divisible_by_power_of_2_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.divisible_by_power_of_2(u64)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|u| u as u64)),
        ),
        function_f: &(|(n, pow): (native::Integer, u64)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Integer, u64)| {
                          n.trailing_zeros().map_or(true, |z| z >= pow)
                      }),
        x_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_integer_to_native(n), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        g_name: "Integer.trailing\\\\_zeros().map\\\\_or(true, |z| z >= u64)",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
