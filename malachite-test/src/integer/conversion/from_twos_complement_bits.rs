use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_bool;
use malachite_nz::integer::Integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_from_twos_complement_bits_asc);
    register_demo!(registry, demo_integer_from_twos_complement_bits_desc);
    register_bench!(
        registry,
        Large,
        benchmark_integer_from_twos_complement_bits_asc
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_from_twos_complement_bits_desc
    );
}

fn demo_integer_from_twos_complement_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_twos_complement_bits_asc({:?}) = {:?}",
            bits,
            Integer::from_twos_complement_bits_asc(&bits)
        );
    }
}

fn demo_integer_from_twos_complement_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_twos_complement_bits_desc({:?}) = {:?}",
            bits,
            Integer::from_twos_complement_bits_desc(&bits)
        );
    }
}

fn benchmark_integer_from_twos_complement_bits_asc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_bits_asc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [(
            "Integer::from_twos_complement_bits_asc(&[bool])",
            &mut (|ref bits| no_out!(Integer::from_twos_complement_bits_asc(bits))),
        )],
    );
}

fn benchmark_integer_from_twos_complement_bits_desc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer::from_twos_complement_bits_desc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &mut [(
            "Integer::from_twos_complement_bits_desc(&[bool])",
            &mut (|ref bits| no_out!(Integer::from_twos_complement_bits_desc(bits))),
        )],
    );
}
