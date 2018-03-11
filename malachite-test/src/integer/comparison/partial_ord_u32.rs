use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned,
                      pairs_of_unsigned_and_integer, rm_pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use num::BigInt;
use std::cmp::Ordering;

pub fn num_partial_cmp_u32(x: &BigInt, u: u32) -> Option<Ordering> {
    x.partial_cmp(&BigInt::from(u))
}

pub fn demo_integer_partial_cmp_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

pub fn demo_u32_partial_cmp_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", u, n),
            Ordering::Equal => println!("{} = {}", u, n),
            Ordering::Greater => println!("{} > {}", u, n),
        }
    }
}

pub fn benchmark_integer_partial_cmp_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.partial_cmp(&u32)",
        BenchmarkType::Ordinary,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.partial_cmp(&y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_cmp_u32(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

pub fn benchmark_u32_partial_cmp_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.partial_cmp(&Integer)",
        BenchmarkType::Ordinary,
        rm_pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, (x, y))| no_out!(x.partial_cmp(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}
