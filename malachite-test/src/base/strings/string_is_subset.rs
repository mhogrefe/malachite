use malachite_base::strings::string_is_subset;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{pairs_of_ascii_strings, pairs_of_strings};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_string_is_subset);
    register_ns_demo!(registry, demo_ascii_string_is_subset);
    register_ns_bench!(registry, Large, benchmark_string_is_subset);
}

fn demo_string_is_subset(gm: NoSpecialGenerationMode, limit: usize) {
    for (s, t) in pairs_of_strings(gm).take(limit) {
        println!(
            "{:?} is {}a subset of {:?}",
            s,
            if string_is_subset(&s, &t) { "" } else { "not " },
            t
        );
    }
}

fn demo_ascii_string_is_subset(gm: NoSpecialGenerationMode, limit: usize) {
    for (s, t) in pairs_of_ascii_strings(gm).take(limit) {
        println!(
            "{:?} is {}a subset of {:?}",
            s,
            if string_is_subset(&s, &t) { "" } else { "not " },
            t
        );
    }
}

fn benchmark_string_is_subset(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "string_is_subset(&str, &str)",
        BenchmarkType::Single,
        pairs_of_strings(gm),
        gm.name(),
        limit,
        file_name,
        &(|(s, t)| s.len() + t.len()),
        "s.len() + t.len()",
        &mut [(
            "malachite",
            &mut (|(s, t)| no_out!(string_is_subset(&s, &t))),
        )],
    );
}
