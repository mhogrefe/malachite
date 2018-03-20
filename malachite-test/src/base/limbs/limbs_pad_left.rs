use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_small_usize_and_unsigned;
use malachite_base::limbs::limbs_pad_left;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_pad_left);
    register_bench!(registry, Small, benchmark_limbs_pad_left);
}

fn demo_limbs_pad_left(gm: GenerationMode, limit: usize) {
    for (limbs, pad_size, pad_limb) in
        triples_of_unsigned_vec_small_usize_and_unsigned(gm).take(limit)
    {
        let mut mut_limbs = limbs.clone();
        limbs_pad_left(&mut mut_limbs, pad_size, pad_limb);
        println!(
            "limbs := {:?}; limbs_pad_left(&mut limbs, {}, {}); x = {:?}",
            limbs, pad_size, pad_limb, mut_limbs
        );
    }
}

fn benchmark_limbs_pad_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_pad_left(&mut Vec<u32>, usize, u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_usize_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|(mut limbs, pad_size, pad_limb)| {
                    limbs_pad_left(&mut limbs, pad_size, pad_limb)
                }),
            ),
        ],
    );
}
