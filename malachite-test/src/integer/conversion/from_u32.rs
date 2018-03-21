use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_u32);
    register_bench!(
        registry,
        None,
        benchmark_integer_from_u32_library_comparison
    );
}

fn demo_integer_from_u32(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u32>(gm).take(limit) {
        println!("from({}) = {}", u, Integer::from(u));
    }
}

fn benchmark_integer_from_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| u.significant_bits() as usize),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Integer::from(u)))),
            ("num", &mut (|u| no_out!(BigInt::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}
