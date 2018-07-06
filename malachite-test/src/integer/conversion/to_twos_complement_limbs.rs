use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{vecs_of_u32_var_1, vecs_of_unsigned};
use inputs::integer::{integers, pairs_of_integer_and_small_usize};
use malachite_base::num::{SignificantBits, WrappingNegAssign};
use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
use malachite_nz::natural::arithmetic::sub_u32::limbs_sub_limb_in_place;
use malachite_nz::natural::logic::not::limbs_not_in_place;
use std::u32;

pub fn limbs_slice_to_twos_complement_limbs_negative_alt_1(limbs: &mut [u32]) -> bool {
    let i = limbs.iter().cloned().take_while(|&x| x == 0).count();
    let len = limbs.len();
    if i == len {
        return true;
    }
    limbs[i].wrapping_neg_assign();
    let j = i + 1;
    if j != len {
        limbs_not_in_place(&mut limbs[j..]);
    }
    false
}

pub fn limbs_slice_to_twos_complement_limbs_negative_alt_2(limbs: &mut [u32]) -> bool {
    let carry = limbs_sub_limb_in_place(limbs, 1);
    limbs_not_in_place(limbs);
    carry
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_to_twos_complement_limbs_non_negative);
    register_demo!(registry, demo_limbs_slice_to_twos_complement_limbs_negative);
    register_demo!(registry, demo_limbs_vec_to_twos_complement_limbs_negative);
    register_demo!(registry, demo_integer_to_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_to_twos_complement_limbs_desc);
    register_demo!(registry, demo_integer_into_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_into_twos_complement_limbs_desc);
    register_demo!(registry, demo_integer_twos_complement_limbs);
    register_demo!(registry, demo_integer_twos_complement_limbs_rev);
    register_demo!(registry, demo_integer_twos_complement_limbs_get);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_to_twos_complement_limbs_non_negative
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_to_twos_complement_limbs_negative_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_to_twos_complement_limbs_negative
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

fn demo_limbs_to_twos_complement_limbs_non_negative(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_to_twos_complement_limbs_non_negative(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_to_twos_complement_limbs_non_negative(&mut limbs); limbs = {:?}",
            limbs, mut_limbs
        );
    }
}

fn demo_limbs_slice_to_twos_complement_limbs_negative(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        let carry = limbs_slice_to_twos_complement_limbs_negative(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_slice_to_twos_complement_limbs_negative(&mut limbs) = {}; \
             limbs = {:?}",
            limbs, carry, mut_limbs
        );
    }
}

fn demo_limbs_vec_to_twos_complement_limbs_negative(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_u32_var_1(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_vec_to_twos_complement_limbs_negative(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_vec_to_twos_complement_limbs_negative(&mut limbs); \
             limbs = {:?}",
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
            n.twos_complement_limbs().collect::<Vec<u32>>()
        );
    }
}

fn demo_integer_twos_complement_limbs_rev(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs({}).rev() = {:?}",
            n,
            n.twos_complement_limbs().rev().collect::<Vec<u32>>()
        );
    }
}

fn demo_integer_twos_complement_limbs_get(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_usize(gm).take(limit) {
        println!(
            "twos_complement_limbs({}).get({}) = {:?}",
            n,
            i,
            n.twos_complement_limbs().get(i)
        );
    }
}

fn benchmark_limbs_to_twos_complement_limbs_non_negative(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_to_twos_complement_limbs_non_negative(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| limbs_to_twos_complement_limbs_non_negative(limbs)),
        )],
    );
}

fn benchmark_limbs_slice_to_twos_complement_limbs_negative_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_to_twos_complement_limbs_negative(&mut [u32])",
        BenchmarkType::Algorithms,
        vecs_of_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [
            (
                "default",
                &mut (|ref mut limbs| {
                    no_out!(limbs_slice_to_twos_complement_limbs_negative(limbs))
                }),
            ),
            (
                "integrated",
                &mut (|ref mut limbs| {
                    no_out!(limbs_slice_to_twos_complement_limbs_negative_alt_1(limbs))
                }),
            ),
            (
                "sub 1 and not",
                &mut (|ref mut limbs| {
                    no_out!(limbs_slice_to_twos_complement_limbs_negative_alt_2(limbs))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_vec_to_twos_complement_limbs_negative(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_to_twos_complement_limbs_negative(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| limbs_vec_to_twos_complement_limbs_negative(limbs)),
        )],
    );
}

#[allow(unused_collect)]
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
        &(|n| n.significant_bits() as usize),
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
                "Integer.twos_complement_limbs().collect::<Vec<u32>>()",
                &mut (|n| no_out!(n.twos_complement_limbs().collect::<Vec<u32>>())),
            ),
        ],
    );
}

#[allow(unused_collect)]
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
        &(|n| n.significant_bits() as usize),
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
                "Integer.twos_complement_limbs().rev().collect::<Vec<u32>>()",
                &mut (|n| no_out!(n.twos_complement_limbs().collect::<Vec<u32>>())),
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
        pairs_of_integer_and_small_usize(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.twos_complement_limbs().get(u)",
                &mut (|(n, u)| no_out!(n.twos_complement_limbs().get(u))),
            ),
            (
                "Integer.into_twos_complement_limbs_asc()[u]",
                &mut (|(n, u)| {
                    let non_negative = n >= 0;
                    let limbs = n.into_twos_complement_limbs_asc();
                    if u >= limbs.len() {
                        if non_negative {
                            0
                        } else {
                            u32::MAX
                        }
                    } else {
                        limbs[u]
                    };
                }),
            ),
        ],
    );
}
