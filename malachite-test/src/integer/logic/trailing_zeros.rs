use malachite_base::num::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;

pub fn integer_trailing_zeros_alt(n: &Integer) -> Option<u64> {
    if *n == 0 as Limb {
        None
    } else {
        Some(n.twos_complement_bits().take_while(|&b| !b).count() as u64)
    }
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_trailing_zeros);
    register_bench!(registry, Large, benchmark_integer_trailing_zeros_algorithms);
}

fn demo_integer_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_integer_trailing_zeros_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.trailing_zeros()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.trailing_zeros()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(integer_trailing_zeros_alt(&n))),
            ),
        ],
    );
}
