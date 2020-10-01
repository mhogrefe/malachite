use malachite_base::slices::slice_move_left;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::slices::slice_move_left_naive;
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_usize_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_slice_move_left);
    register_bench!(registry, Small, benchmark_slice_move_left_algorithms);
}

fn demo_slice_move_left(gm: GenerationMode, limit: usize) {
    for (xs, amount) in pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm).take(limit) {
        let mut mut_xs = xs.clone();
        slice_move_left(&mut mut_xs, amount);
        println!(
            "xs := {:?}; slice_move_left(&mut xs, {}); xs = {:?}",
            xs, amount, mut_xs
        );
    }
}

fn benchmark_slice_move_left_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "slice_move_left(&mut Vec<Limb>, usize)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_small_usize_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut xs, amount)| slice_move_left(&mut xs, amount)),
            ),
            (
                "naive",
                &mut (|(mut xs, amount)| slice_move_left_naive(&mut xs, amount)),
            ),
        ],
    );
}
