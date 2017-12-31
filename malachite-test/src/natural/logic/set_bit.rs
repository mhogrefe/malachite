use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num::{BigUint, One};
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

pub fn num_set_bit(x: &mut BigUint, index: u64) {
    *x = x.clone() | (BigUint::one() << index as usize);
}

type It = Iterator<Item = (gmp::Natural, u64)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale).map(|i| i as u64)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_set_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_natural_set_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.set_bit(u64)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, index): (gmp::Natural, u64)| n.set_bit(index)),
        function_g: &(|(mut n, index): (native::Natural, u64)| n.set_bit(index)),
        function_h: &(|(mut n, index): (BigUint, u64)| num_set_bit(&mut n, index)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_num_biguint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "Natural.set\\\\_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
