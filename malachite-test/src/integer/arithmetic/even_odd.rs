use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_is_even(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        if n.is_even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

pub fn demo_random_integer_is_even(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        if n.is_even() {
            println!("{} is even", n);
        } else {
            println!("{} is not even", n);
        }
    }
}

pub fn demo_exhaustive_integer_is_odd(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        if n.is_odd() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

pub fn demo_random_integer_is_odd(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        if n.is_odd() {
            println!("{} is odd", n);
        } else {
            println!("{} is not odd", n);
        }
    }
}

pub fn benchmark_exhaustive_integer_is_even(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.is_even()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.is_even()),
        function_g: &(|n: native::Integer| n.is_even()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_even()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_is_even(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.is_even()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.is_even()),
        function_g: &(|n: native::Integer| n.is_even()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_even()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_is_odd(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.is_odd()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.is_odd()),
        function_g: &(|n: native::Integer| n.is_odd()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_odd()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_is_odd(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.is_odd()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.is_odd()),
        function_g: &(|n: native::Integer| n.is_odd()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.is\\\\_odd()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
