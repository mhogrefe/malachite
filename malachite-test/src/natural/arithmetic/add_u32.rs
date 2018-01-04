use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer,
             GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

#[allow(unknown_lints, assign_op_pattern)]
pub fn num_add_u32(mut x: num::BigUint, u: u32) -> num::BigUint {
    x = x + num::BigUint::from(u);
    x
}

type It1 = Iterator<Item = (gmp::Natural, u32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (u32, gmp::Natural)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs(exhaustive_u(), exhaustive_naturals()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_natural_add_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        n += u;
        println!("x := {}; x += {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_add_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", n_old, u, n + u);
    }
}

pub fn demo_natural_add_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_1(gm).take(limit) {
        println!("&{} + {} = {}", n, u, &n + u);
    }
}

pub fn demo_u32_add_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", u, n_old, u + n);
    }
}

pub fn demo_u32_add_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + &{} = {}", u, n_old, u + &n);
    }
}

pub fn benchmark_natural_add_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural += u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut n, u)| n += u),
        function_g: &(|(mut n, u): (native::Natural, u32)| n += u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n += u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural += u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural + u32", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, u)| n + u),
        function_g: &(|(n, u): (native::Natural, u32)| n + u),
        function_h: &(|(n, u): (num::BigUint, u32)| num_add_u32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n + u),
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
        title: "Natural + u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural + u32 evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_1(gm),
        function_f: &(|(n, u)| n + u),
        function_g: &(|(n, u)| &n + u),
        x_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Natural + u32",
        g_name: "\\\\&Natural + u32",
        title: "Natural + u32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_add_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 + Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_2(gm),
        function_f: &(|(u, n)| u + n),
        function_g: &(|(u, n): (u32, native::Natural)| u + n),
        function_h: &(|(u, n): (u32, rugint::Integer)| u + n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_natural_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 + Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_add_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} u32 + Natural evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(u, n)| u + n),
        function_g: &(|(u, n)| u + &n),
        x_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        y_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "u32 + Natural",
        g_name: "u32 + \\\\&Natural",
        title: "u32 + Natural evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
