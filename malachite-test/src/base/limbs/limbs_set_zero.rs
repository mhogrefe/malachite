use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_set_zero;
use malachite_nz::platform::Limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_set_zero);
    register_bench!(registry, Small, benchmark_limbs_set_zero);
}

fn demo_limbs_set_zero(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned::<Limb>(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_set_zero(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_set_zero(&mut limbs); limbs = {:?}",
            limbs, mut_limbs
        );
    }
}

fn benchmark_limbs_set_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_set_zero(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [("malachite", &mut (|mut limbs| limbs_set_zero(&mut limbs)))],
    );
}
