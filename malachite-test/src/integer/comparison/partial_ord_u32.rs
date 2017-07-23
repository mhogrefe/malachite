use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3, BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::Ordering;

pub fn num_partial_cmp_u32(x: &num::BigInt, u: u32) -> Option<Ordering> {
    x.partial_cmp(&num::BigInt::from(u))
}

pub fn demo_exhaustive_integer_partial_cmp_u32(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

pub fn demo_random_integer_partial_cmp_u32(limit: usize) {
    for (n, u) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_integers(seed, 32)),
                               &(|seed| random_x::<u32>(seed)))
                .take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

pub fn demo_exhaustive_u32_partial_cmp_integer(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", u, n),
            Ordering::Equal => println!("{} = {}", u, n),
            Ordering::Greater => println!("{} > {}", u, n),
        }
    }
}

pub fn demo_random_u32_partial_cmp_integer(limit: usize) {
    for (u, n) in random_pairs(&EXAMPLE_SEED,
                               &(|seed| random_x::<u32>(seed)),
                               &(|seed| random_integers(seed, 32)))
                .take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", u, n),
            Ordering::Equal => println!("{} = {}", u, n),
            Ordering::Greater => println!("{} > {}", u, n),
        }
    }
}

pub fn benchmark_exhaustive_integer_partial_cmp_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.partial_cmp(&u32)");
    benchmark_4(BenchmarkOptions4 {
                    xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
                    function_f: &(|(n, u): (gmp::Integer, u32)| n.partial_cmp(&u)),
                    function_g: &(|(n, u): (native::Integer, u32)| n.partial_cmp(&u)),
                    function_h: &(|(n, u): (num::BigInt, u32)| num_partial_cmp_u32(&n, u)),
                    function_i: &(|(n, u): (rugint::Integer, u32)| n.partial_cmp(&u)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
                    z_cons: &(|&(ref n, u)| (gmp_integer_to_num_bigint(n), u)),
                    w_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
                    x_param: &(|&(ref n, _)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Integer.partial\\\\_cmp(\\\\&u32)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_integer_partial_cmp_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.partial_cmp(&u32)");
    benchmark_4(BenchmarkOptions4 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_integers(seed, scale)),
                                     &(|seed| random_x::<u32>(seed))),
                    function_f: &(|(n, u): (gmp::Integer, u32)| n.partial_cmp(&u)),
                    function_g: &(|(n, u): (native::Integer, u32)| n.partial_cmp(&u)),
                    function_h: &(|(n, u): (num::BigInt, u32)| num_partial_cmp_u32(&n, u)),
                    function_i: &(|(n, u): (rugint::Integer, u32)| n.partial_cmp(&u)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
                    z_cons: &(|&(ref n, u)| (gmp_integer_to_num_bigint(n), u)),
                    w_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
                    x_param: &(|&(ref n, _)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    i_name: "rugint",
                    title: "Integer.partial\\\\_cmp(\\\\&u32)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_exhaustive_u32_partial_cmp_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive u32.partial_cmp(&Integer)");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()),
                    function_f: &(|(u, n): (u32, gmp::Integer)| u.partial_cmp(&n)),
                    function_g: &(|(u, n): (u32, native::Integer)| u.partial_cmp(&n)),
                    function_h: &(|(u, n): (u32, rugint::Integer)| u.partial_cmp(&n)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
                    z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
                    x_param: &(|&(_, ref n)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "u32.partial\\\\_cmp(\\\\&Integer)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_u32_partial_cmp_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random u32.partial_cmp(&Integer)");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_pairs(&EXAMPLE_SEED,
                                     &(|seed| random_x::<u32>(seed)),
                                     &(|seed| random_integers(seed, scale))),
                    function_f: &(|(u, n): (u32, gmp::Integer)| u.partial_cmp(&n)),
                    function_g: &(|(u, n): (u32, native::Integer)| u.partial_cmp(&n)),
                    function_h: &(|(u, n): (u32, rugint::Integer)| u.partial_cmp(&n)),
                    x_cons: &(|p| p.clone()),
                    y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
                    z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
                    x_param: &(|&(_, ref n)| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "rugint",
                    title: "u32.partial\\\\_cmp(\\\\&Integer)",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
