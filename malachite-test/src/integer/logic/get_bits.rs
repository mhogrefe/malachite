use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::integers::_get_bits_naive;
use malachite_base::num::logic::traits::{BitBlockAccess, SignificantBits};
use malachite_nz::integer::logic::bit_block_access::{
    limbs_neg_limb_get_bits, limbs_slice_neg_get_bits, limbs_vec_neg_get_bits,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2,
    triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1,
};
use inputs::integer::triples_of_integer_small_unsigned_and_small_unsigned_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_neg_limb_get_bits);
    register_demo!(registry, demo_limbs_slice_neg_get_bits);
    register_demo!(registry, demo_limbs_vec_neg_get_bits);
    register_demo!(registry, demo_integer_get_bits);
    register_demo!(registry, demo_integer_get_bits_owned);
    register_bench!(registry, Small, benchmark_limbs_neg_limb_get_bits);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_neg_get_bits_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_get_bits_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_get_bits_algorithms);
}

fn demo_limbs_neg_limb_get_bits(gm: GenerationMode, limit: usize) {
    for (limb, start, end) in
        triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_neg_limb_get_bits({}, {}, {}) = {:?}",
            limb,
            start,
            end,
            limbs_neg_limb_get_bits(limb, start, end)
        );
    }
}

fn demo_limbs_slice_neg_get_bits(gm: GenerationMode, limit: usize) {
    for (limbs, start, end) in
        triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2(gm).take(limit)
    {
        println!(
            "limbs_slice_neg_get_bits({:?}, {}, {}) = {:?}",
            limbs,
            start,
            end,
            limbs_slice_neg_get_bits(&limbs, start, end)
        );
    }
}

fn demo_limbs_vec_neg_get_bits(gm: GenerationMode, limit: usize) {
    for (limbs, start, end) in
        triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2(gm).take(limit)
    {
        let old_limbs = limbs.clone();
        println!(
            "limbs_vec_neg_get_bits({:?}, {}, {}) = {:?}",
            old_limbs,
            start,
            end,
            limbs_vec_neg_get_bits(limbs, start, end)
        );
    }
}

fn demo_integer_get_bits(gm: GenerationMode, limit: usize) {
    for (n, start, end) in
        triples_of_integer_small_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "({}).get_bits({}, {}) = {}",
            n,
            start,
            end,
            n.get_bits(start, end)
        );
    }
}

fn demo_integer_get_bits_owned(gm: GenerationMode, limit: usize) {
    for (n, start, end) in
        triples_of_integer_small_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        let old_n = n.clone();
        println!(
            "({}).get_bits_owned({}, {}) = {}",
            old_n,
            start,
            end,
            n.get_bits_owned(start, end)
        );
    }
}

fn benchmark_limbs_neg_limb_get_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_neg_limb_get_bits(Limb, u64, u64)",
        BenchmarkType::Single,
        triples_of_positive_unsigned_small_unsigned_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, start, end)| usize::exact_from(max(start, end))),
        "max(start, end)",
        &mut [(
            "limbs_neg_limb_get_bits",
            &mut (|(limb, start, end)| no_out!(limbs_neg_limb_get_bits(limb, start, end))),
        )],
    );
}

fn benchmark_limbs_neg_get_bits_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_neg_get_bits(&[Limb], u64, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_limb_vec_small_unsigned_and_small_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_slice_neg_get_bits",
                &mut (|(ref limbs, start, end)| {
                    no_out!(limbs_slice_neg_get_bits(limbs, start, end))
                }),
            ),
            (
                "limbs_vec_neg_get_bits",
                &mut (|(limbs, start, end)| no_out!(limbs_vec_neg_get_bits(limbs, start, end))),
            ),
        ],
    );
}

fn benchmark_integer_get_bits_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.get_bits(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_small_unsigned_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "get_bits",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "get_bits_owned",
                &mut (|(n, start, end)| no_out!(n.get_bits_owned(start, end))),
            ),
        ],
    );
}

fn benchmark_integer_get_bits_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.get_bits(u64, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_small_unsigned_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|(n, _, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(n, start, end)| no_out!(n.get_bits(start, end))),
            ),
            (
                "naive",
                &mut (|(n, start, end)| {
                    no_out!(_get_bits_naive::<Integer, Natural>(&n, start, end))
                }),
            ),
        ],
    );
}
