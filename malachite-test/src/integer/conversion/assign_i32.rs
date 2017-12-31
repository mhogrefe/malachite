use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint,
             GenerationMode};
use malachite_base::traits::Assign;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions4, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_i;
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};

pub fn num_assign_i32(x: &mut num::BigInt, i: i32) {
    *x = num::BigInt::from(i);
}

type It = Iterator<Item = (gmp::Integer, i32)>;

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

pub fn demo_integer_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn benchmark_integer_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(i32)", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(mut n, i): (gmp::Integer, i32)| n.assign(i)),
        function_g: &(|(mut n, i): (native::Integer, i32)| n.assign(i)),
        function_h: &(|(mut n, i): (num::BigInt, i32)| num_assign_i32(&mut n, i)),
        function_i: &(|(mut n, i): (rugint::Integer, i32)| n.assign(i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (gmp_integer_to_native(n), i)),
        z_cons: &(|&(ref n, i)| (gmp_integer_to_num_bigint(n), i)),
        w_cons: &(|&(ref n, i)| (gmp_integer_to_rugint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.assign(i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
