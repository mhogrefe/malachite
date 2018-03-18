use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub fn demo_integer_twos_complement_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs_asc({}) = {:?}",
            n,
            n.twos_complement_limbs_asc()
        );
    }
}

pub fn demo_integer_twos_complement_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "twos_complement_limbs_desc({}) = {:?}",
            n,
            n.twos_complement_limbs_desc()
        );
    }
}

pub fn benchmark_integer_twos_complement_limbs_asc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.twos_complement_limbs_asc()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|n| no_out!(n.twos_complement_limbs_asc())),
            ),
        ],
    );
}

pub fn benchmark_integer_twos_complement_limbs_desc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.twos_complement_limbs_desc()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|n| no_out!(n.twos_complement_limbs_desc())),
            ),
        ],
    );
}
