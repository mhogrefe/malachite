use malachite_base::num::arithmetic::traits::{CeilingLogBase2, FloorLogBase2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::log_base_2::{
    limbs_ceiling_log_base_2, limbs_floor_log_base_2,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned_var_1;
use malachite_test::inputs::natural::positive_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_floor_log_base_2);
    register_demo!(registry, demo_limbs_ceiling_log_base_2);
    register_demo!(registry, demo_natural_floor_log_base_2);
    register_demo!(registry, demo_natural_ceiling_log_base_2);
    register_bench!(registry, Small, benchmark_limbs_floor_log_base_2);
    register_bench!(registry, Small, benchmark_limbs_ceiling_log_base_2);
    register_bench!(registry, Large, benchmark_natural_floor_log_base_2);
    register_bench!(registry, Large, benchmark_natural_ceiling_log_base_2);
}

fn demo_limbs_floor_log_base_2(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_floor_log_base_2({:?}) = {}",
            limbs,
            limbs_floor_log_base_2(&limbs)
        );
    }
}

fn demo_limbs_ceiling_log_base_2(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_ceiling_log_base_2({:?}) = {}",
            limbs,
            limbs_ceiling_log_base_2(&limbs)
        );
    }
}

fn demo_natural_floor_log_base_2(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("floor_log_base_2({}) = {}", n, n.floor_log_base_2());
    }
}

fn demo_natural_ceiling_log_base_2(gm: GenerationMode, limit: usize) {
    for n in positive_naturals(gm).take(limit) {
        println!("ceiling_log_base_2({}) = {}", n, n.ceiling_log_base_2());
    }
}

fn benchmark_limbs_floor_log_base_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_floor_log_base_2(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref limbs| no_out!(limbs_floor_log_base_2(limbs))),
        )],
    );
}

fn benchmark_limbs_ceiling_log_base_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_ceiling_log_base_2(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref limbs| no_out!(limbs_ceiling_log_base_2(limbs))),
        )],
    );
}

fn benchmark_natural_floor_log_base_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.floor_log_base_2()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|n| no_out!(n.floor_log_base_2())))],
    );
}

fn benchmark_natural_ceiling_log_base_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.ceiling_log_base_2()",
        BenchmarkType::Single,
        positive_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|n| no_out!(n.ceiling_log_base_2())))],
    );
}
