use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use inputs::natural::{pairs_of_naturals, pairs_of_naturals_var_1};
use malachite_base::num::SignificantBits;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use std::cmp::max;
use std::ops::Sub;

pub fn checked_sub<T: Ord + Sub>(x: T, y: T) -> Option<<T as Sub>::Output> {
    if x >= y {
        Some(x - y)
    } else {
        None
    }
}

pub fn demo_natural_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {:?}", x_old, y, x - &y);
    }
}

pub fn demo_natural_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} - &{} = {:?}", x, y, &x - &y);
    }
}

pub fn benchmark_natural_sub_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural -= &Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals_var_1(gm),
        function_f: &(|(mut x, y)| x -= &y),
        function_g: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x -= &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Natural -= \\\\&Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural - &Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(x, y)| x - &y),
        function_g: &(|(x, y)| checked_sub(x, y)),
        function_h: &(|(x, y)| checked_sub(x, y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
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
        xs: pairs_of_naturals(gm),
        function_f: &(|(x, y)| x - &y),
        function_g: &(|(x, y)| &x - &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
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
