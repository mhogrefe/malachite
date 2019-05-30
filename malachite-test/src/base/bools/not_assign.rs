use malachite_base::num::logic::traits::NotAssign;

use common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, NoSpecialGenerationMode, ScaleType,
};
use inputs::base::bools;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_bool_not_assign);
    register_ns_bench!(registry, None, benchmark_bool_not_assign);
}

fn demo_bool_not_assign(gm: NoSpecialGenerationMode, limit: usize) {
    for mut b in bools(gm).take(limit) {
        let b_old = b;
        b.not_assign();
        println!("b := {:?}; b.not_assign(); b = {:?}", b_old, b);
    }
}

fn benchmark_bool_not_assign(gm: NoSpecialGenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "bool.not_assign()",
        BenchmarkType::Single,
        bools(gm),
        gm.name(),
        limit,
        file_name,
        &(|&b| if b { 1 } else { 0 }),
        "if b { 1 } else { 0 }",
        &mut [("malachite", &mut (|mut b| b.not_assign()))],
    );
}
