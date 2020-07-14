use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_nz::integer::Integer;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use malachite_test::inputs::base::small_unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_integer_low_mask);
    register_ns_bench!(registry, Large, benchmark_integer_low_mask);
}

fn demo_integer_low_mask(gm: NoSpecialGenerationMode, limit: usize) {
    for bits in small_unsigneds(gm).take(limit) {
        println!("Integer::low_mask({}) = {}", bits, Integer::low_mask(bits));
    }
}

fn benchmark_integer_low_mask(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        &format!("Integer.low_mask(u64)"),
        BenchmarkType::Single,
        small_unsigneds(gm),
        gm.name(),
        limit,
        file_name,
        &(|&bits| usize::exact_from(bits)),
        "bits",
        &mut [("malachite", &mut (|bits| no_out!(Integer::low_mask(bits))))],
    );
}
