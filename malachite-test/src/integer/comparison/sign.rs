use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::SignificantBits;
use num::bigint::Sign;
use num::BigInt;
use std::cmp::Ordering;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_sign);
    register_bench!(registry, Large, benchmark_integer_sign_library_comparison);
}

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}

fn demo_integer_sign(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

fn benchmark_integer_sign_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.sign()))),
            ("num", &mut (|(n, _, _)| no_out!(num_sign(&n)))),
            ("rug", &mut (|(_, n, _)| no_out!(n.cmp0()))),
        ],
    );
}
