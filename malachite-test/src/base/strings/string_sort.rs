use malachite_base::strings::string_sort;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{ascii_strings, strings};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_string_sort);
    register_ns_demo!(registry, demo_ascii_string_sort);
    register_ns_bench!(registry, Large, benchmark_string_sort);
}

fn demo_string_sort(gm: NoSpecialGenerationMode, limit: usize) {
    for s in strings(gm).take(limit) {
        println!("string_sort({:?}) = {:?}", s, string_sort(&s));
    }
}

fn demo_ascii_string_sort(gm: NoSpecialGenerationMode, limit: usize) {
    for s in ascii_strings(gm).take(limit) {
        println!("string_sort({:?}) = {:?}", s, string_sort(&s));
    }
}

fn benchmark_string_sort(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "string_sort(&str)",
        BenchmarkType::Single,
        strings(gm),
        gm.name(),
        limit,
        file_name,
        &(|s| s.len()),
        "s.len()",
        &mut [("Malachite", &mut (|s| no_out!(string_sort(&s))))],
    );
}
