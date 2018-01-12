use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_nz::integer::Integer;
use num::BigInt;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_mul_i32(mut x: BigInt, i: i32) -> BigInt {
    x = x * BigInt::from(i);
    x
}

type It1 = Iterator<Item = (Integer, i32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_i()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (i32, Integer)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs(exhaustive_i(), exhaustive_integers()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_integers(seed, scale)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_integer_mul_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        n *= i;
        println!("x := {}; x *= {}; x = {}", n_old, i, n);
    }
}

pub fn demo_integer_mul_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, i, n * i);
    }
}

pub fn demo_integer_mul_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in select_inputs_1(gm).take(limit) {
        println!("&{} * {} = {}", n, i, &n * i);
    }
}

pub fn demo_i32_mul_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", i, n_old, i * n);
    }
}

pub fn demo_i32_mul_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", i, n_old, i * &n);
    }
}

pub fn benchmark_integer_mul_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer *= i32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut n, i)| n *= i),
        function_g: &(|(mut n, i): (rugint::Integer, i32)| n *= i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_rugint_integer(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer *= i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mul_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer * i32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i)| n * i),
        function_g: &(|(n, i): (BigInt, i32)| num_mul_i32(n, i)),
        function_h: &(|(n, i): (rugint::Integer, i32)| n * i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        z_cons: &(|&(ref n, i)| (integer_to_rugint_integer(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer * i32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_mul_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer * i32 evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, i)| n * i),
        function_g: &(|(n, i)| &n * i),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
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

pub fn benchmark_i32_mul_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} i32 * Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(i, n)| i * n),
        function_g: &(|(i, n): (i32, rugint::Integer)| i * n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(i, ref n)| (i, integer_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "i32 * Integer",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_i32_mul_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} i32 * Integer evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(i, n)| i * n),
        function_g: &(|(i, n)| i * &n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
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
