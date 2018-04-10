use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_u32_var_1;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_trailing_zeros);
    register_demo!(registry, demo_natural_trailing_zeros);
    register_bench!(registry, Small, benchmark_limbs_trailing_zeros);
    register_bench!(registry, Large, benchmark_natural_trailing_zeros);
}

fn demo_limbs_trailing_zeros(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_u32_var_1(gm).take(limit) {
        println!(
            "limbs_trailing_zeros({:?}) = {}",
            limbs,
            limbs_trailing_zeros(&limbs)
        );
    }
}

fn demo_natural_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_limbs_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_trailing_zeros(&[u32])",
        BenchmarkType::Single,
        vecs_of_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|limbs| no_out!(limbs_trailing_zeros(&limbs))),
            ),
        ],
    );
}

fn benchmark_natural_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.trailing_zeros()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.trailing_zeros())))],
    );
}
