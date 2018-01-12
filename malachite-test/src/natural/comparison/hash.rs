use common::{natural_to_biguint, GenerationMode};
use malachite_nz::natural::Natural;
use num::BigUint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash<T: Hash>(n: &T) -> u64 {
    let mut s = DefaultHasher::new();
    n.hash(&mut s);
    s.finish()
}

type It = Iterator<Item = Natural>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_naturals())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_naturals(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_hash(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

pub fn benchmark_natural_hash(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural hash", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n| hash(&n)),
        function_g: &(|n: BigUint| hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Natural hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
