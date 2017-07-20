use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_into_natural(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        let n_clone = n.clone();
        println!("into_natural({}) = {:?}", n_clone, n.into_natural());
    }
}

pub fn demo_random_integer_into_natural(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        let n_clone = n.clone();
        println!("into_natural({}) = {:?}", n_clone, n.into_natural());
    }
}

pub fn demo_exhaustive_integer_to_natural(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("to_natural(&{}) = {:?}", n, n.to_natural());
    }
}

pub fn demo_random_integer_to_natural(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_natural(&{}) = {:?}", n, n.to_natural());
    }
}

pub fn benchmark_exhaustive_integer_to_natural(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_natural()");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: gmp::Integer| n.into_natural()),
                    function_g: &(|n: native::Integer| n.into_natural()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.to\\\\_natural()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_to_natural(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.to_natural()");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_integers(&EXAMPLE_SEED, scale),
                    function_f: &(|n: gmp::Integer| n.into_natural()),
                    function_g: &(|n: native::Integer| n.into_natural()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.to\\\\_natural()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_integer_to_natural_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_natural() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: native::Integer| n.into_natural()),
                    function_g: &(|n: native::Integer| n.to_natural()),
                    x_cons: &(|x| gmp_integer_to_native(x)),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "into\\\\_natural (by value)",
                    g_name: "to\\\\_natural (by reference)",
                    title: "Integer.to\\\\_natural()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_to_natural_evaluation_strategy(limit: usize,
                                                               scale: u32,
                                                               file_name: &str) {
    println!("benchmarking random Integer.to_natural() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_integers(&EXAMPLE_SEED, scale),
                    function_f: &(|n: native::Integer| n.into_natural()),
                    function_g: &(|n: native::Integer| n.to_natural()),
                    x_cons: &(|x| gmp_integer_to_native(x)),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "into\\\\_natural (by value)",
                    g_name: "to\\\\_natural (by reference)",
                    title: "Integer.to\\\\_natural()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
