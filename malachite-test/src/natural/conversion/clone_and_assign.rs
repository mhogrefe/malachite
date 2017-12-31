use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer,
             GenerationMode};
use malachite_base::traits::Assign;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_pairs_from_single, random_pairs_from_single};
use std::cmp::max;

type It1 = Iterator<Item = gmp::Natural>;

pub fn exhaustive_inputs_1() -> Box<It1> {
    Box::new(exhaustive_naturals())
}

pub fn random_inputs_1(scale: u32) -> Box<It1> {
    Box::new(random_naturals(&EXAMPLE_SEED, scale))
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

pub fn demo_natural_clone(gm: GenerationMode, limit: usize) {
    for n in select_inputs_1(gm).take(limit) {
        println!("clone({}) = {:?}", n, n.clone());
    }
}

pub fn demo_natural_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs_2(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_natural_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs_2(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_natural_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in select_inputs_2(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_natural_clone(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.clone()", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs_1(gm),
        function_f: &(|n: gmp::Natural| n.clone()),
        function_g: &(|n: native::Natural| n.clone()),
        function_h: &(|n: num::BigUint| n.clone()),
        function_i: &(|n: rugint::Integer| n.clone()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        z_cons: &(|x| gmp_natural_to_num_biguint(x)),
        w_cons: &(|x| gmp_natural_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural.clone()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_clone_from(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.clone_from(Natural)", gm.name());
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs_2(gm),
        function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.clone_from(&y)),
        function_g: &(|(mut x, y): (native::Natural, native::Natural)| x.clone_from(&y)),
        function_h: &(|(mut x, y): (num::BigUint, num::BigUint)| x.clone_from(&y)),
        function_i: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.clone_from(&y)),
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
        title: "Natural.clone\\\\_from(Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(Natural)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: select_inputs_2(gm),
        function_f: &(|(mut x, y): (gmp::Natural, gmp::Natural)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(y)),
        function_h: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
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
        title: "Natural.assign(Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.assign(Natural) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs_2(gm),
        function_f: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(y)),
        function_g: &(|(mut x, y): (native::Natural, native::Natural)| x.assign(&y)),
        x_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        y_cons: &(|&(ref x, ref y)| (gmp_natural_to_native(x), gmp_natural_to_native(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural.assign(Integer)",
        g_name: "Natural.assign(\\\\&Natural)",
        title: "Natural.assign(Natural) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
