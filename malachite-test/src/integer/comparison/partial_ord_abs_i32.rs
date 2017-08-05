use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::PartialOrdAbs as gmp_ord_abs;
use malachite_native::integer as native;
use malachite_native::traits::PartialOrdAbs as native_ord_abs;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::Ordering;

pub fn demo_exhaustive_integer_partial_cmp_abs_i32(limit: usize) {
    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

pub fn demo_random_integer_partial_cmp_abs_i32(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

pub fn demo_exhaustive_i32_partial_cmp_abs_integer(limit: usize) {
    for (i, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        match gmp_ord_abs::partial_cmp_abs(&i, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

pub fn demo_random_i32_partial_cmp_abs_integer(limit: usize) {
    for (i, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        match gmp_ord_abs::partial_cmp_abs(&i, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_cmp_abs_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.partial_cmp_abs(&i32)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i): (gmp::Integer, i32)| n.partial_cmp_abs(&i)),
        function_g: &(|(n, i): (native::Integer, i32)| n.partial_cmp_abs(&i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_partial_cmp_abs_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.partial_cmp_abs_(&i32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, i): (gmp::Integer, i32)| n.partial_cmp_abs(&i)),
        function_g: &(|(n, i): (native::Integer, i32)| n.partial_cmp_abs(&i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_partial_cmp_abs_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32.partial_cmp_abs(&Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(i, n): (i32, gmp::Integer)| gmp_ord_abs::partial_cmp_abs(&i, &n)),
        function_g: &(|(i, n): (i32, native::Integer)| native_ord_abs::partial_cmp_abs(&i, &n)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "i32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_partial_cmp_abs_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random i32.partial_cmp_abs(&Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(i, n): (i32, gmp::Integer)| gmp_ord_abs::partial_cmp_abs(&i, &n)),
        function_g: &(|(i, n): (i32, native::Integer)| native_ord_abs::partial_cmp_abs(&i, &n)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "i32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
