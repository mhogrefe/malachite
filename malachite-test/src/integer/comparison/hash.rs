use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use hash::hash;
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::traits::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_hash);
    register_bench!(registry, Large, benchmark_integer_hash_library_comparison);
}

fn demo_integer_hash(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

fn benchmark_integer_hash_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer hash",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(hash(&n)))),
            ("num", &mut (|(_, n, _)| no_out!(hash(&n)))),
            ("rug", &mut (|(n, _, _)| no_out!(hash(&n)))),
        ],
    );
}
