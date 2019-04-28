use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::crement::Crementable;
use malachite_base::num::traits::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_decrement);
    register_bench!(registry, Large, benchmark_integer_decrement);
}

fn demo_integer_decrement(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

fn benchmark_integer_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.decrement()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.decrement()))],
    );
}
