use malachite_base::strings::string_unique;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{ascii_strings, strings};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_string_unique);
    register_ns_demo!(registry, demo_ascii_string_unique);
    register_ns_bench!(registry, Large, benchmark_string_unique);
}

fn demo_string_unique(gm: NoSpecialGenerationMode, limit: usize) {
    for s in strings(gm).take(limit) {
        println!("string_unique({:?}) = {:?}", s, string_unique(&s));
    }
}

fn demo_ascii_string_unique(gm: NoSpecialGenerationMode, limit: usize) {
    for s in ascii_strings(gm).take(limit) {
        println!("string_unique({:?}) = {:?}", s, string_unique(&s));
    }
}

fn benchmark_string_unique(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "string_unique(&str)",
        BenchmarkType::Single,
        strings(gm),
        gm.name(),
        limit,
        file_name,
        &(|s| s.len()),
        "s.len()",
        &mut [("Malachite", &mut (|s| no_out!(string_unique(&s))))],
    );
}
