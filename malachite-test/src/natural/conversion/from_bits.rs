use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::vecs_of_bool;
use malachite_nz::natural::conversion::from_bits::{limbs_asc_from_bits_asc,
                                                   limbs_asc_from_bits_desc};
use malachite_nz::natural::Natural;

pub fn demo_limbs_asc_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "limbs_asc_from_bits_asc({:?}) = {:?}",
            bits,
            limbs_asc_from_bits_asc(&bits)
        );
    }
}

pub fn demo_limbs_asc_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "limbs_asc_from_bits_desc({:?}) = {:?}",
            bits,
            limbs_asc_from_bits_desc(&bits)
        );
    }
}

pub fn demo_natural_from_bits_asc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Natural::from_bits_asc(bits.as_slice())
        );
    }
}

pub fn demo_natural_from_bits_desc(gm: GenerationMode, limit: usize) {
    for bits in vecs_of_bool(gm).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Natural::from_bits_desc(bits.as_slice())
        );
    }
}

pub fn benchmark_limbs_asc_from_bits_asc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_asc_from_bits_asc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &[
            (
                "limbs_asc_from_bits_asc(&[bool])",
                &mut (|ref bits| no_out!(limbs_asc_from_bits_asc(bits))),
            ),
        ],
    );
}

pub fn benchmark_limbs_asc_from_bits_desc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_asc_from_bits_desc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &[
            (
                "limbs_asc_from_bits_desc(&[bool])",
                &mut (|ref bits| no_out!(limbs_asc_from_bits_desc(bits))),
            ),
        ],
    );
}

pub fn benchmark_natural_from_bits_asc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from_bits_asc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &[
            (
                "Natural::from_bits_asc(&[bool])",
                &mut (|ref bits| no_out!(Natural::from_bits_asc(bits))),
            ),
        ],
    );
}

pub fn benchmark_natural_from_bits_desc(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural::from_bits_desc(&[bool])",
        BenchmarkType::Single,
        vecs_of_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|bits| bits.len()),
        "bits.len()",
        &[
            (
                "Natural::from_bits_desc(&[bool])",
                &mut (|ref bits| no_out!(Natural::from_bits_desc(bits))),
            ),
        ],
    );
}
