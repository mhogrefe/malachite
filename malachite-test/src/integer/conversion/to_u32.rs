use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{integers, rm_integers};
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_to_u32);
    register_demo!(registry, demo_integer_to_u32_wrapping);
    register_bench!(registry, Large, benchmark_integer_to_u32_library_comparison);
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_u32_wrapping_library_comparison
    );
}

fn demo_integer_to_u32(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

fn demo_integer_to_u32_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u32_wrapping({}) = {:?}", n, n.to_u32_wrapping());
    }
}

fn benchmark_integer_to_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_u32()",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(n.to_u32()))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32()))),
        ],
    );
}

fn benchmark_integer_to_u32_wrapping_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_u32_wrapping()",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(n.to_u32_wrapping()))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32_wrapping()))),
        ],
    );
}
