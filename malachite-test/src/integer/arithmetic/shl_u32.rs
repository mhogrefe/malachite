use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, BenchmarkOptions4, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn demo_exhaustive_integer_shl_assign_u32(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_shl_assign_u32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_shl_u32(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

pub fn demo_random_integer_shl_u32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

pub fn demo_exhaustive_integer_shl_u32_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

pub fn demo_random_integer_shl_u32_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

pub fn benchmark_exhaustive_integer_shl_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer <<= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u)| n <<= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n <<= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n <<= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer <<= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shl_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer <<= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u)| n <<= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n <<= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n <<= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer <<= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shl_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer << u32");
    benchmark_4(BenchmarkOptions4 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n << u),
        function_g: &(|(n, u): (native::Integer, u32)| n << u),
        function_h: &(|(n, u): (num::BigInt, u32)| n << u as usize),
        function_i: &(|(n, u): (rugint::Integer, u32)| n << u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_num_bigint(n), index)),
        w_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shl_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer << u32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| n << u),
        function_g: &(|(n, u): (native::Integer, u32)| n << u),
        function_h: &(|(n, u): (num::BigInt, u32)| n << u as usize),
        function_i: &(|(n, u): (rugint::Integer, u32)| n << u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_num_bigint(n), index)),
        w_cons: &(|&(ref n, index)| (gmp_integer_to_rugint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_shl_u32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive &Integer << u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| &n << u),
        function_g: &(|(n, u): (native::Integer, u32)| &n << u),
        function_h: &(|(n, u): (num::BigInt, u32)| &n << u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_num_bigint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "\\\\&Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_shl_u32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random &Integer << u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| &n << u),
        function_g: &(|(n, u): (native::Integer, u32)| &n << u),
        function_h: &(|(n, u): (num::BigInt, u32)| &n << u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_integer_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_integer_to_num_bigint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "\\\\&Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
