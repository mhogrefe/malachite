use malachite_base::conversion::CheckedFrom;
use malachite_base::crement::Crementable;
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_increment);
    register_bench!(registry, Large, benchmark_natural_increment);
}

fn demo_natural_increment(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let n_old = n.clone();
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

fn benchmark_natural_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.increment()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.increment()))],
    );
}
