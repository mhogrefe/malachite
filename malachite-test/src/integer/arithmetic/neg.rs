use common::{gmp_integer_to_native, gmp_integer_to_num_bigint, gmp_integer_to_rugint,
             GenerationMode};
use malachite_base::traits::NegAssign;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use num;
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

pub fn demo_integer_neg_assign(gm: GenerationMode, limit: usize) {
    for mut n in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {}; n.neg_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_neg(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

pub fn demo_integer_neg_ref(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

pub fn benchmark_integer_neg_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.neg_assign()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|mut n: gmp::Integer| n.neg_assign()),
        function_g: &(|mut n: native::Integer| n.neg_assign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.neg_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} -Integer", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| -n),
        function_g: &(|n: native::Integer| -n),
        function_h: &(|n: num::BigInt| -n),
        function_i: &(|n: rugint::Integer| -n),
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
        title: "-Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} -Integer evaluation strategy", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: native::Integer| -n),
        function_g: &(|n: native::Integer| -&n),
        x_cons: &(|x| gmp_integer_to_native(x)),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "-Integer",
        g_name: "-\\\\&Integer",
        title: "-Integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
