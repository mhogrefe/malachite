use malachite_base::chars::{char_to_contiguous_range, decrement_char, increment_char};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, NoSpecialGenerationMode, ScaleType};
use malachite_test::inputs::base::{chars_not_max, chars_not_min};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_increment_char);
    register_ns_demo!(registry, demo_decrement_char);
    register_ns_bench!(registry, None, benchmark_increment_char);
    register_ns_bench!(registry, None, benchmark_decrement_char);
}

fn demo_increment_char(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_max(gm).take(limit) {
        let c_old = c;
        increment_char(&mut c);
        println!("c := {:?}; increment_char(&mut c); c = {:?}", c_old, c);
    }
}

fn demo_decrement_char(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_min(gm).take(limit) {
        let c_old = c;
        decrement_char(&mut c);
        println!("c := {:?}; decrement_char(&mut c); c = {:?}", c_old, c);
    }
}

fn benchmark_increment_char(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "increment_char(&mut char)",
        BenchmarkType::Single,
        chars_not_max(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|mut c| increment_char(&mut c)))],
    );
}

fn benchmark_decrement_char(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "increment_char(&mut char)",
        BenchmarkType::Single,
        chars_not_min(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|mut c| decrement_char(&mut c)))],
    );
}
