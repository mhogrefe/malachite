use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint,
             GenerationMode};
use malachite_base::traits::AbsAssign;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num::{self, Signed};
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions4, benchmark_2, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

type It = Iterator<Item = gmp::Integer>;

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

pub fn demo_integer_abs_assign(gm: GenerationMode, limit: usize) {
    for mut n in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_abs(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("abs({}) = {}", n.clone(), n.abs());
    }
}

pub fn demo_integer_abs_ref(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("abs_ref(&{}) = {}", n, n.abs_ref());
    }
}

pub fn demo_integer_natural_abs(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("natural_abs({}) = {}", n.clone(), n.natural_abs());
    }
}

pub fn demo_integer_natural_abs_ref(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("natural_abs_ref(&{}) = {}", n, n.natural_abs_ref());
    }
}

pub fn benchmark_integer_abs_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.abs_assign()", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs()),
        function_h: &(|n: num::BigInt| n.abs()),
        function_i: &(|mut n: rugint::Integer| n.abs().sign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        z_cons: &(|x| gmp_integer_to_num_bigint(x)),
        w_cons: &(|x| gmp_integer_to_rugint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Integer.abs\\\\_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.abs()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.abs() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: native::Integer| n.abs()),
        function_g: &(|n: native::Integer| n.abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.abs()",
        g_name: "Integer.abs_ref()",
        title: "Integer.abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_natural_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.natural_abs()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.natural\\\\_abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_natural_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.natural_abs() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: native::Integer| n.natural_abs()),
        function_g: &(|n: native::Integer| n.natural_abs_ref()),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.natural_abs()",
        g_name: "Integer.natural_abs_ref()",
        title: "Integer.natural\\\\_abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
