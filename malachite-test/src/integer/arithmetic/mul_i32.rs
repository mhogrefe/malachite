use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint};
use malachite_native::integer as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_mul_i32(mut x: num::BigInt, u: i32) -> num::BigInt {
    x = x * num::BigInt::from(u);
    x
}

pub fn demo_exhaustive_integer_mul_assign_i32(limit: usize) {
    for (mut n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_random_integer_mul_assign_i32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_integer_mul_i32(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

pub fn demo_random_integer_mul_i32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

pub fn demo_exhaustive_integer_mul_i32_ref(limit: usize) {
    for (n, u) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

pub fn demo_random_integer_mul_i32_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

pub fn demo_exhaustive_i32_mul_integer(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

pub fn demo_random_i32_mul_integer(limit: usize) {
    for (u, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

pub fn demo_exhaustive_i32_mul_integer_ref(limit: usize) {
    for (u, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

pub fn demo_random_i32_mul_integer_ref(limit: usize) {
    for (u, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

pub fn benchmark_exhaustive_integer_mul_assign_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer *= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(mut n, u)| n *= u),
        function_g: &(|(mut n, u): (native::Integer, i32)| n *= u),
        function_h: &(|(mut n, u): (rugint::Integer, i32)| n *= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer *= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_assign_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer *= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(mut n, u)| n *= u),
        function_g: &(|(mut n, u): (native::Integer, i32)| n *= u),
        function_h: &(|(mut n, u): (rugint::Integer, i32)| n *= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer *= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mul_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer * i32");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u): (native::Integer, i32)| n * u),
        function_h: &(|(n, u): (num::BigInt, i32)| num_mul_i32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, i32)| n * u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_num_bigint(n), u)),
        w_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer * i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer * i32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u): (native::Integer, i32)| n * u),
        function_h: &(|(n, u): (num::BigInt, i32)| num_mul_i32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, i32)| n * u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_integer_to_num_bigint(n), u)),
        w_cons: &(|&(ref n, u)| (gmp_integer_to_rugint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer * i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_mul_i32_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer * i32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u)| &n * u),
        x_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer * i32",
        g_name: "\\\\&Integer * i32",
        title: "Integer * i32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_mul_i32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer * i32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u)| &n * u),
        x_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        y_cons: &(|&(ref n, u)| (gmp_integer_to_native(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer * i32",
        g_name: "\\\\&Integer * i32",
        title: "Integer * i32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_mul_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32 * Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n): (i32, native::Integer)| u * n),
        function_h: &(|(u, n): (i32, rugint::Integer)| u * n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 * Integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_mul_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random i32 * Integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n): (i32, native::Integer)| u * n),
        function_h: &(|(u, n): (i32, rugint::Integer)| u * n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 * Integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_mul_integer_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32 * Integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n)| u * &n),
        x_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "i32 * Integer",
        g_name: "i32 * \\\\&Integer",
        title: "i32 * Integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_mul_integer_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random i32 * Integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n)| u * &n),
        x_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        y_cons: &(|&(u, ref n)| (u, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "i32 * Integer",
        g_name: "i32 * \\\\&Integer",
        title: "i32 * Integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
