use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer,
             GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

#[allow(unknown_lints, assign_op_pattern)]
pub fn num_sub(mut x: num::BigUint, y: num::BigUint) -> Option<num::BigUint> {
    if x >= y {
        x = x - y;
        Some(x)
    } else {
        None
    }
}

pub fn rugint_sub(x: rugint::Integer, y: rugint::Integer) -> Option<rugint::Integer> {
    if x >= y {
        Some(x - y)
    } else {
        None
    }
}

type It1 = Iterator<Item = (gmp::Natural, gmp::Natural)>;

//TODO use subset_pairs
pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_inputs_2().filter(|&(ref x, ref y)| x >= y))
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_inputs_2(scale).filter(|&(ref x, ref y)| x >= y))
}

pub fn select_inputs_1(gm: GenerationMode) -> Box<It1> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_1(),
        GenerationMode::Random(scale) => random_inputs_1(scale),
    }
}

type It2 = Iterator<Item = (gmp::Natural, gmp::Natural)>;

pub fn exhaustive_inputs_2() -> Box<It2> {
    Box::new(exhaustive_pairs_from_single(exhaustive_naturals()))
}

pub fn random_inputs_2(scale: u32) -> Box<It2> {
    Box::new(random_pairs_from_single(random_naturals(
        &EXAMPLE_SEED,
        scale,
    )))
}

pub fn select_inputs_2(gm: GenerationMode) -> Box<It2> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs_2(),
        GenerationMode::Random(scale) => random_inputs_2(scale),
    }
}

pub fn demo_natural_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs_1(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs_2(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {:?}", x_old, y, x - &y);
    }
}

pub fn demo_natural_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs_2(gm).take(limit) {
        println!("&{} - &{} = {:?}", x, y, &x - &y);
    }
}

pub fn benchmark_natural_sub_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural -= &Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_1(gm),
        function_f: &(|(mut x, y)| x -= &y),
        function_g: &(|(mut x, y): (native::Natural, native::Natural)| x -= &y),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x -= &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| {
            (
                gmp_natural_to_rugint_integer(x),
                gmp_natural_to_rugint_integer(y),
            )
        }),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural -= \\\\&Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural - &Natural", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs_2(gm),
        function_f: &(|(x, y)| x - &y),
        function_g: &(|(x, y)| x - &y),
        function_h: &(|(x, y)| num_sub(x, y)),
        function_i: &(|(x, y)| rugint_sub(x, y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_natural_to_num_biguint(x), gmp_natural_to_num_biguint(y))),
        w_cons: &(|&(ref x, ref y)| {
            (
                gmp_natural_to_rugint_integer(x),
                gmp_natural_to_rugint_integer(y),
            )
        }),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural - \\\\&Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural - Natural evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(x, y)| x - &y),
        function_g: &(|(x, y)| &x - &y),
        x_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural - \\\\&Natural",
        g_name: "\\\\&Natural - \\\\&Natural",
        title: "Natural + Natural evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
