use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use malachite_base::num::PartialOrdAbs;
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
    m_run_benchmark(
        "Integer.partial_cmp_abs(&u32)",
        BenchmarkType::Ordinary,
        pairs_of_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y)))),
        ],
    );
}

pub fn benchmark_u32_partial_cmp_abs_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.partial_cmp_abs(&Integer)",
        BenchmarkType::Ordinary,
        pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(x, y)| no_out!(x.partial_cmp_abs(&y)))),
        ],
    );
}
