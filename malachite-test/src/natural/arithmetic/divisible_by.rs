use malachite_base::limbs::limbs_test_zero;
use malachite_nz::natural::arithmetic::divisible_by::limbs_divisible_by;
use malachite_nz::natural::arithmetic::mod_op::limbs_mod;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_unsigned_vec_var_13, pairs_of_unsigned_vec_var_14};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_divisible_by);
    register_bench!(registry, Small, benchmark_limbs_divisible_by_algorithms);
}

fn demo_limbs_divisible_by(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_13(gm).take(limit) {
        println!(
            "limbs_divisible_by({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by(&ns, &ds)
        );
    }
}

fn benchmark_limbs_divisible_by_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_divisible_by(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_14(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_divisible_by",
                &mut (|(ref ns, ref ds)| no_out!(limbs_divisible_by(ns, ds))),
            ),
            (
                "divisibility using limbs_mod",
                &mut (|(ref ns, ref ds)| no_out!(limbs_test_zero(&limbs_mod(ns, ds)))),
            ),
        ],
    );
}
