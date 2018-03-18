use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use malachite_base::num::PartialOrdAbs;
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
    m_run_benchmark(
        "Integer.partial_cmp_abs(&i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y)))),
        ],
    );
}

pub fn benchmark_i32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32.partial_cmp_abs(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y)))),
        ],
    );
}
