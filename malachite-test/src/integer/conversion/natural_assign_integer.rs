use common::{gmp_integer_to_native, gmp_integer_to_rugint, gmp_natural_to_native,
             gmp_natural_to_rugint_integer, GenerationMode};
use malachite_base::traits::Assign;
use malachite_gmp as gmp;
use malachite_native as native;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_natural_integers, random_natural_integers};
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs, random_pairs};
use std::cmp::max;

type It = Iterator<Item = (gmp::natural::Natural, gmp::integer::Integer)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_integers(),
    ))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_natural_integers(seed, scale)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_natural_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_natural_assign_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(Integer)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(mut x, y): (gmp::natural::Natural, gmp::integer::Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (native::natural::Natural, native::integer::Integer)| {
            x.assign(y)
        }),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_rugint_integer(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural.assign(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.assign(Integer) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut x, y): (native::natural::Natural, native::integer::Integer)| {
            x.assign(y)
        }),
        function_g: &(|(mut x, y): (native::natural::Natural, native::integer::Integer)| {
            x.assign(&y)
        }),
        x_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural.assign(Integer)",
        g_name: "Natural.assign(\\\\&Integer)",
        title: "Natural.assign(Integer) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
