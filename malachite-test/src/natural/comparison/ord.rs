use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals};
use malachite_base::num::SignificantBits;
use std::cmp::{max, Ordering};

pub fn demo_natural_cmp(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_natural_cmp(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.cmp(&Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| max(x.significant_bits(), y.significant_bits()) as usize),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.cmp(&y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.cmp(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.cmp(&y)))),
        ],
    );
}
