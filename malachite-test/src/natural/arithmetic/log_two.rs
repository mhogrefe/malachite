use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::positive_naturals;
use malachite_base::num::{CeilingLogTwo, FloorLogTwo, SignificantBits};
use malachite_nz::natural::arithmetic::log_two::{limbs_ceiling_log_two, limbs_floor_log_two};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_floor_log_two);
    register_demo!(registry, demo_limbs_ceiling_log_two);
    register_demo!(registry, demo_natural_floor_log_two);
    register_demo!(registry, demo_natural_ceiling_log_two);
    register_bench!(registry, Small, benchmark_limbs_floor_log_two);
    register_bench!(registry, Small, benchmark_limbs_ceiling_log_two);
    register_bench!(registry, Large, benchmark_natural_floor_log_two);
    register_bench!(registry, Large, benchmark_natural_ceiling_log_two);
}

fn demo_limbs_floor_log_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_floor_log_two({:?}) = {}",
            limbs,
            limbs_floor_log_two(&limbs)
        );
    }
}

fn demo_limbs_ceiling_log_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_ceiling_log_two({:?}) = {}",
            limbs,
            limbs_ceiling_log_two(&limbs)
        );
    }
}

fn demo_natural_floor_log_two(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("floor_log_two({}) = {}", n, n.floor_log_two());
    }
}

fn demo_natural_ceiling_log_two(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("ceiling_log_two({}) = {}", n, n.ceiling_log_two());
    }
}

fn benchmark_limbs_floor_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_floor_log_two(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|ref limbs| no_out!(limbs_floor_log_two(limbs))),
            ),
        ],
    );
}

fn benchmark_limbs_ceiling_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_ceiling_log_two(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|ref limbs| no_out!(limbs_ceiling_log_two(limbs))),
            ),
        ],
    );
}

fn benchmark_natural_floor_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.floor_log_two()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.floor_log_two())))],
    );
}

fn benchmark_natural_ceiling_log_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.ceiling_log_two()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.ceiling_log_two())))],
    );
}
