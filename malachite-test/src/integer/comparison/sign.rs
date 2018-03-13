use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::SignificantBits;
use num::BigInt;
use num::bigint::Sign;
use std::cmp::Ordering;

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}

pub fn demo_integer_sign(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

pub fn benchmark_integer_sign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, n)| no_out!(n.sign()))),
            ("num", &mut (|(n, _, _)| no_out!(num_sign(&n)))),
            ("rug", &mut (|(_, n, _)| no_out!(n.cmp0()))),
        ],
    );
}
