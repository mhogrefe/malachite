use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_sub_u32(mut x: num::BigInt, u: u32) -> num::BigInt {
    x = x - num::BigInt::from(u);
    x
}

pub fn demo_exhaustive_integer_sub_assign_u32(limit: usize) {
    for (mut n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_sub_assign_u32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_sub_u32(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, u, n - u);
    }
}

pub fn demo_random_integer_sub_u32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, u, n - u);
    }
}

pub fn demo_exhaustive_integer_sub_u32_ref(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()).take(limit) {
        println!("&{} - {} = {}", n, u, &n - u);
    }
}

pub fn demo_random_integer_sub_u32_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<u32>(seed)),
    ).take(limit)
    {
        println!("&{} - {} = {}", n, u, &n - u);
    }
}

pub fn demo_exhaustive_u32_sub_integer(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", u, n_old, u - n);
    }
}

pub fn demo_random_u32_sub_integer(limit: usize) {
    for (u, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<u32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - {} = {}", u, n_old, u - n);
    }
}

pub fn demo_exhaustive_u32_sub_integer_ref(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} - &{} = {}", u, n_old, u - &n);
    }
}

pub fn demo_random_u32_sub_integer_ref(limit: usize) {
    for (u, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<u32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - &{} = {}", u, n_old, u - &n);
    }
}

pub fn benchmark_exhaustive_integer_sub_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer -= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u)| n -= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n -= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n -= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "integer -= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random integer -= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<u32>(seed)),
        ),
        function_f: &(|(mut n, u)| n -= u),
        function_g: &(|(mut n, u): (native::Integer, u32)| n -= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n -= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "integer -= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer - u32");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n - u),
        function_g: &(|(n, u): (native::Integer, u32)| n - u),
        function_h: &(|(n, u): (num::BigInt, u32)| num_sub_u32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n - u),
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
        title: "integer - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random integer - u32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<u32>(seed)),
        ),
        function_f: &(|(n, u)| n - u),
        function_g: &(|(n, u): (native::Integer, u32)| n - u),
        function_h: &(|(n, u): (num::BigInt, u32)| num_sub_u32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n - u),
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
        title: "integer - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_u32_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer - u32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n - u),
        function_g: &(|(n, u)| &n - u),
        x_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "integer - u32",
        g_name: "\\\\&integer - u32",
        title: "integer - u32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_u32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random integer - u32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<u32>(seed)),
        ),
        function_f: &(|(n, u)| n - u),
        function_g: &(|(n, u)| &n - u),
        x_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit: limit,
        f_name: "integer - u32",
        g_name: "\\\\&integer - u32",
        title: "integer - u32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_u32_sub_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive u32 - integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()),
        function_f: &(|(u, n)| u - n),
        function_g: &(|(u, n): (u32, native::Integer)| u - n),
        function_h: &(|(u, n): (u32, rugint::Integer)| u - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 - integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_u32_sub_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random u32 - integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<u32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(u, n)| u - n),
        function_g: &(|(u, n): (u32, native::Integer)| u - n),
        function_h: &(|(u, n): (u32, rugint::Integer)| u - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 - integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_u32_sub_integer_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive u32 - integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_u::<u32>(), exhaustive_integers()),
        function_f: &(|(u, n)| u - n),
        function_g: &(|(u, n)| u - &n),
        x_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "u32 - integer",
        g_name: "u32 - \\\\&integer",
        title: "u32 - integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_u32_sub_integer_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive u32 - integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<u32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(u, n)| u - n),
        function_g: &(|(u, n)| u - &n),
        x_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit: limit,
        f_name: "u32 - integer",
        g_name: "u32 - \\\\&integer",
        title: "u32 - integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
