use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_u64);
    register_bench!(
        registry,
        None,
        benchmark_natural_from_u64_library_comparison
    );
}

fn demo_natural_from_u64(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u64>(gm).take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}

fn benchmark_natural_from_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| u.significant_bits() as usize),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Natural::from(u)))),
            ("num", &mut (|u| no_out!(BigUint::from(u)))),
        ],
    );
}
