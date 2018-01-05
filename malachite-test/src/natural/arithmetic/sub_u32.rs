use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer,
             GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use natural::comparison::partial_ord_u32::num_partial_cmp_u32;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::Ordering;

#[allow(unknown_lints, assign_op_pattern)]
pub fn num_sub_u32(mut x: num::BigUint, u: u32) -> Option<num::BigUint> {
    if num_partial_cmp_u32(&x, u) != Some(Ordering::Less) {
        x = x - num::BigUint::from(u);
        Some(x)
    } else {
        None
    }
}

pub fn rugint_sub_u32(x: rugint::Integer, u: u32) -> Option<rugint::Integer> {
    if x >= u {
        Some(x - u)
    } else {
        None
    }
}

type It1 = Iterator<Item = (gmp::Natural, u32)>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_inputs_2().filter(|&(ref n, u)| *n >= u))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_inputs_2(scale).filter(|&(ref n, u)| *n >= u))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (gmp::Natural, u32)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

type It3 = Iterator<Item = (u32, gmp::Natural)>;

pub fn exhaustive_inputs_3() -> Box<It3> {
    Box::new(exhaustive_pairs(exhaustive_u(), exhaustive_naturals()))
}

pub fn random_inputs_3(scale: u32) -> Box<It3> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_naturals(seed, scale)),
    ))
}

pub fn select_inputs_3(gm: GenerationMode) -> Box<It3> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_3(),
        GenerationMode::Random(scale) => random_inputs_3(scale),
    }
}

pub fn demo_natural_sub_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs_1(gm).take(limit) {
        let n_old = n.clone();
        n -= u;
        println!("x := {}; x -= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_sub_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", n_old, u, n - u);
    }
}

pub fn demo_natural_sub_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs_2(gm).take(limit) {
        println!("&{} - {} = {:?}", n, u, &n - u);
    }
}

pub fn demo_u32_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in select_inputs_3(gm).take(limit) {
        let n_old = n.clone();
        println!("{} - {} = {:?}", u, n_old, u - &n);
    }
}

pub fn benchmark_natural_sub_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural -= u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut n, u)| n -= u),
        function_g: &(|(mut n, u): (native::Natural, u32)| n -= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n -= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (gmp_natural_to_native(n), u)),
        z_cons: &(|&(ref n, u)| (gmp_natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural -= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural - u32", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs_2(gm),
        function_f: &(|(n, u)| n - u),
        function_g: &(|(n, u): (native::Natural, u32)| n - u),
        function_h: &(|(n, u): (num::BigUint, u32)| num_sub_u32(n, u)),
        function_i: &(|(n, u): (rugint::Integer, u32)| n - u),
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
        title: "Natural - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Natural - u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(n, u)| &n - u),
        function_g: &(|(n, u): (native::Natural, u32)| &n - u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, other)| (gmp_natural_to_native(n), other)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "\\\\&Natural - u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_sub_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 - Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_3(gm),
        function_f: &(|(u, n)| u - &n),
        function_g: &(|(u, n): (u32, native::Natural)| u - &n),
        function_h: &(|(u, n): (u32, rugint::Integer)| u - n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, gmp_natural_to_native(n))),
        z_cons: &(|&(u, ref n)| (u, gmp_natural_to_rugint_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "u32 - Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
