use common::GenerationMode;
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use malachite_base::num::PartialOrdAbs;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use std::cmp::Ordering;

pub fn demo_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        match n.partial_cmp_abs(&i).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, i),
            Ordering::Equal => println!("|{}| = |{}|", n, i),
            Ordering::Greater => println!("|{}| > |{}|", n, i),
        }
    }
}

pub fn demo_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        match PartialOrdAbs::partial_cmp_abs(&i, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", i, n),
            Ordering::Equal => println!("|{}| = |{}|", i, n),
            Ordering::Greater => println!("|{}| > |{}|", i, n),
        }
    }
}

pub fn benchmark_integer_partial_cmp_abs_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.partial_cmp_abs(&i32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_signed::<i32>(gm),
        function_f: &(|(n, i): (Integer, i32)| n.partial_cmp_abs(&i)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} i32.partial_cmp_abs(&Integer)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_signed_and_integer(gm),
        function_f: &(|(i, n): (i32, Integer)| PartialOrdAbs::partial_cmp_abs(&i, &n)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "i32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
