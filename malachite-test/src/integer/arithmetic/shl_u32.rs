use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use malachite_nz::integer::Integer;
use num::BigInt;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

type It = Iterator<Item = (Integer, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_shl_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n <<= u;
        println!("x := {}; x <<= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_integer_shl_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        println!("{} << {} = {}", n_old, u, n << u);
    }
}

pub fn demo_integer_shl_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in select_inputs(gm).take(limit) {
        println!("&{} << {} = {}", n, u, &n << u);
    }
}

pub fn benchmark_integer_shl_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer <<= u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u)| n <<= u),
        function_g: &(|(mut n, u): (rugint::Integer, u32)| n <<= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer <<= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shl_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer << u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(n, u)| n << u),
        function_g: &(|(n, u): (BigInt, u32)| n << u as usize),
        function_h: &(|(n, u): (rugint::Integer, u32)| n << u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_bigint(n), index)),
        z_cons: &(|&(ref n, index)| (integer_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_shl_u32_ref(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} &Integer << u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(n, u)| &n << u),
        function_g: &(|(n, u): (BigInt, u32)| &n << u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_bigint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "\\\\&Integer << u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
