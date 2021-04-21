use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2, NegModPowerOf2Assign, RemPowerOf2,
    RemPowerOf2Assign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mod_power_of_2::{
    limbs_mod_power_of_2, limbs_neg_mod_power_of_2, limbs_neg_mod_power_of_2_in_place,
    limbs_slice_mod_power_of_2_in_place, limbs_vec_mod_power_of_2_in_place,
};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_power_of_2);
    register_demo!(registry, demo_limbs_slice_mod_power_of_2_in_place);
    register_demo!(registry, demo_limbs_vec_mod_power_of_2_in_place);
    register_demo!(registry, demo_limbs_neg_mod_power_of_2);
    register_demo!(registry, demo_limbs_neg_mod_power_of_2_in_place);
    register_demo!(registry, demo_natural_mod_power_of_2_assign);
    register_demo!(registry, demo_natural_mod_power_of_2);
    register_demo!(registry, demo_natural_mod_power_of_2_ref);
    register_demo!(registry, demo_natural_rem_power_of_2_assign);
    register_demo!(registry, demo_natural_rem_power_of_2);
    register_demo!(registry, demo_natural_rem_power_of_2_ref);
    register_demo!(registry, demo_natural_neg_mod_power_of_2_assign);
    register_demo!(registry, demo_natural_neg_mod_power_of_2);
    register_demo!(registry, demo_natural_neg_mod_power_of_2_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_2);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_mod_power_of_2_in_place
    );
    register_bench!(registry, Small, benchmark_limbs_vec_mod_power_of_2_in_place);
    register_bench!(registry, Small, benchmark_limbs_neg_mod_power_of_2);
    register_bench!(registry, Small, benchmark_limbs_neg_mod_power_of_2_in_place);
    register_bench!(registry, Large, benchmark_natural_rem_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_power_of_2_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_power_of_2_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_mod_power_of_2({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_mod_power_of_2(&limbs, pow)
        );
    }
}

fn demo_limbs_slice_mod_power_of_2_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned::<Limb, u64>(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_slice_mod_power_of_2_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_slice_mod_power_of_2_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_limbs_vec_mod_power_of_2_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_mod_power_of_2_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_vec_mod_power_of_2_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_limbs_neg_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_neg_mod_power_of_2({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_neg_mod_power_of_2(&limbs, pow)
        );
    }
}

fn demo_limbs_neg_mod_power_of_2_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_neg_mod_power_of_2_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_neg_mod_power_of_2_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_natural_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

fn demo_natural_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).mod_power_of_2({}) = {}",
            n,
            u,
            (&n).mod_power_of_2(u)
        );
    }
}

fn demo_natural_rem_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {}; x.rem_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_rem_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

fn demo_natural_rem_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).rem_power_of_2({}) = {}",
            n,
            u,
            (&n).rem_power_of_2(u)
        );
    }
}

fn demo_natural_neg_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.neg_mod_power_of_2_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_natural_neg_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.neg_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.neg_mod_power_of_2(u)
        );
    }
}

fn demo_natural_neg_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).neg_mod_power_of_2({}) = {}",
            n,
            u,
            (&n).neg_mod_power_of_2(u)
        );
    }
}

fn benchmark_limbs_mod_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits)| no_out!(limbs_mod_power_of_2(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_slice_mod_power_of_2_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_slice_mod_power_of_2_in_place(&mut Vec<T>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| limbs_slice_mod_power_of_2_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_vec_mod_power_of_2_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_mod_power_of_2_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| limbs_vec_mod_power_of_2_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_neg_mod_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_mod_power_of_2(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(limbs, bits)| no_out!(limbs_neg_mod_power_of_2(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_neg_mod_power_of_2_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_neg_mod_power_of_2_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, bits)| limbs_neg_mod_power_of_2_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [("Malachite", &mut (|(mut n, u)| n.mod_power_of_2_assign(u)))],
    );
}

fn benchmark_natural_mod_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.mod_power_of_2(u))),
            ),
            (
                "(&Natural).mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).mod_power_of_2(u))),
            ),
        ],
    );
}

fn benchmark_natural_rem_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.rem_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [("Malachite", &mut (|(mut n, u)| n.rem_power_of_2_assign(u)))],
    );
}

fn benchmark_natural_rem_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.rem_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.rem_power_of_2(u))),
            ),
            (
                "(&Natural).rem_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).rem_power_of_2(u))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.neg_mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [(
            "Malachite",
            &mut (|(mut n, u)| n.neg_mod_power_of_2_assign(u)),
        )],
    );
}

fn benchmark_natural_neg_mod_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.neg_mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.neg_mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.neg_mod_power_of_2(u))),
            ),
            (
                "(&Natural).neg_mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).neg_mod_power_of_2(u))),
            ),
        ],
    );
}
