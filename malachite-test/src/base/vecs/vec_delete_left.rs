use malachite_base::vecs::vec_delete_left;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_usize_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_vec_delete_left);
    register_bench!(registry, Small, benchmark_vec_delete_left);
}

fn demo_vec_delete_left(gm: GenerationMode, limit: usize) {
    for (xs, delete_size) in pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm).take(limit) {
        let mut mut_xs = xs.clone();
        vec_delete_left(&mut mut_xs, delete_size);
        println!(
            "xs := {:?}; vec_delete_left(&mut xs, {}); xs = {:?}",
            xs, delete_size, mut_xs
        );
    }
}

fn benchmark_vec_delete_left(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "vec_delete_left(&mut Vec<Limb>, usize)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [(
            "Malachite",
            &mut (|(mut xs, delete_size)| vec_delete_left(&mut xs, delete_size)),
        )],
    );
}
