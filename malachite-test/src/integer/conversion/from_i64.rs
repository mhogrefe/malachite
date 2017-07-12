use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_i;

pub fn demo_exhaustive_integer_from_i64(limit: usize) {
    for i in exhaustive_i::<i64>().take(limit) {
        println!("from({}) = {}", i, gmp::Integer::from(i));
    }
}

pub fn demo_random_integer_from_i64(limit: usize) {
    for i in random_x::<i64>(&EXAMPLE_SEED).take(limit) {
        println!("from({}) = {}", i, gmp::Integer::from(i));
    }
}

pub fn benchmark_exhaustive_integer_from_i64(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from(i64)");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_i::<i64>(),
                    function_f: &(|i| gmp::Integer::from(i)),
                    function_g: &(|i| native::Integer::from(i)),
                    function_h: &(|i| num::BigInt::from(i)),
                    x_cons: &(|&i| i),
                    y_cons: &(|&i| i),
                    z_cons: &(|&i| i),
                    x_param: &(|&i| (64 - i.leading_zeros()) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Integer::from(i64)",
                    x_axis_label: "i.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_from_i64(limit: usize, file_name: &str) {
    println!("benchmarking random Integer::from(i64)");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_x::<i64>(&EXAMPLE_SEED),
                    function_f: &(|i| gmp::Integer::from(i)),
                    function_g: &(|i| native::Integer::from(i)),
                    function_h: &(|i| num::BigInt::from(i)),
                    x_cons: &(|&i| i),
                    y_cons: &(|&i| i),
                    z_cons: &(|&i| i),
                    x_param: &(|&i| (64 - i.leading_zeros()) as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Integer::from(i64)",
                    x_axis_label: "i.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
