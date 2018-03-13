use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;

pub fn demo_integer_decrement(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_integer_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.decrement()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|mut n| n.decrement()))],
    );
}
