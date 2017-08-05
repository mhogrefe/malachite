use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_trailing_zeros(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

pub fn demo_random_integer_trailing_zeros(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

pub fn benchmark_exhaustive_integer_trailing_zeros(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.trailing_zeros()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.trailing_zeros()),
        function_g: &(|n: native::Integer| n.trailing_zeros()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.trailing\\\\_zeros()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_trailing_zeros(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.trailing_zeros()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.trailing_zeros()),
        function_g: &(|n: native::Integer| n.trailing_zeros()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.trailing\\\\_zeros()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
