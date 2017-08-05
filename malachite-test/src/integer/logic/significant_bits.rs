use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_significant_bits(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn demo_random_integer_significant_bits(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_exhaustive_integer_significant_bits(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.significant_bits()");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.significant_bits()),
        function_g: &(|n: native::Integer| n.significant_bits()),
        function_h: &(|n: num::BigInt| n.bits()),
        function_i: &(|n: rugint::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        w_cons: &(|x| gmp_integer_to_rugint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.significant\\\\_bits()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_significant_bits(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.significant_bits()");
    benchmark_4(BenchmarkOptions4 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.significant_bits()),
        function_g: &(|n: native::Integer| n.significant_bits()),
        function_h: &(|n: num::BigInt| n.bits()),
        function_i: &(|n: rugint::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        w_cons: &(|x| gmp_integer_to_rugint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.significant\\\\_bits()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
