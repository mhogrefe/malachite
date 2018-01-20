use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::natural::Natural;
use num::BigUint;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_u32(x: &mut BigUint, u: u32) {
    *x = BigUint::from(u);
}

type It = Iterator<Item = (Natural, u32)>;

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

pub fn demo_natural_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_natural_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, u): (Natural, u32)| n.assign(u)),
        function_g: &(|(mut n, u): (BigUint, u32)| num_assign_u32(&mut n, u)),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n.assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        z_cons: &(|&(ref n, u)| (natural_to_rugint_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural.assign(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
