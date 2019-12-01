use std::cmp::max;

use malachite_nz::natural::arithmetic::eq_mod::{
    _limbs_eq_limb_mod_naive, _limbs_eq_mod_limb_naive, _limbs_eq_mod_naive, limbs_eq_limb_mod,
    limbs_eq_mod, limbs_eq_mod_limb,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2, triples_of_unsigned_vec_var_55,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod);
    register_demo!(registry, demo_limbs_eq_mod_limb);
    register_demo!(registry, demo_limbs_eq_mod);
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_algorithms);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_limb_algorithms);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_algorithms);
}

fn demo_limbs_eq_limb_mod(gm: GenerationMode, limit: usize) {
    for (xs, y, modulus) in triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod({:?}, {}, {:?}) = {}",
            xs,
            y,
            modulus,
            limbs_eq_limb_mod(&xs, y, &modulus)
        );
    }
}

fn demo_limbs_eq_mod_limb(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_limb({:?}, {:?}, {}) = {}",
            xs,
            ys,
            modulus,
            limbs_eq_mod_limb(&xs, &ys, modulus)
        );
    }
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

fn benchmark_limbs_eq_limb_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_limb_mod(&[Limb], Limb, &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(ref xs, y, ref modulus)| no_out!(limbs_eq_limb_mod(xs, y, modulus))),
            ),
            (
                "naive",
                &mut (|(ref xs, y, ref modulus)| no_out!(_limbs_eq_limb_mod_naive(xs, y, modulus))),
            ),
        ],
    );
}

fn benchmark_limbs_eq_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "standard",
                &mut (|(ref xs, ref ys, modulus)| no_out!(limbs_eq_mod_limb(xs, ys, modulus))),
            ),
            (
                "naive",
                &mut (|(ref xs, ref ys, modulus)| {
                    no_out!(_limbs_eq_mod_limb_naive(xs, ys, modulus))
                }),
            ),
        ],
    );
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
                "standard",
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
