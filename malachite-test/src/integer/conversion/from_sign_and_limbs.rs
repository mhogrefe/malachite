use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_ordering_and_vec_of_unsigned_var_1;
use malachite_nz::integer::Integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_sign_and_limbs_asc);
    register_demo!(registry, demo_integer_from_sign_and_limbs_desc);
    register_demo!(registry, demo_integer_from_sign_and_owned_limbs_asc);
    register_demo!(registry, demo_integer_from_sign_and_owned_limbs_desc);
    register_bench!(
        registry,
        Small,
        benchmark_integer_from_sign_and_limbs_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_integer_from_sign_and_limbs_desc_evaluation_strategy
    );
}

fn demo_integer_from_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_asc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_asc(sign, &limbs)
        );
    }
}

fn demo_integer_from_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_desc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_desc(sign, &limbs)
        );
    }
}

fn demo_integer_from_sign_and_owned_limbs_asc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_owned_limbs_asc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_owned_limbs_asc(sign, limbs.clone())
        );
    }
}

fn demo_integer_from_sign_and_owned_limbs_desc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_owned_limbs_desc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_owned_limbs_desc(sign, limbs.clone())
        );
    }
}

fn benchmark_integer_from_sign_and_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_sign_and_limbs_asc(Ordering, &[Limb])",
        BenchmarkType::EvaluationStrategy,
        pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "Integer::from_sign_and_limbs_asc(&[Limb])",
                &mut (|(sign, ref limbs)| no_out!(Integer::from_sign_and_limbs_asc(sign, limbs))),
            ),
            (
                "Integer::from_sign_and_owned_limbs_asc(&[Limb])",
                &mut (|(sign, limbs)| no_out!(Integer::from_sign_and_owned_limbs_asc(sign, limbs))),
            ),
        ],
    );
}

fn benchmark_integer_from_sign_and_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_sign_and_limbs_desc(Ordering, &[Limb])",
        BenchmarkType::EvaluationStrategy,
        pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "Integer::from_sign_and_limbs_desc(&[Limb])",
                &mut (|(sign, ref limbs)| no_out!(Integer::from_sign_and_limbs_desc(sign, limbs))),
            ),
            (
                "Integer::from_sign_and_owned_limbs_desc(&[Limb])",
                &mut (|(sign, limbs)| {
                    no_out!(Integer::from_sign_and_owned_limbs_desc(sign, limbs))
                }),
            ),
        ],
    );
}
