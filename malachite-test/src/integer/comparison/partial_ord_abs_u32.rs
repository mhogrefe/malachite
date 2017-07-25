use common::gmp_integer_to_native;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::PartialOrdAbs as gmp_ord_abs;
use malachite_native::integer as native;
use malachite_native::traits::PartialOrdAbs as native_ord_abs;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::Ordering;

pub fn demo_exhaustive_integer_partial_cmp_abs_u32(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        match n.partial_cmp_abs(&u).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, u),
            Ordering::Equal => println!("|{}| = |{}|", n, u),
            Ordering::Greater => println!("|{}| > |{}|", n, u),
        }
    }
}

pub fn demo_random_integer_partial_cmp_abs_u32(limit: usize) {
    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<u32>(seed)))
                .take(limit) {
        match n.partial_cmp_abs(&u).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, u),
            Ordering::Equal => println!("|{}| = |{}|", n, u),
            Ordering::Greater => println!("|{}| > |{}|", n, u),
        }
    }
}

pub fn demo_exhaustive_u32_partial_cmp_abs_integer(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()).take(limit) {
        match gmp_ord_abs::partial_cmp_abs(&u, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", u, n),
            Ordering::Equal => println!("|{}| = |{}|", u, n),
            Ordering::Greater => println!("|{}| > |{}|", u, n),
        }
    }
}

pub fn demo_random_u32_partial_cmp_abs_integer(limit: usize) {
    for (u, n) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_x::<u32>(seed)),
                               &(|seed| random_integers(seed, 32)))
                .take(limit) {
        match gmp_ord_abs::partial_cmp_abs(&u, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", u, n),
            Ordering::Equal => println!("|{}| = |{}|", u, n),
            Ordering::Greater => println!("|{}| > |{}|", u, n),
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_cmp_abs_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.partial_cmp_abs(&u32)");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
                    function_f: &(|(n, u): (gmp::Integer, u32)| n.partial_cmp_abs(&u)),
                    function_g: &(|(n, u): (native::Integer, u32)| n.partial_cmp_abs(&u)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
                    x_param: &(|&(ref n, _)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&u32)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_partial_cmp_abs_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.partial_cmp_abs_(&u32)");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_integers(seed, scale)),
                                     &(|seed| random_x::<u32>(seed))),
                    function_f: &(|(n, u): (gmp::Integer, u32)| n.partial_cmp_abs(&u)),
                    function_g: &(|(n, u): (native::Integer, u32)| n.partial_cmp_abs(&u)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
                    x_param: &(|&(ref n, _)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&u32)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_u32_partial_cmp_abs_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive u32.partial_cmp_abs(&Integer)");
    benchmark_2(BenchmarkOptions2 {
                    xs: exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()),
                    function_f: &(|(u, n): (u32, gmp::Integer)| {
                                      gmp_ord_abs::partial_cmp_abs(&u, &n)
                                  }),
                    function_g: &(|(u, n): (u32, native::Integer)| {
                                      native_ord_abs::partial_cmp_abs(&u, &n)
                                  }),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
                    x_param: &(|&(_, ref n)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "u32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_u32_partial_cmp_abs_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random u32.partial_cmp_abs(&Integer)");
    benchmark_2(BenchmarkOptions2 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_x::<u32>(seed)),
                                     &(|seed| random_integers(seed, scale))),
                    function_f: &(|(u, n): (u32, gmp::Integer)| {
                                      gmp_ord_abs::partial_cmp_abs(&u, &n)
                                  }),
                    function_g: &(|(u, n): (u32, native::Integer)| {
                                      native_ord_abs::partial_cmp_abs(&u, &n)
                                  }),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
                    x_param: &(|&(_, ref n)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    title: "u32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
