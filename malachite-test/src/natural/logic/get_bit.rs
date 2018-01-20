use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use malachite_base::num::BitAccess;
use malachite_nz::natural::Natural;
use num::{BigUint, One, Zero};
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << index as usize) != BigUint::zero()
}

type It = Iterator<Item = (Natural, u64)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_get_bit(gm: GenerationMode, limit: usize) {
    for (n, index) in select_inputs(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

pub fn benchmark_natural_get_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.get_bit(u64)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(n, index): (Natural, u64)| n.get_bit(index)),
        function_g: &(|(n, index): (BigUint, u64)| num_get_bit(&n, index)),
        function_h: &(|(n, index): (rugint::Integer, u64)| n.get_bit(index as u32)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (natural_to_biguint(n), index)),
        z_cons: &(|&(ref n, index)| (natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural.get\\\\_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
