use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint,
             GenerationMode};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

type It = Iterator<Item = (gmp::Integer, gmp::Integer)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_pairs_from_single(exhaustive_integers()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs_from_single(random_integers(
        &EXAMPLE_SEED,
        scale,
    )))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {}; x -= {}; x = {}", x_old, y, x);
    }
}

pub fn demo_integer_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_integer_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

pub fn demo_integer_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

pub fn demo_integer_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

pub fn demo_integer_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in select_inputs(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

pub fn benchmark_integer_sub_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer -= Integer", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs(gm),
        function_f: &(|(mut x, y)| x -= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x -= y),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x -= y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Integer -= Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer -= Integer evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut x, y)| x -= y),
        function_g: &(|(mut x, y): (native::Integer, native::Integer)| x -= &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Integer -= Integer",
        g_name: "Integer -= \\\\&Integer",
        title: "Integer -= Integer evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer - Integer", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(x, y)| x - y),
        function_g: &(|(x, y)| x - y),
        function_h: &(|(x, y)| x - y),
        function_i: &(|(x, y)| x - y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_num_bigint(x), gmp_integer_to_num_bigint(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_rugint(x), gmp_integer_to_rugint(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer - Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer - Integer evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(x, y)| x - y),
        function_g: &(|(x, y)| x - &y),
        function_h: &(|(x, y)| &x - y),
        function_i: &(|(x, y)| &x - &y),
        x_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        z_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        w_cons: &(|&(ref x, ref y)| (gmp_integer_to_native(x), gmp_integer_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Integer - Integer",
        g_name: "Integer - \\\\&Integer",
        h_name: "\\\\&Integer - Integer",
        i_name: "\\\\&Integer - \\\\&Integer",
        title: "Integer - Integer evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
