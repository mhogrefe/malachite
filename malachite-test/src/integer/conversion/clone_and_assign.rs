use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

pub fn demo_exhaustive_integer_clone(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        println!("clone({}) = {:?}", n, n.clone());
    }
}

pub fn demo_random_integer_clone(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        println!("clone({}) = {:?}", n, n.clone());
    }
}

pub fn demo_exhaustive_integer_clone_from(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_integer_clone_from(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_exhaustive_integer_assign(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_random_integer_assign(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_exhaustive_integer_assign_ref(limit: usize) {
    for (mut x, y) in exhaustive_pairs_from_single(exhaustive_integers()).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn demo_random_integer_assign_ref(limit: usize) {
    for (mut x, y) in random_pairs_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_exhaustive_integer_clone(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.clone()");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_integers(),
        function_f: &(|n: gmp::Integer| n.clone()),
        function_g: &(|n: native::Integer| n.clone()),
        function_h: &(|n: num::BigInt| n.clone()),
        function_i: &(|n: rugint::Integer| n.clone()),
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
        title: "Integer.clone()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_clone(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.clone()");
    benchmark_4(BenchmarkOptions4 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Integer| n.clone()),
        function_g: &(|n: native::Integer| n.clone()),
        function_h: &(|n: num::BigInt| n.clone()),
        function_i: &(|n: rugint::Integer| n.clone()),
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
        title: "Integer.clone()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_clone_from(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.clone_from(Integer)");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(mut x, y): (gmp::Integer, gmp::Integer)| x.clone_from(&y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.clone_from(&y)),
        function_h: &(|(mut x, y): (num::BigInt, num::BigInt)| x.clone_from(&y)),
        function_i: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.clone_from(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.clone\\\\_from(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_clone_from(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.clone_from(Integer)");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut x, y): (gmp::Integer, gmp::Integer)| x.clone_from(&y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.clone_from(&y)),
        function_h: &(|(mut x, y): (num::BigInt, num::BigInt)| x.clone_from(&y)),
        function_i: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.clone_from(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.clone\\\\_from(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(Integer)");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(mut x, y): (gmp::Integer, gmp::Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(y)),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.assign(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.assign(Integer)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut x, y): (gmp::Integer, gmp::Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(y)),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.assign(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_assign_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(Integer) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs_from_single(exhaustive_integers()),
        function_f: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(&y)),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.assign(Integer)",
        g_name: "Integer.assign(\\\\&Integer)",
        title: "Integer.assign(Integer) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_assign_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.assign(Integer) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x.assign(&y)),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.assign(Integer)",
        g_name: "Integer.assign(\\\\&Integer)",
        title: "Integer.assign(Integer) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
