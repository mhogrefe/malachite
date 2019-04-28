use malachite_base::crement::Crementable;
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::positive_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_decrement);
    register_bench!(registry, Large, benchmark_natural_decrement);
}

fn demo_natural_decrement(gm: GenerationMode, limit: usize) {
    for mut n in positive_naturals(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

fn benchmark_natural_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.decrement()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.decrement()))],
    );
}
