use common::{gmp_integer_to_native, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_to_u32(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_random_integer_to_u32(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_exhaustive_integer_to_u32_wrapping(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32_wrapping());
    }
}

pub fn demo_random_integer_to_u32_wrapping(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32_wrapping());
    }
}

pub fn benchmark_exhaustive_integer_to_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_u32()");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: gmp::Integer| n.to_u32()),
                    function_g: &(|n: native::Integer| n.to_u32()),
                    function_h: &(|n: rugint::Integer| n.to_u32()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.to\\\\_u32()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_to_u32(limit: usize, file_name: &str) {
    println!("benchmarking random Integer.to_u32()");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_integers(&EXAMPLE_SEED, 32),
                    function_f: &(|n: gmp::Integer| n.to_u32()),
                    function_g: &(|n: native::Integer| n.to_u32()),
                    function_h: &(|n: rugint::Integer| n.to_u32()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.to\\\\_u32()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_integer_to_u32_wrapping(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.to_u32_wrapping()");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: gmp::Integer| n.to_u32_wrapping()),
                    function_g: &(|n: native::Integer| n.to_u32_wrapping()),
                    function_h: &(|n: rugint::Integer| n.to_u32_wrapping()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.to\\\\_u32\\\\_wrapping()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_to_u32_wrapping(limit: usize, file_name: &str) {
    println!("benchmarking random Integer.to_u32_wrapping()");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_integers(&EXAMPLE_SEED, 32),
                    function_f: &(|n: gmp::Integer| n.to_u32_wrapping()),
                    function_g: &(|n: native::Integer| n.to_u32_wrapping()),
                    function_h: &(|n: rugint::Integer| n.to_u32_wrapping()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "Integer.to\\\\_u32\\\\_wrapping()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
