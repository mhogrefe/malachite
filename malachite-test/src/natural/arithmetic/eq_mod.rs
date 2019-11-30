use std::cmp::max;

use malachite_nz::natural::arithmetic::eq_mod::{_limbs_eq_mod_naive, limbs_eq_mod};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_var_55;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_mod);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_algorithms);
}

fn demo_limbs_eq_mod(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        println!(
            "limbs_eq_mod({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            modulus,
            limbs_eq_mod(&xs, &ys, &modulus)
        );
    }
}

fn benchmark_limbs_eq_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_55(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "limbs_eq_mod",
                &mut (|(ref xs, ref ys, ref modulus)| no_out!(limbs_eq_mod(xs, ys, modulus))),
            ),
            (
                "naive",
                &mut (|(ref xs, ref ys, ref modulus)| {
                    no_out!(_limbs_eq_mod_naive(xs, ys, modulus))
                }),
            ),
        ],
    );
}
