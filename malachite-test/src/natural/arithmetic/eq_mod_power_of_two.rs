use malachite_base::num::arithmetic::traits::{EqModPowerOfTwo, ModPowerOfTwo};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::eq_mod_power_of_two::{
    limbs_eq_limb_mod_power_of_two, limbs_eq_mod_power_of_two,
};

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    rm_triples_of_natural_natural_and_small_unsigned, triples_of_natural_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod_power_of_two);
    register_demo!(registry, demo_limbs_eq_mod_power_of_two);
    register_demo!(registry, demo_natural_eq_mod_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_mod_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_mod_power_of_two_algorithms
    );
}

fn demo_limbs_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, limb, pow) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod_power_of_two({:?}, {}, {}) = {:?}",
            limbs,
            limb,
            pow,
            limbs_eq_limb_mod_power_of_two(&limbs, limb, pow)
        );
    }
}

fn demo_limbs_eq_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys, pow) in
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_two({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_eq_mod_power_of_two(xs, ys, pow)
        );
    }
}

fn demo_natural_eq_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (ref x, ref y, pow) in triples_of_natural_natural_and_small_unsigned(gm).take(limit) {
        if x.eq_mod_power_of_two(y, pow) {
            println!("{} is equal to {} mod 2^{}", x, y, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", x, y, pow);
        }
    }
}

fn benchmark_limbs_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_limb_mod_power_of_two(&[Limb], u64)",
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
                no_out!(limbs_eq_limb_mod_power_of_two(limbs, limb, pow))
            }),
        )],
    );
}

fn benchmark_limbs_eq_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_power_of_two(&[u32], &[u32], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, pow)| min!(usize::exact_from(pow), xs.len(), ys.len())),
        "min(pow, xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_eq_mod_power_of_two(xs, ys, pow))),
        )],
    );
}

fn benchmark_natural_eq_mod_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&Natural, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_natural_natural_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (ref n, ref u, pow))| no_out!(n.eq_mod_power_of_two(u, pow))),
            ),
            (
                "rug",
                &mut (|((ref n, ref u, pow), _)| {
                    no_out!(n.is_congruent_2pow(u, u32::exact_from(pow)))
                }),
            ),
        ],
    );
}

fn benchmark_natural_eq_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, pow)| {
            usize::exact_from(min!(pow, x.significant_bits(), y.significant_bits()))
        }),
        "min(pow, x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.eq_mod_power_of_two(&Natural, u64)",
                &mut (|(ref x, ref y, pow)| no_out!(x.eq_mod_power_of_two(y, pow))),
            ),
            (
                "Natural.mod_power_of_two(u64) == Natural.mod_power_of_two(u64)",
                &mut (|(ref x, ref y, pow)| {
                    no_out!(x.mod_power_of_two(pow) == y.mod_power_of_two(pow))
                }),
            ),
        ],
    );
}
