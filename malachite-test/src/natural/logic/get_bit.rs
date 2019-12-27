use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use num::{BigUint, One, Zero};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_unsigned;
use inputs::natural::{
    nrm_pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_get_bit);
    register_demo!(registry, demo_natural_get_bit);
    register_bench!(registry, Small, benchmark_limbs_get_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_get_bit_library_comparison
    );
}

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << usize::exact_from(index)) != BigUint::zero()
}

fn demo_limbs_get_bit(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_get_bit({:?}, {}) = {}",
            limbs,
            index,
            limbs_get_bit(&limbs, index)
        );
    }
}

fn demo_natural_get_bit(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_limbs_get_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_get_bit(&[u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref limbs, index)| no_out!(limbs_get_bit(limbs, index))),
        )],
    );
}

fn benchmark_natural_get_bit_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.get_bit(u64)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (_, index))| usize::exact_from(index)),
        "index",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (n, index))| no_out!(n.get_bit(index))),
            ),
            (
                "num",
                &mut (|((n, index), _, _)| no_out!(num_get_bit(&n, index))),
            ),
            (
                "rug",
                &mut (|(_, (n, index), _)| no_out!(n.get_bit(u32::exact_from(index)))),
            ),
        ],
    );
}
