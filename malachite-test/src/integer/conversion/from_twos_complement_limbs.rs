use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use malachite_nz::integer::Integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_from_twos_complement_limbs_desc);
    register_demo!(registry, demo_integer_from_owned_twos_complement_limbs_asc);
    register_demo!(registry, demo_integer_from_owned_twos_complement_limbs_desc);
    register_bench!(
        registry,
        Small,
        benchmark_integer_from_twos_complement_limbs_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_integer_from_twos_complement_limbs_desc_evaluation_strategy
    );
}

fn demo_integer_from_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_asc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_asc(&xs)
        );
    }
}

fn demo_integer_from_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_desc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_desc(&xs)
        );
    }
}

fn demo_integer_from_owned_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_twos_complement_limbs_asc({:?}) = {:?}",
            xs,
            Integer::from_owned_twos_complement_limbs_asc(xs.clone())
        );
    }
}

fn demo_integer_from_owned_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_twos_complement_limbs_desc({:?}) = {:?}",
            xs,
            Integer::from_owned_twos_complement_limbs_desc(xs.clone())
        );
    }
}

fn benchmark_integer_from_twos_complement_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_limbs_asc(&[u32])",
        BenchmarkType::EvaluationStrategy,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "limbs.len()",
        &mut [
            (
                "Integer::from_twos_complement_limbs_asc(&[u32])",
                &mut (|ref limbs| no_out!(Integer::from_twos_complement_limbs_asc(limbs))),
            ),
            (
                "Integer::from_owned_twos_complement_limbs_asc(Vec<u32>)",
                &mut (|ref limbs| {
                    no_out!(Integer::from_owned_twos_complement_limbs_asc(
                        limbs.to_vec()
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_integer_from_twos_complement_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_limbs_desc(&[u32])",
        BenchmarkType::EvaluationStrategy,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "limbs.len()",
        &mut [
            (
                "Integer::from_twos_complement_limbs_desc(&[u32])",
                &mut (|ref limbs| no_out!(Integer::from_twos_complement_limbs_desc(limbs))),
            ),
            (
                "Integer::from_owned_twos_complement_limbs_desc(Vec<u32>)",
                &mut (|ref limbs| {
                    no_out!(Integer::from_owned_twos_complement_limbs_desc(
                        limbs.to_vec()
                    ))
                }),
            ),
        ],
    );
}
