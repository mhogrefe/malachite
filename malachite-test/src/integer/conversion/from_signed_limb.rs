use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::signeds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_signed_limb);
    register_bench!(
        registry,
        None,
        benchmark_integer_from_signed_limb_library_comparison
    );
}

fn demo_integer_from_signed_limb(gm: GenerationMode, limit: usize) {
    for i in signeds::<i32>(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

fn benchmark_integer_from_signed_limb_library_comparison(
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
        &(|&i| usize::exact_from(i.significant_bits())),
        "i.significant_bits()",
        &mut [
            ("malachite", &mut (|i| no_out!(Integer::from(i)))),
            ("num", &mut (|i| no_out!(BigInt::from(i)))),
            ("rug", &mut (|i| no_out!(rug::Integer::from(i)))),
        ],
    );
}
