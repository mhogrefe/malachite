use malachite_base::limbs::limbs_leading_zero_limbs;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_leading_zero_limbs);
    register_bench!(registry, Small, benchmark_limbs_leading_zero_limbs);
}

fn demo_limbs_leading_zero_limbs(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned::<u32>(gm).take(limit) {
        println!(
            "limbs_leading_zero_limbs({:?}) = {:?}",
            xs,
            limbs_leading_zero_limbs(&xs)
        );
    }
}

fn benchmark_limbs_leading_zero_limbs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_leading_zero_limbs(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|limbs| no_out!(limbs_leading_zero_limbs(&limbs))),
        )],
    );
}
