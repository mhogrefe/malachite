use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_set_zero;

pub fn demo_limbs_set_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_xs = xs.clone();
        limbs_set_zero(&mut mut_xs);
        println!("xs := {:?}; limbs_set_zero(&mut xs); x = {:?}", xs, mut_xs);
    }
}

pub fn benchmark_limbs_set_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_set_zero(&mut [u32])",
        BenchmarkType::Ordinary,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &[("malachite", &mut (|mut limbs| limbs_set_zero(&mut limbs)))],
    );
}
