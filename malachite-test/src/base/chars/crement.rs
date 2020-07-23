use malachite_base::chars::char_to_contiguous_range;
use malachite_base::crement::Crementable;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{chars_not_max, chars_not_min};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_char_increment);
    register_ns_demo!(registry, demo_char_decrement);
    register_ns_bench!(registry, None, benchmark_char_increment);
    register_ns_bench!(registry, None, benchmark_char_decrement);
}

fn demo_char_increment(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_max(gm).take(limit) {
        let c_old = c;
        c.increment();
        println!("c := {:?}; c.increment(); c = {:?}", c_old, c);
    }
}

fn demo_char_decrement(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_min(gm).take(limit) {
        let c_old = c;
        c.decrement();
        println!("c := {:?}; c.decrement(); c = {:?}", c_old, c);
    }
}

fn benchmark_char_increment(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "char.increment()",
        BenchmarkType::Single,
        chars_not_max(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|mut c| c.increment()))],
    );
}

fn benchmark_char_decrement(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "char.decrement()",
        BenchmarkType::Single,
        chars_not_min(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|mut c| c.decrement()))],
    );
}
