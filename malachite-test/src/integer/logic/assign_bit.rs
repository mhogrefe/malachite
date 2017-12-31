use common::{gmp_integer_to_native, gmp_integer_to_rugint, GenerationMode};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, lex_pairs, random_triples};
use std::cmp::max;

type It = Iterator<Item = (gmp::Integer, u64, bool)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(
        lex_pairs(
            exhaustive_pairs(exhaustive_integers(), exhaustive_u()),
            exhaustive_bools(),
        ).map(|((n, index), bit)| (n, index, bit)),
    )
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale).map(|i| u64::from(i))),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_assign_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

pub fn benchmark_integer_assign_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign_bit(u64)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, index, bit): (gmp::Integer, u64, bool)| n.assign_bit(index, bit)),
        function_g: &(|(mut n, index, bit): (native::Integer, u64, bool)| n.assign_bit(index, bit)),
        function_h: &(|(mut n, index, bit): (rugint::Integer, u64, bool)| {
            n.set_bit(index as u32, bit);
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, bit)| (gmp_integer_to_native(n), index, bit)),
        z_cons: &(|&(ref n, index, bit)| (gmp_integer_to_rugint(n), index, bit)),
        x_param: &(|&(ref n, index, _)| max(n.significant_bits(), index) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer.assign\\\\_bit(u64, bool)",
        x_axis_label: "max(n.significant\\\\_bits(), index)",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
