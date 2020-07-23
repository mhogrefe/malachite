use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::hash::hash;
use malachite_test::inputs::natural::{naturals, nrm_naturals};

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
    run_benchmark(
        "Natural hash",
        BenchmarkType::LibraryComparison,
        nrm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(hash(&n)))),
            ("num", &mut (|(_, n, _)| no_out!(hash(&n)))),
            ("rug", &mut (|(n, _, _)| no_out!(hash(&n)))),
        ],
    );
}
