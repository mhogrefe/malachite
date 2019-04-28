use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::triples_of_integer_integer_and_unsigned;
use malachite_base::num::traits::{AddMul, AddMulAssign, SignificantBits};
use malachite_nz::platform::Limb;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_add_mul_assign_limb);
    register_demo!(registry, demo_integer_add_mul_assign_limb_ref);
    register_demo!(registry, demo_integer_add_mul_limb);
    register_demo!(registry, demo_integer_add_mul_limb_val_ref);
    register_demo!(registry, demo_integer_add_mul_limb_ref_val);
    register_demo!(registry, demo_integer_add_mul_limb_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_limb_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_add_mul_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_limb_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_limb_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_limb_ref_ref_algorithms
    );
}

fn demo_integer_add_mul_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_integer_add_mul_assign_limb_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, c);
        println!("a := {}; x.add_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

fn demo_integer_add_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.add_mul({}, {}) = {}", a_old, b_old, c, a.add_mul(b, c));
    }
}

fn demo_integer_add_mul_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, {}) = {}", a_old, b, c, a.add_mul(&b, c));
    }
}

fn demo_integer_add_mul_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).add_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).add_mul(b, c)
        );
    }
}

fn demo_integer_add_mul_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).add_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).add_mul(&b, c)
        );
    }
}

fn benchmark_integer_add_mul_assign_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(Integer, Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul_assign(Integer, Limb)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Integer.add_mul_assign(&Integer, Limb)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul_assign(Integer, Limb)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Integer += Integer * Limb",
                &mut (|(mut a, b, c)| a += b * c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_limb_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(&Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul_assign(&Integer, Limb)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Integer += &Integer * Limb",
                &mut (|(mut a, b, c)| a += &b * c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(Integer, Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul(Integer, Limb)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Integer.add_mul(&Integer, Limb)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "(&Integer).add_mul(Integer, Limb)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(b, c))),
            ),
            (
                "(&Integer).add_mul(&Integer, Limb)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, c))),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.add_mul(Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul(Integer, Limb)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Integer + Integer * Limb",
                &mut (|(a, b, c)| no_out!(a + b * c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_limb_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(&Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Integer.add_mul(&Integer, Limb)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Integer + &Integer * Limb",
                &mut (|(a, b, c)| no_out!(a + &b * c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_limb_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).add_mul(Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Integer).add_mul(Integer, Limb)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(b, c))),
            ),
            (
                "(&Integer) + Integer * Limb",
                &mut (|(a, b, c)| no_out!(&a + b * c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_limb_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).add_mul(&Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Integer).add_mul(&Integer, Limb)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, c))),
            ),
            (
                "(&Integer) + &Integer * Limb",
                &mut (|(a, b, c)| no_out!(&a + &b * c)),
            ),
        ],
    );
}
