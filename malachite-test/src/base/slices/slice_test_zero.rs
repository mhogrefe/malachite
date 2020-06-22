use malachite_base::slices::slice_test_zero;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_slice_test_zero);
    register_bench!(registry, Small, benchmark_slice_test_zero);
}

fn demo_slice_test_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned::<u32>(gm).take(limit) {
        println!("slice_test_zero({:?}) = {:?}", xs, slice_test_zero(&xs));
    }
}

fn benchmark_slice_test_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "slice_test_zero(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "xs.len()",
        &mut [("malachite", &mut (|xs| no_out!(slice_test_zero(&xs))))],
    );
}
