use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use hash::hash;
use inputs::integer::{integers, nm_integers};
use malachite_base::num::SignificantBits;

pub fn demo_integer_hash(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

pub fn benchmark_integer_hash_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer hash",
        BenchmarkType::LibraryComparison,
        nm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, n)| no_out!(hash(&n)))),
            ("rug", &mut (|(n, _)| no_out!(hash(&n)))),
        ],
    );
}
