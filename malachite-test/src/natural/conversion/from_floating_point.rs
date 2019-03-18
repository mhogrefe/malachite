use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{f32s_var_1, f64s_var_1};
use malachite_base::num::PrimitiveFloat;
use malachite_nz::natural::Natural;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_f32);
    register_demo!(registry, demo_natural_from_f64);
    register_bench!(registry, None, benchmark_natural_from_f32);
    register_bench!(registry, None, benchmark_natural_from_f64);
}

fn demo_natural_from_f32(gm: GenerationMode, limit: usize) {
    for f in f32s_var_1(gm).take(limit) {
        println!("from({:?}) = {}", f, Natural::from(f));
    }
}

fn demo_natural_from_f64(gm: GenerationMode, limit: usize) {
    for f in f64s_var_1(gm).take(limit) {
        println!("from({:?}) = {}", f, Natural::from(f));
    }
}

fn benchmark_natural_from_f32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from(f32)",
        BenchmarkType::Single,
        f32s_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&f| f.adjusted_exponent() as usize),
        "f.adjusted_exponent()",
        &mut [("malachite", &mut (|f| no_out!(Natural::from(f))))],
    );
}

fn benchmark_natural_from_f64(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from(f64)",
        BenchmarkType::Single,
        f64s_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&f| f.adjusted_exponent() as usize),
        "f.adjusted_exponent()",
        &mut [("malachite", &mut (|f| no_out!(Natural::from(f))))],
    );
}
