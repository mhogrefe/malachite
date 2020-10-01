use malachite_base::slices::slice_set_zero;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_slice_set_zero);
    register_bench!(registry, Small, benchmark_slice_set_zero);
}

fn demo_slice_set_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned::<Limb>(gm).take(limit) {
        let mut mut_xs = xs.clone();
        slice_set_zero(&mut mut_xs);
        println!("xs := {:?}; slice_set_zero(&mut xs); xs = {:?}", xs, mut_xs);
    }
}

fn benchmark_slice_set_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "slice_set_zero(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "xs.len()",
        &mut [("Malachite", &mut (|mut xs| slice_set_zero(&mut xs)))],
    );
}
