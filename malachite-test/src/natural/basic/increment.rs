use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;

pub fn demo_natural_increment(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let n_old = n.clone();
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_natural_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.increment()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}
