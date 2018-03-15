use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, rm_integers};
use malachite_base::num::SignificantBits;

pub fn demo_integer_to_i32(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i32({}) = {:?}", n, n.to_i32());
    }
}

pub fn demo_integer_to_i32_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i32_wrapping({}) = {:?}", n, n.to_i32_wrapping());
    }
}

pub fn benchmark_integer_to_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_i32()",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, n)| no_out!(n.to_i32()))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32()))),
        ],
    );
}

pub fn benchmark_integer_to_i32_wrapping_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_i32_wrapping()",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, n)| no_out!(n.to_i32_wrapping()))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32_wrapping()))),
        ],
    );
}
