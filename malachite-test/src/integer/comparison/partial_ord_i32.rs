use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed,
                      pairs_of_signed_and_integer, rm_pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;
use std::cmp::Ordering;

pub fn num_partial_cmp_i32(x: &BigInt, i: i32) -> Option<Ordering> {
    x.partial_cmp(&BigInt::from(i))
}

pub fn demo_integer_partial_cmp_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        match n.partial_cmp(&i).unwrap() {
            Ordering::Less => println!("{} < {}", n, i),
            Ordering::Equal => println!("{} = {}", n, i),
            Ordering::Greater => println!("{} > {}", n, i),
        }
    }
}

pub fn demo_i32_partial_cmp_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        match i.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", i, n),
            Ordering::Equal => println!("{} = {}", i, n),
            Ordering::Greater => println!("{} > {}", i, n),
        }
    }
}

pub fn benchmark_integer_partial_cmp_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.partial_cmp(&i32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.partial_cmp(&y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_cmp_i32(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

pub fn benchmark_i32_partial_cmp_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.partial_cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.partial_cmp(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}
