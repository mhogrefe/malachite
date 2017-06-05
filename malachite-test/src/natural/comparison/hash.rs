use common::{gmp_natural_to_native, gmp_natural_to_num_biguint};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn native_hash(n: &native::Natural) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn gmp_hash(n: &gmp::Natural) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

fn num_hash(n: &num::BigUint) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

pub fn demo_exhaustive_natural_hash(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        let hash = gmp_hash(&n);
        println!("hash({}) = {}", n, hash);
    }
}

pub fn demo_random_natural_hash(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        let hash = gmp_hash(&n);
        println!("hash({}) = {}", n, hash);
    }
}

pub fn benchmark_exhaustive_natural_hash(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural hash");
    benchmark_3(BenchmarkOptions3 {
                    xs: exhaustive_naturals(),
                    function_f: &(|n| gmp_hash(&n)),
                    function_g: &(|n: native::Natural| native_hash(&n)),
                    function_h: &(|n: num::BigUint| num_hash(&n)),
                    x_to_y: &(|x| gmp_natural_to_native(x)),
                    x_to_z: &(|x| gmp_natural_to_num_biguint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural hash",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}

pub fn benchmark_random_natural_hash(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural hash");
    benchmark_3(BenchmarkOptions3 {
                    xs: random_naturals(&EXAMPLE_SEED, scale),
                    function_f: &(|n| gmp_hash(&n)),
                    function_g: &(|n: native::Natural| native_hash(&n)),
                    function_h: &(|n: num::BigUint| num_hash(&n)),
                    x_to_y: &(|x| gmp_natural_to_native(x)),
                    x_to_z: &(|x| gmp_natural_to_num_biguint(x)),
                    x_param: &(|n| n.significant_bits() as usize),
                    limit: limit,
                    f_name: "malachite-gmp",
                    g_name: "malachite-native",
                    h_name: "num",
                    title: "Natural hash",
                    x_axis_label: "n.significant\\\\_bits()",
                    y_axis_label: "time (ns)",
                    file_name: &format!("benchmarks/{}", file_name),
                });
}
