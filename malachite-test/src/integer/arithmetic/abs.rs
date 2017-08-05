use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::AbsAssign as gmp_abs_assign;
use malachite_native::integer as native;
use malachite_native::traits::AbsAssign as native_abs_assign;
use num::{self, Signed};
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions4, benchmark_2, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

pub fn demo_exhaustive_integer_abs_assign(limit: usize) {
    for mut n in exhaustive_integers().take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

pub fn demo_random_integer_abs_assign(limit: usize) {
    for mut n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

pub fn demo_exhaustive_integer_abs(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("abs({}) = {}", n.clone(), n.abs());
    }
}

pub fn demo_random_integer_abs(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("abs({}) = {}", n.clone(), n.abs());
    }
}

pub fn demo_exhaustive_integer_abs_ref(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("abs_ref(&{}) = {}", n, n.abs_ref());
    }
}

pub fn demo_random_integer_abs_ref(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("abs_ref(&{}) = {}", n, n.abs_ref());
    }
}

pub fn demo_exhaustive_integer_natural_abs(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("natural_abs({}) = {}", n.clone(), n.natural_abs());
    }
}

pub fn demo_random_integer_natural_abs(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("natural_abs({}) = {}", n.clone(), n.natural_abs());
    }
}

pub fn demo_exhaustive_integer_natural_abs_ref(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("natural_abs_ref(&{}) = {}", n, n.natural_abs_ref());
    }
}

pub fn demo_random_integer_natural_abs_ref(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("natural_abs_ref(&{}) = {}", n, n.natural_abs_ref());
    }
}

pub fn benchmark_exhaustive_integer_abs_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.abs_assign()");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs()),
        function_h: &(|n: num::BigInt| n.abs()),
        function_i: &(|mut n: rugint::Integer| n.abs().sign()),
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
        title: "Integer.abs\\\\_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_abs_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.abs_assign()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|mut n: gmp::Integer| n.abs_assign()),
        function_g: &(|mut n: native::Integer| n.abs_assign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.abs\\\\_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_abs(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.abs()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_abs(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.abs()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_abs_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.abs() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: native::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "Integer.abs()",
        g_name: "Integer.abs_ref()",
        title: "Integer.abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_abs_evaluation_strategy(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.abs() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: native::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "Integer.abs()",
        g_name: "Integer.abs_ref()",
        title: "Integer.abs()",
        x_axis_label: "n.significant\\\\_bits() evaluation strategy",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_natural_abs(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.natural_abs()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.natural\\\\_abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_natural_abs(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.natural_abs()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.natural\\\\_abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_natural_abs_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.natural_abs() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_integers(),
        function_f: &(|n: native::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "Integer.natural_abs()",
        g_name: "Integer.natural_abs_ref()",
        title: "Integer.natural\\\\_abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_natural_abs_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.natural_abs() evaluation_strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: native::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "Integer.natural_abs()",
        g_name: "Integer.natural_abs_ref()",
        title: "Integer.natural\\\\_abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
