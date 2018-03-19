use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_test_zero;

pub fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_test_zero);
    register_bench!(registry, Small, benchmark_limbs_test_zero);
}

fn demo_limbs_test_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_test_zero({:?}) = {:?}",
            xs,
            limbs_test_zero(xs.as_slice())
        );
    }
}

fn benchmark_limbs_test_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_test_zero(&[u32])",
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
                &mut (|limbs| no_out!(limbs_test_zero(limbs.as_slice()))),
            ),
        ],
    );
}
