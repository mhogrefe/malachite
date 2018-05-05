use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::chars_not_max;
use malachite_base::chars::char_to_contiguous_range;
use malachite_base::misc::Walkable;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_char_increment);
    register_ns_bench!(registry, None, benchmark_char_increment);
}

fn demo_char_increment(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_max(gm).take(limit) {
        let c_old = c;
        c.increment();
        println!("c := {:?}; c.increment(); c = {:?}", c_old, c);
    }
}

fn benchmark_char_increment(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "char.increment()",
        BenchmarkType::Single,
        chars_not_max(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| char_to_contiguous_range(c) as usize),
        "char_to_contiguous_range(char)",
        &mut [("malachite", &mut (|mut c| c.increment()))],
    );
}
