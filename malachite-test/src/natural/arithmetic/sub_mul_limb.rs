use std::cmp::max;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{SignificantBits, SubMul, SubMulAssign};
use malachite_nz::natural::arithmetic::sub_mul_limb::{
    limbs_sub_mul_limb_greater, limbs_sub_mul_limb_greater_in_place_left,
    limbs_sub_mul_limb_greater_in_place_right, limbs_sub_mul_limb_same_length_in_place_left,
    limbs_sub_mul_limb_same_length_in_place_right,
};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
};
use inputs::natural::triples_of_natural_natural_and_limb_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_sub_mul_limb_greater);
    register_demo!(registry, demo_limbs_sub_mul_limb_same_length_in_place_left);
    register_demo!(registry, demo_limbs_sub_mul_limb_greater_in_place_left);
    register_demo!(registry, demo_limbs_sub_mul_limb_same_length_in_place_right);
    register_demo!(registry, demo_limbs_sub_mul_limb_greater_in_place_right);
    register_demo!(registry, demo_natural_sub_mul_assign_limb);
    register_demo!(registry, demo_natural_sub_mul_assign_limb_ref);
    register_demo!(registry, demo_natural_sub_mul_limb);
    register_demo!(registry, demo_natural_sub_mul_limb_val_ref);
    register_demo!(registry, demo_natural_sub_mul_limb_ref_val);
    register_demo!(registry, demo_natural_sub_mul_limb_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_sub_mul_limb_greater);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_mul_limb_same_length_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_mul_limb_greater_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_mul_limb_same_length_in_place_right
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_mul_limb_greater_in_place_right
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_assign_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_assign_limb_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_sub_mul_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_ref_ref_algorithms
    );
}

fn demo_limbs_sub_mul_limb_greater(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_sub_mul_limb_greater({:?}, {:?}, {}) = {:?}",
            a,
            b,
            c,
            limbs_sub_mul_limb_greater(&a, &b, c),
        );
    }
}

fn demo_limbs_sub_mul_limb_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm).take(limit) {
        let a_old = a.clone();
        let borrow = limbs_sub_mul_limb_same_length_in_place_left(&mut a, &b, c);
        println!(
            "a := {:?}; limbs_sub_mul_limb_same_length_in_place_left(&mut a, {:?}, {}) = {}; \
             a = {:?}",
            a_old, b, c, borrow, a,
        );
    }
}

fn demo_limbs_sub_mul_limb_greater_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit) {
        let a_old = a.clone();
        let borrow = limbs_sub_mul_limb_greater_in_place_left(&mut a, &b, c);
        println!(
            "a := {:?}; limbs_sub_mul_limb_greater_in_place_left(&mut a, {:?}, {}) = {}; \
             a = {:?}",
            a_old, b, c, borrow, a,
        );
    }
}

fn demo_limbs_sub_mul_limb_same_length_in_place_right(gm: GenerationMode, limit: usize) {
    for (a, mut b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm).take(limit) {
        let b_old = b.clone();
        limbs_sub_mul_limb_same_length_in_place_right(&a, &mut b, c);
        println!(
            "b := {:?}; limbs_sub_mul_limb_same_length_in_place_right({:?}, &mut b, {}); b = {:?}",
            b_old, a, c, b,
        );
    }
}

fn demo_limbs_sub_mul_limb_greater_in_place_right(gm: GenerationMode, limit: usize) {
    for (a, mut b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm).take(limit) {
        let b_old = b.clone();
        limbs_sub_mul_limb_greater_in_place_right(&a, &mut b, c);
        println!(
            "b := {:?}; limbs_sub_mul_limb_greater_in_place_right({:?}, &mut b, {}); b = {:?}",
            b_old, a, c, b,
        );
    }
}

fn demo_natural_sub_mul_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_sub_mul_assign_limb_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

fn demo_natural_sub_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, {}) = {:?}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, c)
        );
    }
}

fn demo_natural_sub_mul_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {:?}", a_old, b, c, a.sub_mul(&b, c));
    }
}

fn demo_natural_sub_mul_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let b_old = a.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            a,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

fn demo_natural_sub_mul_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, {}) = {:?}",
            a,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

fn benchmark_natural_sub_mul_assign_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul_assign(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul_assign(Natural, Limb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Natural -= Natural * Limb",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_limb_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul_assign(&Natural, Limb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Natural -= &Natural * Limb",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Natural.sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "(&Natural).sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Natural).sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Natural - Natural * Limb",
                &mut (|(a, b, c)| no_out!(a - b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Natural - &Natural * Limb",
                &mut (|(a, b, c)| no_out!(a - &b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).sub_mul(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "&Natural - Natural * Limb",
                &mut (|(a, b, c)| no_out!(&a - b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).sub_mul(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
            (
                "&Natural - &Natural * Limb",
                &mut (|(a, b, c)| no_out!(&a - &b * c)),
            ),
        ],
    );
}

fn benchmark_limbs_sub_mul_limb_greater(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_mul_limb_greater(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, _, _)| a.len()),
        "a.len()",
        &mut [(
            "malachite",
            &mut (|(a, b, c)| no_out!(limbs_sub_mul_limb_greater(&a, &b, c))),
        )],
    );
}

fn benchmark_limbs_sub_mul_limb_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_mul_limb_same_length_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, _, _)| a.len()),
        "a.len()",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| {
                no_out!(limbs_sub_mul_limb_same_length_in_place_left(&mut a, &b, c))
            }),
        )],
    );
}

fn benchmark_limbs_sub_mul_limb_greater_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_mul_limb_greater_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, _, _)| a.len()),
        "a.len()",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| no_out!(limbs_sub_mul_limb_greater_in_place_left(&mut a, &b, c))),
        )],
    );
}

fn benchmark_limbs_sub_mul_limb_same_length_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_mul_limb_same_length_in_place_right(&[Limb], &mut [Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, _, _)| a.len()),
        "a.len()",
        &mut [(
            "malachite",
            &mut (|(a, mut b, c)| {
                no_out!(limbs_sub_mul_limb_same_length_in_place_right(&a, &mut b, c))
            }),
        )],
    );
}

fn benchmark_limbs_sub_mul_limb_greater_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_mul_limb_greater_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, _, _)| a.len()),
        "a.len()",
        &mut [(
            "malachite",
            &mut (|(a, mut b, c)| {
                no_out!(limbs_sub_mul_limb_same_length_in_place_right(&a, &mut b, c))
            }),
        )],
    );
}
