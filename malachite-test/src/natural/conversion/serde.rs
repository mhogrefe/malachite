extern crate serde;
extern crate serde_json;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_serialize_json);
    register_bench!(registry, Large, benchmark_natural_serialize_json);
}

//TODO demo and bench deserialization
fn demo_natural_serialize_json(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            n,
            serde_json::to_string(&n).unwrap()
        );
    }
}

fn benchmark_natural_serialize_json(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "serde_json::to_string(&Natural)",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|n| no_out!(serde_json::to_string(&n).unwrap())),
        )],
    );
}
