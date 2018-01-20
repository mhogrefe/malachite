use common::{integer_to_bigint, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_i64(x: &mut BigInt, i: i64) {
    *x = BigInt::from(i);
}

type It = Iterator<Item = (Integer, i64)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs(exhaustive_integers(), exhaustive_i()))
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

pub fn demo_integer_assign_i64(gm: GenerationMode, limit: usize) {
    for (mut n, i) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn benchmark_integer_assign_i64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(i64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, i): (Integer, i64)| n.assign(i)),
        function_g: &(|(mut n, i): (BigInt, i64)| num_assign_i64(&mut n, i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer.assign(i64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
