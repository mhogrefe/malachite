use malachite_base::strings::string_nub;

use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::{ascii_strings, strings};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_string_nub);
    register_ns_demo!(registry, demo_ascii_string_nub);
    register_ns_bench!(registry, None, benchmark_string_nub);
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
    m_run_benchmark(
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
