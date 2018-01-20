use common::{integer_to_bigint, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_u64(x: &mut BigInt, u: u64) {
    *x = BigInt::from(u);
}

type It = Iterator<Item = (Integer, u64)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_integer_assign_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Integer, u64)| n.assign(u)),
        function_g: &(|(mut n, u): (BigInt, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_bigint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer.assign(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
