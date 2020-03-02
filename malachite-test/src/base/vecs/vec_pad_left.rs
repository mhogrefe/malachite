use malachite_base::vecs::vec_pad_left;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_small_usize_and_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_vec_pad_left);
    register_bench!(registry, Small, benchmark_vec_pad_left);
}

fn demo_vec_pad_left(gm: GenerationMode, limit: usize) {
    for (xs, pad_size, pad_value) in
        triples_of_unsigned_vec_small_usize_and_unsigned::<Limb>(gm).take(limit)
    {
        let mut mut_xs = xs.clone();
        vec_pad_left(&mut mut_xs, pad_size, pad_value);
        println!(
            "xs := {:?}; vec_pad_left(&mut xs, {}, {}); xs = {:?}",
            xs, pad_size, pad_value, mut_xs
        );
    }
}

fn benchmark_vec_pad_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "vec_pad_left(&mut Vec<u32>, usize, u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_usize_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, pad_size, pad_value)| vec_pad_left(&mut xs, pad_size, pad_value)),
        )],
    );
}
