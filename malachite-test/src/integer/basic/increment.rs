use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;

pub fn demo_integer_increment(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_integer_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.increment()",
        BenchmarkType::Ordinary,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &[("malachite", &mut (|mut n| n.increment()))],
    );
}
