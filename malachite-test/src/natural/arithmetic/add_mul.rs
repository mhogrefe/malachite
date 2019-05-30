use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::add_mul::{limbs_add_mul, limbs_add_mul_in_place_left};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_var_27;
use inputs::natural::triples_of_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_add_mul);
    register_demo!(registry, demo_limbs_add_mul_in_place_left);
    register_demo!(registry, demo_natural_add_mul_assign);
    register_demo!(registry, demo_natural_add_mul_assign_val_ref);
    register_demo!(registry, demo_natural_add_mul_assign_ref_val);
    register_demo!(registry, demo_natural_add_mul_assign_ref_ref);
    register_demo!(registry, demo_natural_add_mul);
    register_demo!(registry, demo_natural_add_mul_val_val_ref);
    register_demo!(registry, demo_natural_add_mul_val_ref_val);
    register_demo!(registry, demo_natural_add_mul_val_ref_ref);
    register_demo!(registry, demo_natural_add_mul_ref_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_add_mul);
    register_bench!(registry, Small, benchmark_limbs_add_mul_in_place_left);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_add_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_assign_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_add_mul_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_val_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_val_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_val_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_add_mul_ref_ref_ref_algorithms
    );
}

fn demo_limbs_add_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_unsigned_vec_var_27(gm).take(limit) {
        println!(
            "limbs_add_mul({:?}, {:?}, {:?}) = {:?}",
            a,
            b,
            c,
            limbs_add_mul(&a, &b, &c),
        );
    }
}

fn demo_limbs_add_mul_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_unsigned_vec_var_27(gm).take(limit) {
        let a_old = a.clone();
        limbs_add_mul_in_place_left(&mut a, &b, &c);
        println!(
            "a := {:?}; limbs_add_mul_in_place_left(&mut a, {:?}, {:?}); a = {:?}",
            a_old, b, c, a,
        );
    }
}

fn demo_natural_add_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals(gm).take(limit) {
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

fn demo_natural_add_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!(
            "a := {}; x.add_mul_assign({}, &{}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_add_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!(
            "a := {}; x.add_mul_assign(&{}, {}); x = {}",
            a_old, b, c_old, a
        );
    }
}

fn demo_natural_add_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!(
            "a := {}; x.add_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_natural_add_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
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

fn demo_natural_add_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
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

fn demo_natural_add_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
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

fn demo_natural_add_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

fn demo_natural_add_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        println!(
            "(&{}).add_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).add_mul(&b, &c)
        );
    }
}

fn benchmark_limbs_add_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_add_mul(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_27(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, ref c)| max!(a.len(), b.len(), c.len())),
        "max(a.len(), b.len(), c.len())",
        &mut [(
            "malachite",
            &mut (|(a, b, c)| no_out!(limbs_add_mul(&a, &b, &c))),
        )],
    );
}

fn benchmark_limbs_add_mul_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_add_mul_in_place_left(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_27(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, ref c)| max!(a.len(), b.len(), c.len())),
        "max(a.len(), b.len(), c.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| no_out!(limbs_add_mul_in_place_left(&mut a, &b, &c))),
        )],
    );
}

fn bucketing_function(t: &(Natural, Natural, Natural)) -> usize {
    usize::checked_from(max!(
        t.0.significant_bits(),
        t.1.significant_bits(),
        t.2.significant_bits()
    ))
    .unwrap()
}

const BUCKETING_LABEL: &str = "max(a.significant_bits(), b.significant_bits(), \
                               c.significant_bits())";

fn benchmark_natural_add_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul_assign(Natural, Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Natural.add_mul_assign(Natural, &Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, &c)),
            ),
            (
                "Natural.add_mul_assign(&Natural, Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Natural.add_mul_assign(&Natural, &Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, &c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul_assign(Natural, Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, c)),
            ),
            (
                "Natural += Natural * Natural",
                &mut (|(mut a, b, c)| a += b * c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul_assign(Natural, &Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(b, &c)),
            ),
            (
                "Natural += Natural * &Natural",
                &mut (|(mut a, b, c)| a += b * &c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul_assign(&Natural, Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, c)),
            ),
            (
                "Natural += &Natural * Natural",
                &mut (|(mut a, b, c)| a += &b * c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_assign_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul_assign(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul_assign(&Natural, &Natural)",
                &mut (|(mut a, b, c)| a.add_mul_assign(&b, &c)),
            ),
            (
                "Natural += &Natural * &Natural",
                &mut (|(mut a, b, c)| a += &b * &c),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul(Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Natural.add_mul(Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, &c))),
            ),
            (
                "Natural.add_mul(&Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Natural.add_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, &c))),
            ),
            (
                "(&Natural).add_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, &c))),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul(Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, c))),
            ),
            (
                "Natural + Natural * Natural",
                &mut (|(a, b, c)| no_out!(a + b * c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_val_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul(Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(b, &c))),
            ),
            (
                "Natural + Natural * &Natural",
                &mut (|(a, b, c)| no_out!(a + b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_val_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul(&Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, c))),
            ),
            (
                "Natural + &Natural * Natural",
                &mut (|(a, b, c)| no_out!(a + &b * c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_val_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.add_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.add_mul(&b, &c))),
            ),
            (
                "Natural + &Natural * &Natural",
                &mut (|(a, b, c)| no_out!(a + &b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_add_mul_ref_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).add_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "(&Natural).add_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).add_mul(&b, &c))),
            ),
            (
                "(&Natural) + &Natural * &Natural",
                &mut (|(a, b, c)| no_out!((&a) + &b * &c)),
            ),
        ],
    );
}
