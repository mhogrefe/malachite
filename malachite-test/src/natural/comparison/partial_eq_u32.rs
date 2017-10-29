use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, BenchmarkOptions4, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_partial_eq_u32(x: &num::BigUint, u: u32) -> bool {
    *x == num::BigUint::from(u)
}

pub fn demo_exhaustive_natural_partial_eq_u32(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

pub fn demo_random_natural_partial_eq_u32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        if n == u {
            println!("{} = {}", n, u);
        } else {
            println!("{} ≠ {}", n, u);
        }
    }
}

pub fn demo_exhaustive_u32_partial_eq_natural(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_naturals()).take(limit) {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

pub fn demo_random_u32_partial_eq_natural(limit: usize) {
    for (u, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<u32>(seed)),
        &(|seed| random_naturals(seed, 32)),
    ).take(limit)
    {
        if u == n {
            println!("{} = {}", u, n);
        } else {
            println!("{} ≠ {}", u, n);
        }
    }
}

pub fn benchmark_exhaustive_natural_partial_eq_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural == u32");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n == u),
        function_g: &(|(n, u): (native::Natural, u32)| n == u),
        function_h: &(|(n, u): (num::BigUint, u32)| num_partial_eq_u32(&n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n == u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_num_biguint(n), u)),
        w_cons: &(|&(ref n, u)| (gmp_natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural == u32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_partial_eq_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural == u32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x::<u32>(seed)),
        ),
        function_f: &(|(n, u)| n == u),
        function_g: &(|(n, u): (native::Natural, u32)| n == u),
        function_h: &(|(n, u): (num::BigUint, u32)| num_partial_eq_u32(&n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n == u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_num_biguint(n), u)),
        w_cons: &(|&(ref n, u)| (gmp_natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural == u32",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_u32_partial_eq_natural(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive u32 == Natural");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_naturals()),
        function_f: &(|(u, n)| u == n),
        function_g: &(|(u, n): (u32, native::Natural)| u == n),
        function_h: &(|(u, n): (u32, rugint::Integer)| u == n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_natural_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 == Natural",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_u32_partial_eq_natural(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random u32 == Natural");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<u32>(seed)),
            &(|seed| random_naturals(seed, scale)),
        ),
        function_f: &(|(u, n)| u == n),
        function_g: &(|(u, n): (u32, native::Natural)| n == u),
        function_h: &(|(u, n): (u32, rugint::Integer)| n == u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_natural_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 == Natural",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
