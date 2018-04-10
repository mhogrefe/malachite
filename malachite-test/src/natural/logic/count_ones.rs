use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_count_ones);
    register_demo!(registry, demo_natural_count_ones);
    register_bench!(registry, Small, benchmark_limbs_count_ones);
    register_bench!(registry, Large, benchmark_natural_count_ones);
}

fn demo_limbs_count_ones(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_count_ones({:?}) = {}",
            limbs,
            limbs_count_ones(&limbs)
        );
    }
}

fn demo_natural_count_ones(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("count_ones({}) = {}", n, n.count_ones());
    }
}

fn benchmark_limbs_count_ones(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_count_ones(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|limbs| no_out!(limbs_count_ones(&limbs))),
            ),
        ],
    );
}

fn benchmark_natural_count_ones(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.count_ones()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.count_ones())))],
    );
}
