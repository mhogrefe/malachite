use std::cmp::{max, min};

use malachite_base::num::arithmetic::traits::{EqModPowerOf2, ModPowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::integer::arithmetic::eq_mod_power_of_2::{
    limbs_eq_mod_power_of_2_neg_limb, limbs_eq_mod_power_of_2_neg_pos,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::{
    rm_triples_of_integer_integer_and_small_unsigned, triples_of_integer_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_mod_power_of_2_neg_limb);
    register_demo!(registry, demo_limbs_eq_mod_power_of_2_neg_pos);
    register_demo!(registry, demo_integer_eq_mod_power_of_2);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_power_of_2_neg_limb);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_power_of_2_neg_pos);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_mod_power_of_2_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_mod_power_of_2_algorithms
    );
}

fn demo_limbs_eq_mod_power_of_2_neg_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, pow) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_2_neg_limb({:?}, {}, {}) = {:?}",
            limbs,
            limb,
            pow,
            limbs_eq_mod_power_of_2_neg_limb(&limbs, limb, pow)
        );
    }
}

fn demo_limbs_eq_mod_power_of_2_neg_pos(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys, pow) in
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_2_neg_pos({:?}, {:?}, {}) = {:?}",
            xs,
            ys,
            pow,
            limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow)
        );
    }
}

fn demo_integer_eq_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (ref x, ref y, pow) in triples_of_integer_integer_and_small_unsigned(gm).take(limit) {
        if x.eq_mod_power_of_2(y, pow) {
            println!("{} is equal to {} mod 2^{}", x, y, pow);
        } else {
            println!("{} is not equal to {} mod 2^{}", x, y, pow);
        }
    }
}

fn benchmark_limbs_eq_mod_power_of_2_neg_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_eq_mod_power_of_2_neg_limb(&[Limb], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref limbs, limb, pow)| {
                no_out!(limbs_eq_mod_power_of_2_neg_limb(limbs, limb, pow))
            }),
        )],
    );
}

fn benchmark_limbs_eq_mod_power_of_2_neg_pos(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_eq_mod_power_of_2_neg_pos(&[u32], &[u32], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, pow)| min(usize::exact_from(pow), max(xs.len(), ys.len()))),
        "min(pow, max(xs.len(), ys.len()))",
        &mut [(
            "Malachite",
            &mut (|(ref xs, ref ys, pow)| no_out!(limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow))),
        )],
    );
}

fn benchmark_integer_eq_mod_power_of_2_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.eq_mod_power_of_2(&Integer, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_integer_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Malachite",
                &mut (|(_, (ref n, ref u, pow))| no_out!(n.eq_mod_power_of_2(u, pow))),
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

fn benchmark_integer_eq_mod_power_of_2_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.eq_mod_power_of_2(&Integer, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_small_unsigned::<u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, pow)| {
            usize::exact_from(min(pow, max(x.significant_bits(), y.significant_bits())))
        }),
        "min(pow, max(x.significant_bits(), y.significant_bits()))",
        &mut [
            (
                "Integer.eq_mod_power_of_2(&Integer, u64)",
                &mut (|(ref x, ref y, pow)| no_out!(x.eq_mod_power_of_2(y, pow))),
            ),
            (
                "Integer.mod_power_of_2(u64) == Integer.mod_power_of_2(u64)",
                &mut (|(ref x, ref y, pow)| {
                    no_out!(x.mod_power_of_2(pow) == y.mod_power_of_2(pow))
                }),
            ),
        ],
    );
}
