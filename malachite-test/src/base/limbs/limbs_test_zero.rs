use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_test_zero;

pub fn demo_limbs_test_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_test_zero({:?}) = {:?}",
            xs,
            limbs_test_zero(xs.as_slice())
        );
    }
}

pub fn benchmark_limbs_test_zero(gm: GenerationMode, limit: usize, file_name: &str) {
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
