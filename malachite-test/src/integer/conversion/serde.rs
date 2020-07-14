extern crate serde;
extern crate serde_json;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_serialize_json);
    register_bench!(registry, Large, benchmark_integer_serialize_json);
}

//TODO demo and bench deserialization
fn demo_integer_serialize_json(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            n,
            serde_json::to_string(&n).unwrap()
        );
    }
}

fn benchmark_integer_serialize_json(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "serde_json::to_string(&Natural)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|n| no_out!(serde_json::to_string(&n).unwrap())),
        )],
    );
}
