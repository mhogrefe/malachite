use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::bit_convertible::{_from_bits_asc_alt, _from_bits_desc_alt};
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_bool;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_bits_asc);
    register_demo!(registry, demo_integer_from_bits_desc);
    register_bench!(registry, Large, benchmark_integer_from_bits_asc_algorithms);
    register_bench!(registry, Large, benchmark_integer_from_bits_desc_algorithms);
}

pub fn _from_bits_asc_naive(bits: &[bool]) -> Integer {
    if bits.is_empty() {
        return Integer::ZERO;
    }
    let mut n;
    if *bits.last().unwrap() {
        n = Integer::NEGATIVE_ONE;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { None } else { Some(u64::exact_from(i)) })
        {
            n.clear_bit(i);
        }
    } else {
        n = Integer::ZERO;
        for i in
            bits.iter()
                .enumerate()
                .filter_map(|(i, &bit)| if bit { Some(u64::exact_from(i)) } else { None })
        {
            n.set_bit(i);
        }
    };
    n
}

pub fn _from_bits_desc_naive(bits: &[bool]) -> Integer {
    if bits.is_empty() {
        return Integer::ZERO;
    }
    let mut n;
    if bits[0] {
        n = Integer::NEGATIVE_ONE;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                None
            } else {
                Some(u64::exact_from(i))
            }
        }) {
            n.clear_bit(i);
        }
    } else {
        n = Integer::ZERO;
        for i in bits.iter().rev().enumerate().filter_map(|(i, &bit)| {
            if bit {
                Some(u64::exact_from(i))
            } else {
                None
            }
        }) {
            n.set_bit(i);
        }
    };
    n
}

fn demo_integer_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Integer::from_bits_asc(&bits)
        );
    }
}

fn demo_integer_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Integer::from_bits_desc(&bits)
        );
    }
}

fn benchmark_integer_from_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer::from_bits_asc(&[bool])",
        BenchmarkType::Algorithms,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(Integer::from_bits_asc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(_from_bits_asc_alt::<Integer>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(_from_bits_asc_naive(bits))),
            ),
        ],
    );
}

fn benchmark_integer_from_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer::from_bits_desc(&[bool])",
        BenchmarkType::Algorithms,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [
            (
                "default",
                &mut (|ref bits| no_out!(Integer::from_bits_desc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(_from_bits_desc_alt::<Integer>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(_from_bits_desc_naive(bits))),
            ),
        ],
    );
}
