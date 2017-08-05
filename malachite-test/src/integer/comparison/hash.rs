use common::{gmp_integer_to_native, gmp_integer_to_num_bigint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn native_hash(n: &native::Integer) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn gmp_hash(n: &gmp::Integer) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn num_hash(n: &num::BigInt) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

pub fn demo_exhaustive_integer_hash(limit: usize) {
    for n in exhaustive_integers().take(limit) {
        let hash = gmp_hash(&n);
        println!("hash({}) = {}", n, hash);
    }
}

pub fn demo_random_integer_hash(limit: usize) {
    for n in random_integers(&EXAMPLE_SEED, 32).take(limit) {
        let hash = gmp_hash(&n);
        println!("hash({}) = {}", n, hash);
    }
}

pub fn benchmark_exhaustive_integer_hash(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer hash");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_integers(),
        function_f: &(|n| gmp_hash(&n)),
        function_g: &(|n: native::Integer| native_hash(&n)),
        function_h: &(|n: num::BigInt| num_hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Integer hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_hash(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer hash");
    benchmark_3(BenchmarkOptions3 {
        xs: random_integers(&EXAMPLE_SEED, scale),
        function_f: &(|n| gmp_hash(&n)),
        function_g: &(|n: native::Integer| native_hash(&n)),
        function_h: &(|n: num::BigInt| num_hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Integer hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
