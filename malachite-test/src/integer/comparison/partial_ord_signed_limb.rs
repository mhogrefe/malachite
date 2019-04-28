use std::cmp::Ordering;

use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::SignedLimb;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed, pairs_of_signed_and_integer,
    rm_pairs_of_signed_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_partial_cmp_signed_limb);
    register_demo!(registry, demo_signed_limb_partial_cmp_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_partial_cmp_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_partial_cmp_integer_library_comparison
    );
}

pub fn num_partial_cmp_signed_limb(x: &BigInt, i: SignedLimb) -> Option<Ordering> {
    x.partial_cmp(&BigInt::from(i))
}

fn demo_integer_partial_cmp_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        match n.partial_cmp(&i).unwrap() {
            Ordering::Less => println!("{} < {}", n, i),
            Ordering::Equal => println!("{} = {}", n, i),
            Ordering::Greater => println!("{} > {}", n, i),
        }
    }
}

fn demo_signed_limb_partial_cmp_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<SignedLimb>(gm).take(limit) {
        match i.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", i, n),
            Ordering::Equal => println!("{} = {}", i, n),
            Ordering::Greater => println!("{} > {}", i, n),
        }
    }
}

fn benchmark_integer_partial_cmp_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.partial_cmp(&SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.partial_cmp(&y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_partial_cmp_signed_limb(&x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}

fn benchmark_signed_limb_partial_cmp_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.partial_cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.partial_cmp(&y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.partial_cmp(&y)))),
        ],
    );
}
