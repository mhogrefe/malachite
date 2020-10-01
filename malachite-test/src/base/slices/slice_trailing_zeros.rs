use malachite_base::slices::slice_trailing_zeros;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_slice_trailing_zeros);
    register_bench!(registry, Small, benchmark_slice_trailing_zeros);
}

fn demo_slice_trailing_zeros(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned::<u32>(gm).take(limit) {
        println!(
            "slice_trailing_zeros({:?}) = {:?}",
            xs,
            slice_trailing_zeros(&xs)
        );
    }
}

fn benchmark_slice_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "slice_trailing_zeros(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "xs.len()",
        &mut [("Malachite", &mut (|xs| no_out!(slice_trailing_zeros(&xs))))],
    );
}
