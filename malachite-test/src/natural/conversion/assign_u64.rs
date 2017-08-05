use common::{gmp_natural_to_native, gmp_natural_to_num_biguint};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_u64(x: &mut num::BigUint, u: u64) {
    *x = num::BigUint::from(u);
}

pub fn demo_exhaustive_natural_assign_u64(limit: usize) {
    for (mut n, u) in exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u64>()).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_random_natural_assign_u64(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| random_x::<u64>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_exhaustive_natural_assign_u64(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.assign(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_naturals(), exhaustive_u::<u64>()),
        function_f: &(|(mut n, u): (gmp::Natural, u64)| n.assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u64)| n.assign(u)),
        function_h: &(|(mut n, u): (num::BigUint, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_num_biguint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Natural.assign(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_assign_u64(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.assign(u64)");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| random_x::<u64>(seed)),
        ),
        function_f: &(|(mut n, u): (gmp::Natural, u64)| n.assign(u)),
        function_g: &(|(mut n, u): (native::Natural, u64)| n.assign(u)),
        function_h: &(|(mut n, u): (num::BigUint, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_num_biguint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Natural.assign(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
