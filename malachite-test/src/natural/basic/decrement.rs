use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::positive_naturals;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;

pub fn demo_natural_decrement(gm: GenerationMode, limit: usize) {
    for mut n in positive_naturals(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_natural_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
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
