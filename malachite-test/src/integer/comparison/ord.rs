use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integers, pairs_of_integers};
use malachite_base::num::SignificantBits;
use std::cmp::{max, Ordering};

pub fn demo_integer_cmp(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_integer_cmp(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.cmp(&Integer)",
        BenchmarkType::Ordinary,
        nrm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.cmp(&y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.cmp(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.cmp(&y)))),
        ],
    );
}
