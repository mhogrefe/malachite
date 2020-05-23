use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitConvertible, BitIterable, SignificantBits};
use malachite_base_test_util::num::logic::bit_convertible::{_to_bits_asc_alt, _to_bits_desc_alt};
use malachite_nz::integer::logic::bit_convertible::{
    bits_slice_to_twos_complement_bits_negative, bits_to_twos_complement_bits_non_negative,
    bits_vec_to_twos_complement_bits_negative,
};
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{vecs_of_bool, vecs_of_bool_var_1};
use inputs::integer::integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_bits_to_twos_complement_bits_non_negative);
    register_demo!(registry, demo_bits_slice_to_twos_complement_bits_negative);
    register_demo!(registry, demo_bits_vec_to_twos_complement_bits_negative);
    register_demo!(registry, demo_integer_to_bits_asc);
    register_demo!(registry, demo_integer_to_bits_desc);
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
        benchmark_integer_to_bits_asc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_to_bits_asc_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_to_bits_desc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_to_bits_desc_algorithms);
}

pub fn _to_bits_asc_naive(n: &Integer) -> Vec<bool> {
    let mut bits = Vec::new();
    if *n == 0 {
        return bits;
    }
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    let last_bit = *bits.last().unwrap();
    if last_bit != (*n < 0) {
        bits.push(!last_bit);
    }
    bits
}

pub fn _to_bits_desc_naive(n: &Integer) -> Vec<bool> {
    let mut bits = Vec::new();
    if *n == 0 {
        return bits;
    }
    let significant_bits = n.significant_bits();
    let last_bit = n.get_bit(significant_bits - 1);
    if last_bit != (*n < 0) {
        bits.push(!last_bit);
    }
    for i in (0..significant_bits).rev() {
        bits.push(n.get_bit(i));
    }
    bits
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

fn demo_integer_to_bits_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

fn demo_integer_to_bits_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
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

fn benchmark_integer_to_bits_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer.to_bits_asc()", &mut (|n| no_out!(n.to_bits_asc()))),
            (
                "Integer.bits().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_integer_to_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_bits_asc()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_asc()))),
            ("alt", &mut (|n| no_out!(_to_bits_asc_alt(&n)))),
            ("naive", &mut (|n| no_out!(_to_bits_asc_naive(&n)))),
        ],
    );
}

#[allow(unused_collect)]
fn benchmark_integer_to_bits_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_bits_desc()",
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                "Integer.bits().rev().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

fn benchmark_integer_to_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.to_bits_desc()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_desc()))),
            ("alt", &mut (|n| no_out!(_to_bits_desc_alt(&n)))),
            ("naive", &mut (|n| no_out!(_to_bits_desc_naive(&n)))),
        ],
    );
}
