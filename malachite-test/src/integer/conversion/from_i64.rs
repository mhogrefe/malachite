use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::signeds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;

pub fn demo_integer_from_i64(gm: GenerationMode, limit: usize) {
    for i in signeds::<i64>(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

pub fn benchmark_integer_from_i64_library_comparison(
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
        &(|&i| i.significant_bits() as usize),
        "i.significant_bits()",
        &[
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
        ],
    );
}
