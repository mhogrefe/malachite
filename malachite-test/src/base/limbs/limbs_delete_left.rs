use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::pairs_of_unsigned_vec_and_small_usize_var_1;
use malachite_base::limbs::limbs_delete_left;

pub fn demo_limbs_delete_left(gm: GenerationMode, limit: usize) {
    for (limbs, delete_size) in pairs_of_unsigned_vec_and_small_usize_var_1(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_delete_left(&mut mut_limbs, delete_size);
        println!(
            "limbs := {:?}; limbs_delete_left(&mut limbs, {}); x = {:?}",
            limbs, delete_size, mut_limbs
        );
    }
}

pub fn benchmark_limbs_delete_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_delete_left(&mut Vec<u32>, usize)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_usize_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|(mut limbs, delete_size)| limbs_delete_left(&mut limbs, delete_size)),
            ),
        ],
    );
}
