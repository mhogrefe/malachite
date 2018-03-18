use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;

pub fn demo_integer_from_u64(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u64>(gm).take(limit) {
        println!("from({}) = {}", u, Integer::from(u));
    }
}

pub fn benchmark_integer_from_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from(u64)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| u.significant_bits() as usize),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Integer::from(u)))),
            ("num", &mut (|u| no_out!(BigInt::from(u)))),
        ],
    );
}
