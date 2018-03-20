use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::triples_of_integers;
use malachite_base::num::SignificantBits;
use malachite_base::num::{AddMul, AddMulAssign};
use malachite_nz::integer::Integer;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_add_mul_assign);
    register_demo!(registry, demo_integer_add_mul_assign_val_ref);
    register_demo!(registry, demo_integer_add_mul_assign_ref_val);
    register_demo!(registry, demo_integer_add_mul_assign_ref_ref);
    register_demo!(registry, demo_integer_add_mul);
    register_demo!(registry, demo_integer_add_mul_val_val_ref);
    register_demo!(registry, demo_integer_add_mul_val_ref_val);
    register_demo!(registry, demo_integer_add_mul_val_ref_ref);
    register_demo!(registry, demo_integer_add_mul_ref_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_add_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_assign_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_add_mul_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_val_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_val_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_val_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_add_mul_ref_ref_ref_algorithms
    );
}

fn demo_integer_add_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old, b_old, c_old, a
        );
    }
}

fn demo_integer_add_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!(
            "a := {}; x.add_mul_assign({}, &{}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_integer_add_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!(
            "a := {}; x.add_mul_assign(&{}, {}); x = {}",
            a_old, b, c_old, a
        );
    }
}

fn demo_integer_add_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!(
            "a := {}; x.add_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_integer_add_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.add_mul(b, c)
        );
    }
}

fn demo_integer_add_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.add_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.add_mul(b, &c)
        );
    }
}

fn demo_integer_add_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.add_mul(&b, c)
        );
    }
}

fn demo_integer_add_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

fn demo_integer_add_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        println!(
            "(&{}).add_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).add_mul(&b, &c)
        );
    }
}

fn bucketing_function(t: &(Integer, Integer, Integer)) -> usize {
    max(
        max(t.0.significant_bits(), t.1.significant_bits()),
        t.2.significant_bits(),
    ) as usize
}

const BUCKETING_LABEL: &str = "max(a.significant_bits(), b.significant_bits(), \
                               c.significant_bits())";

fn benchmark_integer_add_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul_assign(Integer, Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Integer.add_mul_assign(Integer, &Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, &c)),
            ),
            (
                "Integer.add_mul_assign(&Integer, Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Integer.add_mul_assign(&Integer, &Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, &c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.add_mul_assign(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul_assign(Integer, Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Integer += Integer * Integer",
                &mut (|(mut a, b, c)| a += b * c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul_assign(Integer, &Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, &c)),
            ),
            (
                "Integer += Integer * &Integer",
                &mut (|(mut a, b, c)| a += b * &c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(&Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul_assign(&Integer, Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Integer += &Integer * Integer",
                &mut (|(mut a, b, c)| a += &b * c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul_assign(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul_assign(&Integer, &Integer)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, &c)),
            ),
            (
                "Integer += &Integer * &Integer",
                &mut (|(mut a, b, c)| a += &b * &c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul(Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Integer.add_mul(Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, &c))),
            ),
            (
                "Integer.add_mul(&Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Integer.add_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, &c))),
            ),
            (
                "(&Integer).add_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, &c))),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.add_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul(Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Integer + Integer * Integer",
                &mut (|(a, b, c)| no_out!(a + b * c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_val_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul(Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, &c))),
            ),
            (
                "Integer + Integer * &Integer",
                &mut (|(a, b, c)| no_out!(a + b * &c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_val_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(&Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul(&Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Integer + &Integer * Integer",
                &mut (|(a, b, c)| no_out!(a + &b * c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_val_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.add_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Integer.add_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, &c))),
            ),
            (
                "Integer + &Integer * &Integer",
                &mut (|(a, b, c)| no_out!(a + &b * &c)),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_ref_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Integer).add_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "(&Integer).add_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, &c))),
            ),
            (
                "(&Integer) + &Integer * &Integer",
                &mut (|(a, b, c)| no_out!((&a) + &b * &c)),
            ),
        ],
    );
}
