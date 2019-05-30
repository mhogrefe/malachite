use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::signeds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_i64);
    register_bench!(
        registry,
        None,
        benchmark_integer_from_i64_library_comparison
    );
}

fn demo_integer_from_i64(gm: GenerationMode, limit: usize) {
    for i in signeds::<i64>(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

fn benchmark_integer_from_i64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from(i64)",
        BenchmarkType::LibraryComparison,
        signeds::<i64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| usize::checked_from(i.significant_bits()).unwrap()),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
        ],
    );
}
