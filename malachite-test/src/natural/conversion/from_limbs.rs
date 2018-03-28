use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned;
use malachite_nz::natural::Natural;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_from_limbs_asc);
    register_demo!(registry, demo_natural_from_limbs_desc);
    register_demo!(registry, demo_natural_from_owned_limbs_asc);
    register_demo!(registry, demo_natural_from_owned_limbs_desc);
    register_bench!(
        registry,
        Small,
        benchmark_natural_from_limbs_asc_evaluation_strategy
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_from_limbs_desc_evaluation_strategy
    );
}

fn demo_natural_from_limbs_asc(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_limbs_asc({:?}) = {:?}",
            limbs,
            Natural::from_limbs_asc(&limbs)
        );
    }
}

fn demo_natural_from_limbs_desc(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_limbs_desc({:?}) = {:?}",
            limbs,
            Natural::from_limbs_desc(&limbs)
        );
    }
}

fn demo_natural_from_owned_limbs_asc(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_limbs_asc({:?}) = {:?}",
            limbs,
            Natural::from_owned_limbs_asc(limbs.clone())
        );
    }
}

fn demo_natural_from_owned_limbs_desc(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_limbs_desc({:?}) = {:?}",
            limbs,
            Natural::from_owned_limbs_desc(limbs.clone())
        );
    }
}

fn benchmark_natural_from_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from_limbs_asc(&[u32])",
        BenchmarkType::EvaluationStrategy,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "Natural::from_limbs_asc(&[u32])",
                &mut (|ref limbs| no_out!(Natural::from_limbs_asc(limbs))),
            ),
            (
                "Natural::from_owned_limbs_asc(&[u32])",
                &mut (|limbs| no_out!(Natural::from_owned_limbs_asc(limbs))),
            ),
        ],
    );
}

fn benchmark_natural_from_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::from_limbs_desc(&[u32])",
        BenchmarkType::EvaluationStrategy,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "Natural::from_limbs_desc(&[u32])",
                &mut (|ref limbs| no_out!(Natural::from_limbs_asc(limbs))),
            ),
            (
                "Natural::from_owned_limbs_desc(&[u32])",
                &mut (|limbs| no_out!(Natural::from_owned_limbs_asc(limbs))),
            ),
        ],
    );
}
