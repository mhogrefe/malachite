use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base_test_util::num::logic::bit_block_access::assign_bits_naive;
use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::*;
use malachite_test::inputs::integer::*;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_neg_assign_bits);
    register_demo!(registry, demo_integer_assign_bits);
    register_bench!(registry, Small, benchmark_limbs_neg_assign_bits);
    register_bench!(registry, Large, benchmark_integer_assign_bits_algorithms);
}

fn demo_limbs_neg_assign_bits(gm: GenerationMode, limit: usize) {
    for (mut limbs, start, end, bits) in
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_2(gm)
            .take(limit)
    {
        let old_limbs = limbs.clone();
        limbs_neg_assign_bits(&mut limbs, start, end, &bits);
        println!(
            "limbs := {:?}; limbs_neg_assign_bits(&mut limbs, {}, {}, &{:?}); limbs = {:?}",
            old_limbs, start, end, bits, limbs
        );
    }
}

fn demo_integer_assign_bits(gm: GenerationMode, limit: usize) {
    for (mut n, start, end, bits) in
        quadruples_of_integer_small_unsigned_small_unsigned_and_natural_var_1(gm).take(limit)
    {
        let old_n = n.clone();
        n.assign_bits(start, end, &bits);
        println!(
            "n := {}; n.assign_bits({}, {}, &{}); n = {}",
            old_n, start, end, bits, n
        );
    }
}

fn benchmark_limbs_neg_assign_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_assign_bits(&mut [Limb], u64, u64, &[Limb])",
        BenchmarkType::Single,
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, end, _)| usize::exact_from(end)),
        "end",
        &mut [(
            "limbs_neg_assign_bits",
            &mut (|(ref mut limbs, start, end, ref bits)| {
                limbs_neg_assign_bits(limbs, start, end, bits)
            }),
        )],
    );
}

fn benchmark_integer_assign_bits_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.assign_bits(u64, u64, &Natural)",
        BenchmarkType::Algorithms,
        quadruples_of_integer_small_unsigned_small_unsigned_and_natural_var_1(gm),
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
                    assign_bits_naive::<Integer, Natural>(&mut n, start, end, &bits)
                }),
            ),
        ],
    );
}
