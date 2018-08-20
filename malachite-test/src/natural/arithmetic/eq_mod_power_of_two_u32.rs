use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1;
use inputs::natural::triples_of_natural_unsigned_and_small_unsigned;
use malachite_base::num::{EqModPowerOfTwo, ModPowerOfTwo, SignificantBits};
use malachite_nz::natural::arithmetic::eq_mod_power_of_two_u32::limbs_eq_mod_power_of_two_limb;
use std::cmp::min;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_mod_power_of_two_limb);
    register_demo!(registry, demo_natural_eq_mod_power_of_two_u32);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_power_of_two_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_mod_power_of_two_u32_algorithms
    );
}

fn demo_limbs_eq_mod_power_of_two_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, pow) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_two_limb({:?}, {}, {}) = {:?}",
            limbs,
            limb,
            pow,
            limbs_eq_mod_power_of_two_limb(&limbs, limb, pow)
        );
    }
}

fn demo_natural_eq_mod_power_of_two_u32(gm: GenerationMode, limit: usize) {
    for (n, u, pow) in triples_of_natural_unsigned_and_small_unsigned::<u32, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            n,
            u,
            pow,
            n.eq_mod_power_of_two(&u, pow)
        );
    }
}

fn benchmark_limbs_eq_mod_power_of_two_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_power_of_two_limb(&[u32], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb, pow)| {
                no_out!(limbs_eq_mod_power_of_two_limb(limbs, limb, pow))
            }),
        )],
    );
}

fn benchmark_natural_eq_mod_power_of_two_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&u32, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_unsigned_and_small_unsigned::<u32, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, pow)| min(pow, n.significant_bits()) as usize),
        "min(pow, n.significant_bits())",
        &mut [
            (
                "Natural.eq_mod_power_of_two(&u32, u64)",
                &mut (|(n, u, pow)| no_out!(n.eq_mod_power_of_two(&u, pow))),
            ),
            (
                "Natural.mod_power_of_two(u64) == u32.mod_power_of_two(u64)",
                &mut (|(n, u, pow)| no_out!(n.mod_power_of_two(pow) == u.mod_power_of_two(pow))),
            ),
        ],
    );
}
