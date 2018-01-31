use common::GenerationMode;
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use malachite_base::num::PartialOrdAbs;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use std::cmp::Ordering;

pub fn demo_integer_partial_cmp_abs_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        match n.partial_cmp_abs(&u).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", n, u),
            Ordering::Equal => println!("|{}| = |{}|", n, u),
            Ordering::Greater => println!("|{}| > |{}|", n, u),
        }
    }
}

pub fn demo_u32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        match PartialOrdAbs::partial_cmp_abs(&u, &n).unwrap() {
            Ordering::Less => println!("|{}| < |{}|", u, n),
            Ordering::Equal => println!("|{}| = |{}|", u, n),
            Ordering::Greater => println!("|{}| > |{}|", u, n),
        }
    }
}

pub fn benchmark_integer_partial_cmp_abs_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.partial_cmp_abs(&u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &(|(n, u): (Integer, u32)| n.partial_cmp_abs(&u)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.partial\\\\_cmp\\\\_abs(\\\\&u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32.partial_cmp_abs(&Integer)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_unsigned_and_integer::<u32>(gm),
        function_f: &(|(u, n): (u32, Integer)| PartialOrdAbs::partial_cmp_abs(&u, &n)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "u32.partial\\\\_cmp\\\\_abs(\\\\&Integer)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
