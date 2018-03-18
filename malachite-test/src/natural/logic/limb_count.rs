use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub fn demo_natural_limb_count(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limb_count({}) = {}", n, n.limb_count());
    }
}

pub fn benchmark_natural_limb_count(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.limb_count()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.limb_count())))],
    );
}
