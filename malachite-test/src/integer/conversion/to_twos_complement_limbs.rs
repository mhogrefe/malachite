use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::integer::conversion::to_twos_complement_limbs::*;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{vecs_of_unsigned, vecs_of_unsigned_var_3};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_unsigned};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_twos_complement);
    register_demo!(registry, demo_limbs_maybe_sign_extend_non_negative_in_place);
    register_demo!(registry, demo_limbs_twos_complement_in_place);
    register_demo!(
        registry,
        demo_limbs_twos_complement_and_maybe_sign_extend_negative_in_place
    );
    register_demo!(registry, demo_integer_to_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_to_twos_complement_limbs_desc);
    register_demo!(registry, demo_integer_into_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_into_twos_complement_limbs_desc);
    register_demo!(registry, demo_integer_twos_complement_limbs);
    register_demo!(registry, demo_integer_twos_complement_limbs_rev);
    register_demo!(registry, demo_integer_twos_complement_limbs_get);
    register_bench!(registry, Small, benchmark_limbs_twos_complement);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_maybe_sign_extend_non_negative_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_twos_complement_in_place_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_twos_complement_and_maybe_sign_extend_negative_in_place
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_twos_complement_limbs_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_twos_complement_limbs_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_twos_complement_limbs_get_algorithms
    );
}

fn demo_limbs_twos_complement(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_3(gm).take(limit) {
        println!(
            "limbs_twos_complement({:?}) = {:?}",
            limbs,
            limbs_twos_complement(&limbs)
        );
    }
}

fn demo_limbs_maybe_sign_extend_non_negative_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_maybe_sign_extend_non_negative_in_place(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_maybe_sign_extend_non_negative_in_place(&mut limbs); \
             limbs = {:?}",
            limbs, mut_limbs
        );
    }
}

fn demo_limbs_twos_complement_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        let carry = limbs_twos_complement_in_place(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_twos_complement_in_place(&mut limbs) = {}; \
             limbs = {:?}",
            limbs, carry, mut_limbs
        );
    }
}

fn demo_limbs_twos_complement_and_maybe_sign_extend_negative_in_place(
    gm: GenerationMode,
    limit: usize,
) {
    for limbs in vecs_of_unsigned_var_3(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_twos_complement_and_maybe_sign_extend_negative_in_place(\
             &mut limbs); limbs = {:?}",
            limbs, mut_limbs
        );
    }
}

fn demo_integer_to_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_twos_complement_limbs_asc({}) = {:?}",
            n,
            n.to_twos_complement_limbs_asc()
        );
    }
}

fn demo_integer_to_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_twos_complement_limbs_desc({}) = {:?}",
            n,
            n.to_twos_complement_limbs_desc()
        );
    }
}

fn demo_integer_into_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_twos_complement_limbs_asc({}) = {:?}",
            n,
            n.clone().into_twos_complement_limbs_asc()
        );
    }
}

fn demo_integer_into_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_twos_complement_limbs_desc({}) = {:?}",
            n,
            n.clone().into_twos_complement_limbs_desc()
        );
    }
}

fn demo_integer_twos_complement_limbs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs({}) = {:?}",
            n,
            n.twos_complement_limbs().collect::<Vec<Limb>>()
        );
    }
}

fn demo_integer_twos_complement_limbs_rev(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs({}).rev() = {:?}",
            n,
            n.twos_complement_limbs().rev().collect::<Vec<Limb>>()
        );
    }
}

fn demo_integer_twos_complement_limbs_get(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "twos_complement_limbs({}).get({}) = {:?}",
            n,
            i,
            n.twos_complement_limbs().get(i)
        );
    }
}

fn benchmark_limbs_twos_complement(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_twos_complement(&[Limb])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref limbs| no_out!(limbs_twos_complement(limbs))),
        )],
    );
}

fn benchmark_limbs_maybe_sign_extend_non_negative_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_maybe_sign_extend_non_negative_in_place(&mut [Limb])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| limbs_maybe_sign_extend_non_negative_in_place(limbs)),
        )],
    );
}

fn benchmark_limbs_twos_complement_in_place_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_twos_complement_in_place(&mut [Limb])",
        BenchmarkType::Algorithms,
        vecs_of_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [
            (
                "default",
                &mut (|ref mut limbs| no_out!(limbs_twos_complement_in_place(limbs))),
            ),
            (
                "integrated",
                &mut (|ref mut limbs| no_out!(limbs_twos_complement_in_place_alt_1(limbs))),
            ),
            (
                "sub 1 and not",
                &mut (|ref mut limbs| no_out!(limbs_twos_complement_in_place_alt_2(limbs))),
            ),
        ],
    );
}

fn benchmark_limbs_twos_complement_and_maybe_sign_extend_negative_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut [Limb])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| {
                limbs_twos_complement_and_maybe_sign_extend_negative_in_place(limbs)
            }),
        )],
    );
}

fn benchmark_integer_to_twos_complement_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_twos_complement_limbs_asc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_twos_complement_limbs_asc()",
                &mut (|n| no_out!(n.to_twos_complement_limbs_asc())),
            ),
            (
                "Integer.into_twos_complement_limbs_asc()",
                &mut (|n| no_out!(n.into_twos_complement_limbs_asc())),
            ),
            (
                "Integer.twos_complement_limbs().collect::<Vec<Limb>>()",
                &mut (|n| no_out!(n.twos_complement_limbs().collect::<Vec<Limb>>())),
            ),
        ],
    );
}

fn benchmark_integer_to_twos_complement_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_twos_complement_limbs_desc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_twos_complement_limbs_desc()",
                &mut (|n| no_out!(n.to_twos_complement_limbs_desc())),
            ),
            (
                "Integer.into_twos_complement_limbs_desc()",
                &mut (|n| no_out!(n.into_twos_complement_limbs_desc())),
            ),
            (
                "Integer.twos_complement_limbs().rev().collect::<Vec<Limb>>()",
                &mut (|n| no_out!(n.twos_complement_limbs().collect::<Vec<Limb>>())),
            ),
        ],
    );
}

fn benchmark_integer_twos_complement_limbs_get_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.twos_complement_limbs().get()",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.twos_complement_limbs().get(u)",
                &mut (|(n, u)| no_out!(n.twos_complement_limbs().get(u))),
            ),
            (
                "Integer.into_twos_complement_limbs_asc()[u]",
                &mut (|(n, u)| {
                    let u = usize::exact_from(u);
                    let non_negative = n >= 0;
                    let limbs = n.into_twos_complement_limbs_asc();
                    if u >= limbs.len() {
                        if non_negative {
                            0
                        } else {
                            Limb::MAX
                        }
                    } else {
                        limbs[u]
                    };
                }),
            ),
        ],
    );
}
