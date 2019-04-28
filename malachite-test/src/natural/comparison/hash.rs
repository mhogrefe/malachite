use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use hash::hash;
use inputs::natural::{naturals, nrm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_hash);
    register_bench!(registry, Large, benchmark_natural_hash_library_comparison);
}

fn demo_natural_hash(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

fn benchmark_natural_hash_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural hash",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
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
