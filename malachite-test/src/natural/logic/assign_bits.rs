use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::_assign_bits_naive;
use malachite_nz::natural::logic::bit_block_access::limbs_assign_bits;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_1;
use inputs::natural::quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_assign_bits);
    register_demo!(registry, demo_natural_assign_bits);
    register_bench!(registry, Small, benchmark_limbs_assign_bits);
    register_bench!(registry, Large, benchmark_natural_assign_bits_algorithms);
}

fn demo_limbs_assign_bits(gm: GenerationMode, limit: usize) {
    for (mut limbs, start, end, bits) in
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_1(gm)
            .take(limit)
    {
        let old_limbs = limbs.clone();
        limbs_assign_bits(&mut limbs, start, end, &bits);
        println!(
            "limbs := {:?}; limbs_assign_bits(&mut limbs, {}, {}, &{:?}); limbs = {:?}",
            old_limbs, start, end, bits, limbs
        );
    }
}

fn demo_natural_assign_bits(gm: GenerationMode, limit: usize) {
    for (mut n, start, end, bits) in
        quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1(gm).take(limit)
    {
        let old_n = n.clone();
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n
        );
    }
}

fn benchmark_limbs_assign_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_assign_bits(&mut [Limb], u64, u64, &[Limb])",
        BenchmarkType::Single,
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, end, _)| usize::exact_from(end)),
        "end",
        &mut [(
            "limbs_assign_bits",
            &mut (|(ref mut limbs, start, end, ref bits)| {
                limbs_assign_bits(limbs, start, end, bits)
            }),
        )],
    );
}

fn benchmark_natural_assign_bits_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.assign_bits(u64, u64, &Natural)",
        BenchmarkType::Algorithms,
        quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, end, _)| usize::exact_from(end)),
        "end",
        &mut [
            (
                "default",
                &mut (|(mut n, start, end, bits)| n.assign_bits(start, end, &bits)),
            ),
            (
                "naive",
                &mut (|(mut n, start, end, bits)| {
                    _assign_bits_naive::<Natural, Natural>(&mut n, start, end, &bits)
                }),
            ),
        ],
    );
}
