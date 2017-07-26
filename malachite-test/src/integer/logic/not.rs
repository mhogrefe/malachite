use common::{gmp_integer_to_native, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::NotAssign as gmp_not_assign;
use malachite_native::integer as native;
use malachite_native::traits::NotAssign as native_not_assign;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2, BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_not_assign(limit: usize) {
    for mut n in exhaustive_integers().take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {}; n.not_assign(); n = {}", n_old, n);
    }
}

pub fn demo_random_integer_not_assign(limit: usize) {
    for mut n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {}; n.not_assign(); n = {}", n_old, n);
    }
}

pub fn demo_exhaustive_integer_not(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_random_integer_not(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_exhaustive_integer_not_ref(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn demo_random_integer_not_ref(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn benchmark_exhaustive_integer_not_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.not_assign()");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_integers(),
                    function_f: &(|mut n: gmp::Integer| n.not_assign()),
                    function_g: &(|mut n: native::Integer| n.not_assign()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.not_assign()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_not_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.not_assign()");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_integers(&EXAMPLE_SEED, scale),
                    function_f: &(|mut n: gmp::Integer| n.not_assign()),
                    function_g: &(|mut n: native::Integer| n.not_assign()),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.not_assign()",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_integer_not(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive !Integer");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: gmp::Integer| !n),
                    function_g: &(|n: native::Integer| !n),
                    function_h: &(|n: rugint::Integer| !n),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "-Integer",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_not(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random !Integer");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_integers(&EXAMPLE_SEED, scale),
                    function_f: &(|n: gmp::Integer| !n),
                    function_g: &(|n: native::Integer| !n),
                    function_h: &(|n: rugint::Integer| !n),
                    x_cons: &(|x| x.clone()),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    z_cons: &(|x| gmp_integer_to_rugint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "-Integer",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_integer_not_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive !Integer evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_integers(),
                    function_f: &(|n: native::Integer| !n),
                    function_g: &(|n: native::Integer| !&n),
                    x_cons: &(|x| gmp_integer_to_native(x)),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "-Integer",
                    g_name: "-\\\\&Integer",
                    title: "-Integer evaluation strategy",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_not_evaluation_strategy(limit: usize,
                                                        scale: u32,
                                                        file_name: &str) {
    println!("benchmarking random !Integer evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_integers(&EXAMPLE_SEED, scale),
                    function_f: &(|n: native::Integer| !n),
                    function_g: &(|n: native::Integer| !&n),
                    x_cons: &(|x| gmp_integer_to_native(x)),
                    y_cons: &(|x| gmp_integer_to_native(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "-Integer",
                    g_name: "-\\\\&Integer",
                    title: "-Integer evaluation strategy",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
