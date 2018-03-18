use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::vecs_of_unsigned;
use malachite_nz::integer::Integer;

pub fn demo_integer_from_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_asc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_asc(xs.as_slice())
        );
    }
}

pub fn demo_integer_from_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_desc({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_desc(xs.as_slice())
        );
    }
}

pub fn benchmark_integer_from_twos_complement_limbs_asc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_limbs_asc(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|ref limbs| no_out!(Integer::from_twos_complement_limbs_asc(limbs))),
            ),
        ],
    );
}

pub fn benchmark_integer_from_twos_complement_limbs_desc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_limbs_desc(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|xs| xs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|ref limbs| no_out!(Integer::from_twos_complement_limbs_desc(limbs))),
            ),
        ],
    );
}
