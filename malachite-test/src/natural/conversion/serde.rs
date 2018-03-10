extern crate serde;
extern crate serde_json;

use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

//TODO demo and bench deserialization
pub fn demo_natural_serialize(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            n,
            serde_json::to_string(&n).unwrap()
        );
    }
}

pub fn benchmark_natural_serialize(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "serde_json::to_string(&Natural)",
        BenchmarkType::Ordinary,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            (
                "malachite",
                &mut (|n| no_out!(serde_json::to_string(&n).unwrap())),
            ),
        ],
    );
}
