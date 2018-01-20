use common::{integer_to_rugint_integer, GenerationMode};
use malachite_base::num::SignificantBits;
use malachite_base::traits::NotAssign;
use malachite_nz::integer::Integer;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

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

pub fn demo_integer_not_assign(gm: GenerationMode, limit: usize) {
    for mut n in select_inputs(gm).take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {}; n.not_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_not(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_integer_not_ref(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn benchmark_integer_not_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.not_assign()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|mut n: Integer| n.not_assign()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.not_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_not(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} !Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: Integer| !n),
        function_g: &(|n: rugint::Integer| !n),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "-Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_not_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} !Integer evaluation strategy", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: Integer| !n),
        function_g: &(|n: Integer| !&n),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
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
