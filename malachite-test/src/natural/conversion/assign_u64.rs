use common::{natural_to_biguint, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::natural::Natural;
use num::BigUint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_u64(x: &mut BigUint, u: u64) {
    *x = BigUint::from(u);
}

type It = Iterator<Item = (Natural, u64)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_natural_assign_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Natural, u64)| n.assign(u)),
        function_g: &(|(mut n, u): (BigUint, u64)| num_assign_u64(&mut n, u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Natural.assign(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
