use std::cmp::max;

use malachite_base::num::arithmetic::traits::{
    PowerOfTwo, RoundToMultiple, RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign,
    ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::round_to_multiple_of_power_of_two::{
    limbs_round_to_multiple_of_power_of_two, limbs_round_to_multiple_of_power_of_two_down,
    limbs_round_to_multiple_of_power_of_two_down_in_place,
    limbs_round_to_multiple_of_power_of_two_in_place,
    limbs_round_to_multiple_of_power_of_two_nearest,
    limbs_round_to_multiple_of_power_of_two_nearest_in_place,
    limbs_round_to_multiple_of_power_of_two_up,
    limbs_round_to_multiple_of_power_of_two_up_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
    triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::triples_of_natural_small_unsigned_and_rounding_mode_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_round_to_multiple_of_power_of_two_down);
    register_demo!(registry, demo_limbs_round_to_multiple_of_power_of_two_up);
    register_demo!(
        registry,
        demo_limbs_round_to_multiple_of_power_of_two_nearest
    );
    register_demo!(registry, demo_limbs_round_to_multiple_of_power_of_two);
    register_demo!(
        registry,
        demo_limbs_round_to_multiple_of_power_of_two_down_in_place
    );
    register_demo!(
        registry,
        demo_limbs_round_to_multiple_of_power_of_two_up_in_place
    );
    register_demo!(
        registry,
        demo_limbs_round_to_multiple_of_power_of_two_nearest_in_place
    );
    register_demo!(
        registry,
        demo_limbs_round_to_multiple_of_power_of_two_in_place
    );
    register_demo!(
        registry,
        demo_natural_round_to_multiple_of_power_of_two_assign
    );
    register_demo!(registry, demo_natural_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_natural_round_to_multiple_of_power_of_two_ref);

    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_down
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_up
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_nearest
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_down_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_up_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_nearest_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_round_to_multiple_of_power_of_two_in_place
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_round_to_multiple_of_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_round_to_multiple_of_power_of_two_evaluation_strategy
    );
}

fn demo_limbs_round_to_multiple_of_power_of_two_down(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_round_to_multiple_of_power_of_two_down({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_round_to_multiple_of_power_of_two_down(&limbs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_up(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_round_to_multiple_of_power_of_two_up({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_round_to_multiple_of_power_of_two_up(&limbs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_nearest(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_round_to_multiple_of_power_of_two_nearest({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_round_to_multiple_of_power_of_two_nearest(&limbs, pow)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, pow, rm) in
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        println!(
            "limbs_round_to_multiple_of_power_of_two({:?}, {}, {}) = {:?}",
            limbs,
            pow,
            rm,
            limbs_round_to_multiple_of_power_of_two(&limbs, pow, rm)
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_down_in_place(gm: GenerationMode, limit: usize) {
    for (mut limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let limbs_old = limbs.clone();
        limbs_round_to_multiple_of_power_of_two_down_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_round_to_multiple_of_power_of_two_down_in_place(&mut limbs, {}); \
            limbs = {:?}",
            limbs_old,
            pow,
            limbs
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_up_in_place(gm: GenerationMode, limit: usize) {
    for (mut limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        let limbs_old = limbs.clone();
        limbs_round_to_multiple_of_power_of_two_up_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; limbs_round_to_multiple_of_power_of_two_up_in_place(&mut limbs, {}); \
            limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_nearest_in_place(gm: GenerationMode, limit: usize) {
    for (mut limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let limbs_old = limbs.clone();
        limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut limbs, pow);
        println!(
            "limbs := {:?}; \
            limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, pow, limbs
        );
    }
}

fn demo_limbs_round_to_multiple_of_power_of_two_in_place(gm: GenerationMode, limit: usize) {
    for (mut limbs, pow, rm) in
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        let limbs_old = limbs.clone();
        let success = limbs_round_to_multiple_of_power_of_two_in_place(&mut limbs, pow, rm);
        println!(
            "limbs := {:?}; \
            limbs_round_to_multiple_of_power_of_two_in_place(&mut limbs, {}, {}) = {}; \
            limbs = {:?}",
            limbs_old, pow, rm, success, limbs
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow, rm) in
        triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        let n_old = n.clone();
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        println!(
            "x := {}; x.round_to_multiple_of_power_of_two_assign({}, {}); x = {}",
            n_old, pow, rm, n
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow, rm) in triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.round_to_multiple_of_power_of_two({}, {}) = {}",
            n_old,
            pow,
            rm,
            n.round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn demo_natural_round_to_multiple_of_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, pow, rm) in triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).round_to_multiple_of_power_of_two({}, {}) = {}",
            n,
            pow,
            rm,
            (&n).round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_down(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_down(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, pow)| {
                no_out!(limbs_round_to_multiple_of_power_of_two_down(&limbs, pow))
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_up(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_up(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, pow)| no_out!(limbs_round_to_multiple_of_power_of_two_up(&limbs, pow))),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_nearest(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_nearest(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, pow)| {
                no_out!(limbs_round_to_multiple_of_power_of_two_nearest(&limbs, pow))
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two(&[Limb], u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow, _)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(limbs, pow, rm)| {
                no_out!(limbs_round_to_multiple_of_power_of_two(&limbs, pow, rm))
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_down_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_down_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, pow)| {
                limbs_round_to_multiple_of_power_of_two_down_in_place(&mut limbs, pow)
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_up_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_up_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, pow)| {
                limbs_round_to_multiple_of_power_of_two_up_in_place(&mut limbs, pow)
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_nearest_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, pow)| {
                limbs_round_to_multiple_of_power_of_two_nearest_in_place(&mut limbs, pow)
            }),
        )],
    );
}

fn benchmark_limbs_round_to_multiple_of_power_of_two_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_round_to_multiple_of_power_of_two_in_place(&mut Vec<Limb>, u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow, _)| max(limbs.len(), usize::exact_from(pow) >> Limb::LOG_WIDTH)),
        "max(limbs.len(), pow / Limb::WIDTH)",
        &mut [(
            "Malachite",
            &mut (|(mut limbs, pow, rm)| {
                no_out!(limbs_round_to_multiple_of_power_of_two_in_place(
                    &mut limbs, pow, rm
                ))
            }),
        )],
    );
}

fn benchmark_natural_round_to_multiple_of_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.round_to_multiple_of_power_of_two_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [(
            "Malachite",
            &mut (|(mut x, y, rm)| x.round_to_multiple_of_power_of_two_assign(y, rm)),
        )],
    );
}

fn benchmark_natural_round_to_multiple_of_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.round_to_multiple_of_power_of_two(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [
            (
                "default",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_two(y, rm))),
            ),
            (
                "using shr_round",
                &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm) << y)),
            ),
            (
                "using round_to_multiple",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple(Natural::power_of_two(y), rm))),
            ),
        ],
    );
}

fn benchmark_natural_round_to_multiple_of_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.round_to_multiple_of_power_of_two(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [
            (
                "Natural.round_to_multiple_of_power_of_two(u64, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_two(y, rm))),
            ),
            (
                "(&Natural).round_to_multiple_of_power_of_two(u64, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).round_to_multiple_of_power_of_two(y, rm))),
            ),
        ],
    );
}
