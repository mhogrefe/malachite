use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::pairs_of_ordering_and_vec_of_unsigned_var_1;
use malachite_nz::integer::Integer;

pub fn demo_integer_from_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_asc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_asc(sign, limbs.as_slice())
        );
    }
}

pub fn demo_integer_from_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_desc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_desc(sign, limbs.as_slice())
        );
    }
}

pub fn demo_integer_from_sign_and_owned_limbs_asc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_owned_limbs_asc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_owned_limbs_asc(sign, limbs.clone())
        );
    }
}

pub fn demo_integer_from_sign_and_owned_limbs_desc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_owned_limbs_desc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_owned_limbs_desc(sign, limbs.clone())
        );
    }
}

pub fn benchmark_integer_from_sign_and_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_sign_and_limbs_asc(Ordering, &[u32])",
        BenchmarkType::EvaluationStrategy,
        pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs)| limbs.len()),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            (
                "Integer::from_sign_and_limbs_asc(&[u32])",
                &mut (|(sign, ref limbs)| no_out!(Integer::from_sign_and_limbs_asc(sign, limbs))),
            ),
            (
                "Integer::from_sign_and_owned_limbs_asc(&[u32])",
                &mut (|(sign, limbs)| no_out!(Integer::from_sign_and_owned_limbs_asc(sign, limbs))),
            ),
        ],
    );
}

pub fn benchmark_integer_from_sign_and_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_sign_and_limbs_desc(Ordering, &[u32])",
        BenchmarkType::EvaluationStrategy,
        pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs)| limbs.len()),
        "max(x.significant_bits(), y.significant_bits())",
        &[
            (
                "Integer::from_sign_and_limbs_desc(&[u32])",
                &mut (|(sign, ref limbs)| no_out!(Integer::from_sign_and_limbs_desc(sign, limbs))),
            ),
            (
                "Integer::from_sign_and_owned_limbs_desc(&[u32])",
                &mut (|(sign, limbs)| {
                    no_out!(Integer::from_sign_and_owned_limbs_desc(sign, limbs))
                }),
            ),
        ],
    );
}
