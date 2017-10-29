use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_to_u64(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64());
    }
}

pub fn demo_random_integer_to_u64(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 64).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64());
    }
}

pub fn demo_exhaustive_integer_to_u64_wrapping(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64_wrapping());
    }
}

pub fn demo_random_integer_to_u64_wrapping(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 64).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64_wrapping());
    }
}

pub fn benchmark_exhaustive_integer_to_u64(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_u64()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.to_u64()),
        function_g: &(|n: native::Integer| n.to_u64()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.to\\\\_u64()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_to_u64(limit: usize, file_name: &str) {
    println!("benchmarking random Integer.to_u64()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, 32),
        function_f: &(|n: gmp::Integer| n.to_u64()),
        function_g: &(|n: native::Integer| n.to_u64()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.to\\\\_u64()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_to_u64_wrapping(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_u64_wrapping()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.to_u64_wrapping()),
        function_g: &(|n: native::Integer| n.to_u64_wrapping()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.to\\\\_u64\\\\_wrapping()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_to_u64_wrapping(limit: usize, file_name: &str) {
    println!("benchmarking random Integer.to_u64_wrapping()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, 32),
        function_f: &(|n: gmp::Integer| n.to_u64_wrapping()),
        function_g: &(|n: native::Integer| n.to_u64_wrapping()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.to\\\\_u64\\\\_wrapping()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
