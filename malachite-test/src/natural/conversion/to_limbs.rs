use itertools::Itertools;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_to_limbs_asc);
    register_demo!(registry, demo_natural_to_limbs_desc);
    register_demo!(registry, demo_natural_into_limbs_asc);
    register_demo!(registry, demo_natural_into_limbs_desc);
    register_demo!(registry, demo_natural_limbs);
    register_demo!(registry, demo_natural_limbs_rev);
    register_demo!(registry, demo_natural_limbs_size_hint);
    register_demo!(registry, demo_natural_limbs_index);
    register_bench!(registry, Large, benchmark_natural_limbs_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_natural_limbs_rev_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_limbs_size_hint);
    register_bench!(registry, Large, benchmark_natural_limbs_index_algorithms);
}

fn demo_natural_to_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_asc({}) = {:?}", n, n.to_limbs_asc());
    }
}

fn demo_natural_to_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_desc({}) = {:?}", n, n.to_limbs_desc());
    }
}

fn demo_natural_into_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_asc({}) = {:?}", n, n.clone().into_limbs_asc());
    }
}

fn demo_natural_into_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("into_limbs_desc({}) = {:?}", n, n.clone().into_limbs_desc());
    }
}

fn demo_natural_limbs(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}) = {:?}", n, n.limbs().collect_vec());
    }
}

fn demo_natural_limbs_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}).rev() = {:?}", n, n.limbs().rev().collect_vec());
    }
}

fn demo_natural_limbs_size_hint(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("limbs({}).size_hint() = {:?}", n, n.limbs().size_hint());
    }
}

fn demo_natural_limbs_index(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!("limbs({})[{}] = {:?}", n, i, n.limbs()[i]);
    }
}

#[allow(unknown_lints, unused_collect)]
fn benchmark_natural_limbs_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.limbs()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.to_limbs_asc()",
                &mut (|n| no_out!(n.to_limbs_asc())),
            ),
            (
                "Natural.into_limbs_asc()",
                &mut (|n| no_out!(n.into_limbs_asc())),
            ),
            (
                "Natural.limbs().collect_vec()",
                &mut (|n| no_out!(n.limbs().collect_vec())),
            ),
        ],
    );
}

#[allow(unknown_lints, unused_collect)]
fn benchmark_natural_limbs_rev_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.limbs().rev()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.to_limbs_desc()",
                &mut (|n| no_out!(n.to_limbs_desc())),
            ),
            (
                "Natural.into_limbs_desc()",
                &mut (|n| no_out!(n.into_limbs_desc())),
            ),
            (
                "Natural.limbs().rev().collect_vec()",
                &mut (|n| no_out!(n.limbs().rev().collect_vec())),
            ),
        ],
    );
}

fn benchmark_natural_limbs_size_hint(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.limbs().size_hint()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Natural.limbs().size_hint()",
            &mut (|n| no_out!(n.limbs().size_hint())),
        )],
    );
}

fn benchmark_natural_limbs_index_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.limbs().index()",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Natural.limbs()[u]", &mut (|(n, u)| no_out!(n.limbs()[u]))),
            (
                "Natural.into_limbs_asc()[u]",
                &mut (|(n, u)| {
                    let limbs = n.into_limbs_asc();
                    if u >= limbs.len() {
                        0
                    } else {
                        limbs[u]
                    };
                }),
            ),
        ],
    );
}
