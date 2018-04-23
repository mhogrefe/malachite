use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{vecs_of_bool, vecs_of_bool_var_1};
use inputs::integer::{integers, pairs_of_integer_and_small_u64};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::conversion::to_twos_complement_bits::*;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_bits_to_twos_complement_bits_non_negative);
    register_demo!(registry, demo_bits_slice_to_twos_complement_bits_negative);
    register_demo!(registry, demo_bits_vec_to_twos_complement_bits_negative);
    register_demo!(registry, demo_integer_to_twos_complement_bits_asc);
    register_demo!(registry, demo_integer_to_twos_complement_bits_desc);
    register_demo!(registry, demo_integer_twos_complement_bits);
    register_demo!(registry, demo_integer_twos_complement_bits_rev);
    register_demo!(registry, demo_integer_twos_complement_bits_index);
    register_bench!(
        registry,
        Small,
        benchmark_bits_to_twos_complement_bits_non_negative
    );
    register_bench!(
        registry,
        Small,
        benchmark_bits_slice_to_twos_complement_bits_negative
    );
    register_bench!(
        registry,
        Small,
        benchmark_bits_vec_to_twos_complement_bits_negative
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_twos_complement_bits_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_twos_complement_bits_desc_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_twos_complement_bits_get_algorithms
    );
}

fn demo_bits_to_twos_complement_bits_non_negative(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        let mut mut_bits = bits.clone();
        bits_to_twos_complement_bits_non_negative(&mut mut_bits);
        println!(
            "bits := {:?}; bits_to_twos_complement_bits_non_negative(&mut bits); bits = {:?}",
            bits, mut_bits
        );
    }
}

fn demo_bits_slice_to_twos_complement_bits_negative(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        let mut mut_bits = bits.clone();
        let carry = bits_slice_to_twos_complement_bits_negative(&mut mut_bits);
        println!(
            "bits := {:?}; bits_slice_to_twos_complement_bits_negative(&mut bits) = {}; \
             bits = {:?}",
            bits, carry, mut_bits
        );
    }
}

fn demo_bits_vec_to_twos_complement_bits_negative(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool_var_1(gm).take(limit) {
        let mut mut_bits = bits.clone();
        bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
        println!(
            "bits := {:?}; bits_vec_to_twos_complement_bits_negative(&mut bits); bits = {:?}",
            bits, mut_bits
        );
    }
}

fn demo_integer_to_twos_complement_bits_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_twos_complement_bits_asc({}) = {:?}",
            n,
            n.to_twos_complement_bits_asc()
        );
    }
}

fn demo_integer_to_twos_complement_bits_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_twos_complement_bits_desc({}) = {:?}",
            n,
            n.to_twos_complement_bits_desc()
        );
    }
}

fn demo_integer_twos_complement_bits(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_bits({}) = {:?}",
            n,
            n.twos_complement_bits().collect::<Vec<bool>>()
        );
    }
}

fn demo_integer_twos_complement_bits_rev(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_bits({}).rev() = {:?}",
            n,
            n.twos_complement_bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_integer_twos_complement_bits_index(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_small_u64(gm).take(limit) {
        println!(
            "twos_complement_bits({})[{}] = {:?}",
            n,
            i,
            n.twos_complement_bits()[i]
        );
    }
}

fn benchmark_bits_to_twos_complement_bits_non_negative(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "bits_to_twos_complement_bits_non_negative(&mut [bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut bits| bits_to_twos_complement_bits_non_negative(bits)),
        )],
    );
}

fn benchmark_bits_slice_to_twos_complement_bits_negative(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "bits_slice_to_twos_complement_bits_negative(&mut [bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut bits| no_out!(bits_slice_to_twos_complement_bits_negative(bits))),
        )],
    );
}

fn benchmark_bits_vec_to_twos_complement_bits_negative(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "bits_vec_to_twos_complement_bits_negative(&mut [bool])",
        BenchmarkType::Single,
        vecs_of_bool_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|ref mut bits| bits_vec_to_twos_complement_bits_negative(bits)),
        )],
    );
}

#[allow(unused_collect)]
fn benchmark_integer_to_twos_complement_bits_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_twos_complement_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_twos_complement_bits_asc()",
                &mut (|n| no_out!(n.to_twos_complement_bits_asc())),
            ),
            (
                "Integer.twos_complement_bits().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.twos_complement_bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

#[allow(unused_collect)]
fn benchmark_integer_to_twos_complement_bits_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_twos_complement_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_twos_complement_bits_desc()",
                &mut (|n| no_out!(n.to_twos_complement_bits_desc())),
            ),
            (
                "Integer.twos_complement_bits().rev().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.twos_complement_bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_integer_twos_complement_bits_get_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.twos_complement_bits()[u64]",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.twos_complement_bits()[u]",
                &mut (|(n, u)| no_out!(n.twos_complement_bits()[u])),
            ),
            (
                "Integer.into_twos_complement_bits_asc()[u]",
                &mut (|(n, u)| {
                    let bits = n.to_twos_complement_bits_asc();
                    let u = u as usize;
                    if u >= bits.len() {
                        n < 0
                    } else {
                        bits[u]
                    };
                }),
            ),
        ],
    );
}
