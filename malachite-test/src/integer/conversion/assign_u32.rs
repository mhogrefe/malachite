use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use malachite_native::traits::Assign as native_assign;
use malachite_gmp::traits::Assign as gmp_assign;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_u32(x: &mut num::BigInt, u: u32) {
    *x = num::BigInt::from(u);
}

pub fn demo_exhaustive_integer_assign_u32(limit: usize) {
    for (mut n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_assign_u32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_exhaustive_integer_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.assign(u32)");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.assign(u)),
        function_h: &(|(mut n, u): (num::BigInt, u32)| num_assign_u32(&mut n, u)),
        function_i: &(|(mut n, u): (rugint::Integer, u32)| n.assign(u)),
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
        title: "Integer.assign(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.assign(u32)");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<u32>(seed)),
        ),
        function_f: &(|(mut n, u): (gmp::Integer, u32)| n.assign(u)),
        function_g: &(|(mut n, u): (native::Integer, u32)| n.assign(u)),
        function_h: &(|(mut n, u): (num::BigInt, u32)| num_assign_u32(&mut n, u)),
        function_i: &(|(mut n, u): (rugint::Integer, u32)| n.assign(u)),
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
        title: "Integer.assign(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
