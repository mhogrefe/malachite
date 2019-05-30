use std::iter::repeat;

use malachite_base::num::conversion::traits::{CheckedFrom, WrappingFrom};
use malachite_base::num::logic::traits::{
    CheckedHammingDistance, HammingDistance, SignificantBits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};

pub fn integer_checked_hamming_distance_limb_alt_1(n: &Integer, u: Limb) -> Option<u64> {
    if *n < 0 as Limb {
        return None;
    }
    let u = Natural::from(u);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= u.significant_bits() {
            Box::new(n.twos_complement_bits().zip(u.bits().chain(repeat(false))))
        } else {
            Box::new(n.twos_complement_bits().chain(repeat(false)).zip(u.bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub fn integer_checked_hamming_distance_limb_alt_2(n: &Integer, u: Limb) -> Option<u64> {
    if *n < 0 as Limb {
        return None;
    }
    let u = Natural::from(u);
    let limb_zip: Box<Iterator<Item = (Limb, Limb)>> =
        if u64::wrapping_from(n.twos_complement_limbs().count()) >= u.limb_count() {
            Box::new(n.twos_complement_limbs().zip(u.limbs().chain(repeat(0))))
        } else {
            Box::new(n.twos_complement_limbs().chain(repeat(0)).zip(u.limbs()))
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_checked_hamming_distance_limb);
    register_demo!(registry, demo_limb_checked_hamming_distance_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_hamming_distance_integer
    );
}

fn demo_integer_checked_hamming_distance_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            n,
            u,
            n.checked_hamming_distance(u)
        );
    }
}

fn demo_limb_checked_hamming_distance_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            u,
            n,
            u.checked_hamming_distance(&n)
        );
    }
}

fn benchmark_integer_checked_hamming_distance_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.checked_hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_limb_alt_1(&n, other))
                }),
            ),
            (
                "using limbs explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_limb_alt_2(&n, other))
                }),
            ),
        ],
    );
}

fn benchmark_limb_checked_hamming_distance_integer(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.checked_hamming_distance(&Integer)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "default",
            &mut (|(u, ref other)| no_out!(u.checked_hamming_distance(other))),
        )],
    );
}
