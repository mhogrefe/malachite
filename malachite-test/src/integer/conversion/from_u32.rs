use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

pub fn demo_exhaustive_integer_from_u32(limit: usize) {
    for u in exhaustive_u::<u32>().take(limit) {
        println!("from({}) = {}", u, gmp::Integer::from(u));
    }
}

pub fn demo_random_integer_from_u32(limit: usize) {
    for u in random_x::<u32>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", u, gmp::Integer::from(u));
    }
}

pub fn benchmark_exhaustive_integer_from_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from(u32)");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_u::<u32>(),
        function_f: &(|u| gmp::Integer::from(u)),
        function_g: &(|u| native::Integer::from(u)),
        function_h: &(|u| num::BigUint::from(u)),
        function_i: &(|u| rugint::Integer::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        z_cons: &(|&u| u),
        w_cons: &(|&u| u),
        x_param: &(|&u| (32 - u.leading_zeros()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer::from(u32)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_from_u32(limit: usize, file_name: &str) {
    println!("benchmarking random Integer::from(u32)");
    benchmark_4(BenchmarkOptions4 {
        xs: random_x::<u32>(&EXAMPLE_SEED),
        function_f: &(|u| gmp::Integer::from(u)),
        function_g: &(|u| native::Integer::from(u)),
        function_h: &(|u| num::BigUint::from(u)),
        function_i: &(|u| rugint::Integer::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        z_cons: &(|&u| u),
        w_cons: &(|&u| u),
        x_param: &(|&u| (32 - u.leading_zeros()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer::from(u32)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
