use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{integers, nrm_integers};
use malachite_base::num::SignificantBits;

pub fn demo_integer_significant_bits(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_integer_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.significant_bits()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "malachite",
                &mut (|(_, _, n)| no_out!(n.significant_bits())),
            ),
            ("num", &mut (|(n, _, _)| no_out!(n.bits()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.significant_bits()))),
        ],
    );
}
