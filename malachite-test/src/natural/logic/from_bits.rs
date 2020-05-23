use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitConvertible};
use malachite_base_test_util::num::logic::bit_convertible::{
    _from_bits_asc_alt, _from_bits_desc_alt,
};
use malachite_nz::natural::logic::bit_convertible::{
    limbs_asc_from_bits_asc, limbs_asc_from_bits_desc,
};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_bool;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_asc_from_bits_asc);
    register_demo!(registry, demo_limbs_asc_from_bits_desc);
    register_demo!(registry, demo_natural_from_bits_asc);
    register_demo!(registry, demo_natural_from_bits_desc);
    register_bench!(registry, Large, benchmark_limbs_asc_from_bits_asc);
    register_bench!(registry, Large, benchmark_limbs_asc_from_bits_desc);
    register_bench!(registry, Large, benchmark_natural_from_bits_asc_algorithms);
    register_bench!(registry, Large, benchmark_natural_from_bits_desc_algorithms);
}

pub fn _from_bits_asc_naive(bits: &[bool]) -> Natural {
    let mut n = Natural::ZERO;
    for i in bits
        .iter()
        .enumerate()
        .filter(|(_, &bit)| bit)
        .map(|p| u64::exact_from(p.0))
    {
        n.set_bit(i);
    }
    n
}

pub fn _from_bits_desc_naive(bits: &[bool]) -> Natural {
    let mut n = Natural::ZERO;
    for i in bits
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, &bit)| bit)
        .map(|p| u64::exact_from(p.0))
    {
        n.set_bit(i);
    }
    n
}

fn demo_limbs_asc_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "limbs_asc_from_bits_asc({:?}) = {:?}",
            bits,
            limbs_asc_from_bits_asc(&bits)
        );
    }
}

fn demo_limbs_asc_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "limbs_asc_from_bits_desc({:?}) = {:?}",
            bits,
            limbs_asc_from_bits_desc(&bits)
        );
    }
}

fn demo_natural_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Natural::from_bits_asc(&bits)
        );
    }
}

fn demo_natural_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Natural::from_bits_desc(&bits)
        );
    }
}

fn benchmark_limbs_asc_from_bits_asc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_asc_from_bits_asc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [(
            "limbs_asc_from_bits_asc(&[bool])",
            &mut (|ref bits| no_out!(limbs_asc_from_bits_asc(bits))),
        )],
    );
}

fn benchmark_limbs_asc_from_bits_desc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_asc_from_bits_desc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [(
            "limbs_asc_from_bits_desc(&[bool])",
            &mut (|ref bits| no_out!(limbs_asc_from_bits_desc(bits))),
        )],
    );
}

fn benchmark_natural_from_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from_bits_asc(&[bool])",
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
                &mut (|ref bits| no_out!(Natural::from_bits_asc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(_from_bits_asc_alt::<Natural>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(_from_bits_asc_naive(bits))),
            ),
        ],
    );
}

fn benchmark_natural_from_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from_bits_desc(&[bool])",
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
                &mut (|ref bits| no_out!(Natural::from_bits_desc(bits))),
            ),
            (
                "alt",
                &mut (|ref bits| no_out!(_from_bits_desc_alt::<Natural>(bits))),
            ),
            (
                "naive",
                &mut (|ref bits| no_out!(_from_bits_desc_naive(bits))),
            ),
        ],
    );
}
