use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_i;

pub fn demo_exhaustive_integer_from_i32(limit: usize) {
    for i in exhaustive_i::<i32>().take(limit) {
        println!("from({}) = {}", i, gmp::Integer::from(i));
    }
}

pub fn demo_random_integer_from_i32(limit: usize) {
    for i in random_x::<i32>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", i, gmp::Integer::from(i));
    }
}

pub fn benchmark_exhaustive_integer_from_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from(i32)");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_i::<i32>(),
        function_f: &(|i| gmp::Integer::from(i)),
        function_g: &(|i| native::Integer::from(i)),
        function_h: &(|i| num::BigInt::from(i)),
        function_i: &(|i| rugint::Integer::from(i)),
        x_cons: &(|&i| i),
        y_cons: &(|&i| i),
        z_cons: &(|&i| i),
        w_cons: &(|&i| i),
        x_param: &(|&i| (32 - i.leading_zeros()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer::from(i32)",
        x_axis_label: "i.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_from_i32(limit: usize, file_name: &str) {
    println!("benchmarking random Integer::from(i32)");
    benchmark_4(BenchmarkOptions4 {
        xs: random_x::<i32>(&EXAMPLE_SEED),
        function_f: &(|i| gmp::Integer::from(i)),
        function_g: &(|i| native::Integer::from(i)),
        function_h: &(|i| num::BigInt::from(i)),
        function_i: &(|i| rugint::Integer::from(i)),
        x_cons: &(|&i| i),
        y_cons: &(|&i| i),
        z_cons: &(|&i| i),
        w_cons: &(|&i| i),
        x_param: &(|&i| (32 - i.leading_zeros()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer::from(i32)",
        x_axis_label: "i.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
