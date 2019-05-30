use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::unsigneds;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_limb);
    register_bench!(
        registry,
        None,
        benchmark_natural_from_limb_library_comparison
    );
}

fn demo_natural_from_limb(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u32>(gm).take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}

fn benchmark_natural_from_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from(u32)",
        BenchmarkType::LibraryComparison,
        unsigneds::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&u| usize::checked_from(u.significant_bits()).unwrap()),
        "u.significant_bits()",
        &mut [
            ("malachite", &mut (|u| no_out!(Natural::from(u)))),
            ("num", &mut (|u| no_out!(BigUint::from(u)))),
            ("rug", &mut (|u| no_out!(rug::Integer::from(u)))),
        ],
    );
}
