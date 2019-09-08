use std::cmp::max;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::arithmetic::add_mul_limb::{
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3;
use inputs::integer::triples_of_integer_integer_and_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb);
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb_in_place_left);
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb_in_place_right);
    register_demo!(
        registry,
        demo_limbs_overflowing_sub_mul_limb_in_place_either
    );
    register_demo!(registry, demo_integer_add_mul_assign_limb);
    register_demo!(registry, demo_integer_add_mul_assign_limb_ref);
    register_demo!(registry, demo_integer_add_mul_limb);
    register_demo!(registry, demo_integer_add_mul_limb_val_ref);
    register_demo!(registry, demo_integer_add_mul_limb_ref_val);
    register_demo!(registry, demo_integer_add_mul_limb_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_overflowing_sub_mul_limb);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_right
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_overflowing_sub_mul_limb_in_place_either
    );
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

fn demo_limbs_overflowing_sub_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        println!(
            "limbs_overflowing_sub_mul_limb({:?}, {:?}, {}) = {:?}",
            a,
            b,
            c,
            limbs_overflowing_sub_mul_limb(&a, &b, c),
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let a_old = a.clone();
        let borrow = limbs_overflowing_sub_mul_limb_in_place_left(&mut a, &b, c);
        println!(
            "a := {:?}; limbs_overflowing_sub_mul_limb_in_place_left(&mut a, {:?}, {}) = {}; \
             a = {:?}",
            a_old, b, c, borrow, a,
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_right(gm: GenerationMode, limit: usize) {
    for (a, mut b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let b_old = b.clone();
        let borrow = limbs_overflowing_sub_mul_limb_in_place_right(&a, &mut b, c);
        println!(
            "b := {:?}; limbs_overflowing_sub_mul_limb_in_place_right({:?}, &mut b, {}) = {}; \
             b = {:?}",
            b_old, a, c, borrow, b,
        );
    }
}

fn demo_limbs_overflowing_sub_mul_limb_in_place_either(gm: GenerationMode, limit: usize) {
    for (mut a, mut b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, c);
        println!(
            "a := {:?}; b := {:?}; \
            limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, {}); a = {:?}; b = {:?}",
            a_old, b_old, c, a, b,
        );
    }
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

fn benchmark_limbs_overflowing_sub_mul_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_overflowing_sub_mul_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(a, b, c)| no_out!(limbs_overflowing_sub_mul_limb(&a, &b, c))),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| {
                no_out!(limbs_overflowing_sub_mul_limb_in_place_left(&mut a, &b, c))
            }),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(a, mut b, c)| {
                no_out!(limbs_overflowing_sub_mul_limb_in_place_right(&a, &mut b, c))
            }),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_either(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_overflowing_sub_mul_limb_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, mut b, c)| {
                no_out!(limbs_overflowing_sub_mul_limb_in_place_either(
                    &mut a, &mut b, c
                ))
            }),
        )],
    );
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
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
