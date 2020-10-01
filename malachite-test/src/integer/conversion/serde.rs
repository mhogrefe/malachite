extern crate serde;
extern crate serde_json;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
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
    run_benchmark_old(
        "serde_json::to_string(&Natural)",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Malachite",
            &mut (|n| no_out!(serde_json::to_string(&n).unwrap())),
        )],
    );
}
