use common::{integer_to_bigint, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash<T: Hash>(n: &T) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

type It = Iterator<Item = Integer>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_integers())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_integers(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_hash(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

pub fn benchmark_integer_hash(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer hash", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n| hash(&n)),
        function_g: &(|n| hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
