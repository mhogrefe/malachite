extern crate serde;
extern crate serde_json;

use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

//TODO demo and bench deserialization
pub fn demo_integer_serialize(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            n,
            serde_json::to_string(&n).unwrap()
        );
    }
}

pub fn benchmark_integer_serialize(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "serde_json::to_string(&Natural)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|n| no_out!(serde_json::to_string(&n).unwrap())),
            ),
        ],
    );
}
