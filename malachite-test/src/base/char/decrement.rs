use common::{m_run_benchmark, BenchmarkType, NoSpecialGenerationMode};
use inputs::base::chars_not_min;
use malachite_base::chars::char_to_contiguous_range;
use malachite_base::misc::Walkable;

pub fn demo_char_decrement(gm: NoSpecialGenerationMode, limit: usize) {
    for mut c in chars_not_min(gm).take(limit) {
        let c_old = c;
        c.decrement();
        println!("c := {:?}; c.decrement(); c = {:?}", c_old, c);
    }
}

pub fn benchmark_char_decrement(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "char.decrement()",
        BenchmarkType::Single,
        chars_not_min(gm),
        gm.name(),
        limit,
        file_name,
        &(|&c| char_to_contiguous_range(c) as usize),
        "char_to_contiguous_range(char)",
        &[("malachite", &mut (|mut c| c.decrement()))],
    );
}
