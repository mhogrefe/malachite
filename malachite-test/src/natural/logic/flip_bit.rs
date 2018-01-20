use common::{natural_to_rugint_integer, GenerationMode};
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::natural::Natural;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::cmp::max;

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

pub fn demo_natural_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_natural_flip_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.flip_bit(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, index): (Natural, u64)| n.flip_bit(index)),
        function_g: &(|(mut n, index): (rugint::Integer, u64)| {
            n.invert_bit(index as u32);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (natural_to_rugint_integer(n), index)),
        x_param: &(|&(ref n, index)| max(n.significant_bits(), index) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Natural.flip\\\\_bit(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
