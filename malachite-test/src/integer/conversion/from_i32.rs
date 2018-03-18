use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::signeds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

pub fn demo_integer_from_i32(gm: GenerationMode, limit: usize) {
    for i in signeds::<i32>(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

pub fn benchmark_integer_from_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from(i32)",
        BenchmarkType::LibraryComparison,
        signeds::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&i| i.significant_bits() as usize),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
            ("rug", &mut (|i| no_out!(rug::Integer::from(i)))),
        ],
    );
}
