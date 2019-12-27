use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, NegModPowerOfTwo, NegModPowerOfTwoAssign, RemPowerOfTwo,
    RemPowerOfTwoAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::mod_power_of_two::{
    limbs_mod_power_of_two, limbs_mod_power_of_two_in_place, limbs_neg_mod_power_of_two,
    limbs_neg_mod_power_of_two_in_place,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_unsigned;
use inputs::natural::pairs_of_natural_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_power_of_two);
    register_demo!(registry, demo_limbs_mod_power_of_two_in_place);
    register_demo!(registry, demo_limbs_neg_mod_power_of_two);
    register_demo!(registry, demo_limbs_neg_mod_power_of_two_in_place);
    register_demo!(registry, demo_natural_mod_power_of_two_assign);
    register_demo!(registry, demo_natural_mod_power_of_two);
    register_demo!(registry, demo_natural_mod_power_of_two_ref);
    register_demo!(registry, demo_natural_rem_power_of_two_assign);
    register_demo!(registry, demo_natural_rem_power_of_two);
    register_demo!(registry, demo_natural_rem_power_of_two_ref);
    register_demo!(registry, demo_natural_neg_mod_power_of_two_assign);
    register_demo!(registry, demo_natural_neg_mod_power_of_two);
    register_demo!(registry, demo_natural_neg_mod_power_of_two_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_mod_power_of_two_in_place);
    register_bench!(registry, Small, benchmark_limbs_neg_mod_power_of_two);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_neg_mod_power_of_two_in_place
    );
    register_bench!(registry, Large, benchmark_natural_rem_power_of_two_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_power_of_two_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_power_of_two_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_power_of_two_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_power_of_two_evaluation_strategy
    );
}

fn demo_limbs_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_mod_power_of_two({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_mod_power_of_two(&limbs, pow)
        );
    }
}

fn demo_limbs_mod_power_of_two_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_mod_power_of_two_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_mod_power_of_two_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_limbs_neg_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_neg_mod_power_of_two({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_neg_mod_power_of_two(&limbs, pow)
        );
    }
}

fn demo_limbs_neg_mod_power_of_two_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_neg_mod_power_of_two_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_neg_mod_power_of_two_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_natural_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_two_assign(u);
        println!(
            "x := {}; x.mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_natural_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_two({}) = {}",
            n_old,
            u,
            n.mod_power_of_two(u)
        );
    }
}

fn demo_natural_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).mod_power_of_two({}) = {}",
            n,
            u,
            (&n).mod_power_of_two(u)
        );
    }
}

fn demo_natural_rem_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_two_assign(u);
        println!(
            "x := {}; x.rem_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_natural_rem_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.rem_power_of_two({}) = {}",
            n_old,
            u,
            n.rem_power_of_two(u)
        );
    }
}

fn demo_natural_rem_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).rem_power_of_two({}) = {}",
            n,
            u,
            (&n).rem_power_of_two(u)
        );
    }
}

fn demo_natural_neg_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_power_of_two_assign(u);
        println!(
            "x := {}; x.neg_mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_natural_neg_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.neg_mod_power_of_two({}) = {}",
            n_old,
            u,
            n.neg_mod_power_of_two(u)
        );
    }
}

fn demo_natural_neg_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).neg_mod_power_of_two({}) = {}",
            n,
            u,
            (&n).neg_mod_power_of_two(u)
        );
    }
}

fn benchmark_limbs_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_power_of_two(&[u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_mod_power_of_two(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_mod_power_of_two_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_power_of_two_in_place(&mut Vec<u32>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_mod_power_of_two_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_limbs_neg_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_mod_power_of_two(&[u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_neg_mod_power_of_two(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_neg_mod_power_of_two_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_neg_mod_power_of_two_in_place(&mut Vec<u32>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_neg_mod_power_of_two_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_natural_mod_power_of_two_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.mod_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_natural_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.mod_power_of_two(u))),
            ),
            (
                "(&Natural).mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).mod_power_of_two(u))),
            ),
        ],
    );
}

fn benchmark_natural_rem_power_of_two_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.rem_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.rem_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_natural_rem_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.rem_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.rem_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.rem_power_of_two(u))),
            ),
            (
                "(&Natural).rem_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).rem_power_of_two(u))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.neg_mod_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_natural_neg_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "other",
        &mut [
            (
                "Natural.neg_mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.neg_mod_power_of_two(u))),
            ),
            (
                "(&Natural).neg_mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).neg_mod_power_of_two(u))),
            ),
        ],
    );
}
