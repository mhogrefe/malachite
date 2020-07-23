use malachite_base::strings::string_nub;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{ascii_strings, strings};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_string_nub);
    register_ns_demo!(registry, demo_ascii_string_nub);
    register_ns_bench!(registry, Large, benchmark_string_nub);
}

fn demo_string_nub(gm: NoSpecialGenerationMode, limit: usize) {
    for s in strings(gm).take(limit) {
        println!("string_nub({:?}) = {:?}", s, string_nub(&s));
    }
}

fn demo_ascii_string_nub(gm: NoSpecialGenerationMode, limit: usize) {
    for s in ascii_strings(gm).take(limit) {
        println!("string_nub({:?}) = {:?}", s, string_nub(&s));
    }
}

fn benchmark_string_nub(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "string_nub(&str)",
        BenchmarkType::Single,
        strings(gm),
        gm.name(),
        limit,
        file_name,
        &(|s| s.len()),
        "s.len()",
        &mut [("malachite", &mut (|s| no_out!(string_nub(&s))))],
    );
}
