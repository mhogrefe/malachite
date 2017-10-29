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

pub fn num_sub_i32(mut x: num::BigInt, i: i32) -> num::BigInt {
    x = x - num::BigInt::from(i);
    x
}

pub fn demo_exhaustive_integer_sub_assign_i32(limit: usize) {
    for (mut n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        n -= i;
        println!("x := {}; x -= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_random_integer_sub_assign_i32(limit: usize) {
    for (mut n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        n -= i;
        println!("x := {}; x -= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_exhaustive_integer_sub_i32(limit: usize) {
    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, i, n - i);
    }
}

pub fn demo_random_integer_sub_i32(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - {} = {}", n_old, i, n - i);
    }
}

pub fn demo_exhaustive_integer_sub_i32_ref(limit: usize) {
    for (n, i) in exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()).take(limit) {
        println!("&{} - {} = {}", n, i, &n - i);
    }
}

pub fn demo_random_integer_sub_i32_ref(limit: usize) {
    for (n, i) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x::<i32>(seed)),
    ).take(limit)
    {
        println!("&{} - {} = {}", n, i, &n - i);
    }
}

pub fn demo_exhaustive_i32_sub_integer(limit: usize) {
    for (i, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {}", i, n_old, i - n);
    }
}

pub fn demo_random_i32_sub_integer(limit: usize) {
    for (i, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - {} = {}", i, n_old, i - n);
    }
}

pub fn demo_exhaustive_i32_sub_integer_ref(limit: usize) {
    for (i, n) in exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()).take(limit) {
        let n_old = n.clone();
        println!("{} - &{} = {}", i, n_old, i - &n);
    }
}

pub fn demo_random_i32_sub_integer_ref(limit: usize) {
    for (i, n) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x::<i32>(seed)),
        &(|seed| random_integers(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} - &{} = {}", i, n_old, i - &n);
    }
}

pub fn benchmark_exhaustive_integer_sub_assign_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer -= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(mut n, i)| n -= i),
        function_g: &(|(mut n, i): (native::Integer, i32)| n -= i),
        function_h: &(|(mut n, i): (rugint::Integer, i32)| n -= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "integer -= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_assign_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random integer -= i32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(mut n, i)| n -= i),
        function_g: &(|(mut n, i): (native::Integer, i32)| n -= i),
        function_h: &(|(mut n, i): (rugint::Integer, i32)| n -= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "integer -= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_i32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer - i32");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i)| n - i),
        function_g: &(|(n, i): (native::Integer, i32)| n - i),
        function_h: &(|(n, i): (num::BigInt, i32)| num_sub_i32(n, i)),
        function_i: &(|(n, i): (rugint::Integer, i32)| n - i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        w_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "integer - i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_i32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random integer - i32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, i)| n - i),
        function_g: &(|(n, i): (native::Integer, i32)| n - i),
        function_h: &(|(n, i): (num::BigInt, i32)| num_sub_i32(n, i)),
        function_i: &(|(n, i): (rugint::Integer, i32)| n - i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        w_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "integer - i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_i32_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive integer - i32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_integers(), exhaustive_i::<i32>()),
        function_f: &(|(n, i)| n - i),
        function_g: &(|(n, i)| &n - i),
        x_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "integer - i32",
        g_name: "\\\\&integer - i32",
        title: "integer - i32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_i32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random integer - i32 evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x::<i32>(seed)),
        ),
        function_f: &(|(n, i)| n - i),
        function_g: &(|(n, i)| &n - i),
        x_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "integer - i32",
        g_name: "\\\\&integer - i32",
        title: "integer - i32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_sub_integer(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32 - integer");
    benchmark_3(BenchmarkOptions3 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(i, n)| i - n),
        function_g: &(|(i, n): (i32, native::Integer)| i - n),
        function_h: &(|(i, n): (i32, rugint::Integer)| i - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        z_cons: &(|&(i, ref n)| (i, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 - integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_sub_integer(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random i32 - integer");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(i, n)| i - n),
        function_g: &(|(i, n): (i32, native::Integer)| i - n),
        function_h: &(|(i, n): (i32, rugint::Integer)| i - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        z_cons: &(|&(i, ref n)| (i, gmp_integer_to_rugint(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "i32 - integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_i32_sub_integer_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive i32 - integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(exhaustive_i::<i32>(), exhaustive_integers()),
        function_f: &(|(i, n)| i - n),
        function_g: &(|(i, n)| i - &n),
        x_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "i32 - integer",
        g_name: "i32 - \\\\&integer",
        title: "i32 - integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_i32_sub_integer_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive i32 - integer evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_x::<i32>(seed)),
            &(|seed| random_integers(seed, scale)),
        ),
        function_f: &(|(i, n)| i - n),
        function_g: &(|(i, n)| i - &n),
        x_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        y_cons: &(|&(i, ref n)| (i, gmp_integer_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "i32 - integer",
        g_name: "i32 - \\\\&integer",
        title: "i32 - integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
