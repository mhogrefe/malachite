use std::cmp::max;

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use malachite_nz::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::integer::Integer;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
    triples_of_unsigned_vec_var_29,
};
use malachite_test::inputs::integer::triples_of_integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb);
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb_in_place_left);
    register_demo!(registry, demo_limbs_overflowing_sub_mul_limb_in_place_right);
    register_demo!(
        registry,
        demo_limbs_overflowing_sub_mul_limb_in_place_either
    );
    register_demo!(registry, demo_limbs_overflowing_sub_mul);
    register_demo!(registry, demo_limbs_overflowing_sub_mul_in_place_left);
    register_demo!(registry, demo_integer_sub_mul_assign);
    register_demo!(registry, demo_integer_sub_mul_assign_val_ref);
    register_demo!(registry, demo_integer_sub_mul_assign_ref_val);
    register_demo!(registry, demo_integer_sub_mul_assign_ref_ref);
    register_demo!(registry, demo_integer_sub_mul);
    register_demo!(registry, demo_integer_sub_mul_val_val_ref);
    register_demo!(registry, demo_integer_sub_mul_val_ref_val);
    register_demo!(registry, demo_integer_sub_mul_val_ref_ref);
    register_demo!(registry, demo_integer_sub_mul_ref_ref_ref);
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
    register_bench!(registry, Small, benchmark_limbs_overflowing_sub_mul);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_overflowing_sub_mul_in_place_left
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_sub_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_assign_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_sub_mul_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_val_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_val_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_val_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_sub_mul_ref_ref_ref_algorithms
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

fn demo_limbs_overflowing_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_unsigned_vec_var_29(gm).take(limit) {
        println!(
            "limbs_overflowing_sub_mul({:?}, {:?}, {:?}) = {:?}",
            a,
            b,
            c,
            limbs_overflowing_sub_mul(&a, &b, &c),
        );
    }
}

fn demo_limbs_overflowing_sub_mul_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_unsigned_vec_var_29(gm).take(limit) {
        let a_old = a.clone();
        let sign = limbs_overflowing_sub_mul_in_place_left(&mut a, &b, &c);
        println!(
            "a := {:?}; limbs_overflowing_sub_mul_in_place_left(&mut a, {:?}, {:?}) = {}; a = {:?}",
            a_old, b, c, sign, a,
        );
    }
}

fn demo_integer_sub_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c_old, a
        );
    }
}

fn demo_integer_sub_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, &c);
        println!(
            "a := {}; x.sub_mul_assign({}, &{}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_integer_sub_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.sub_mul_assign(&b, c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, {}); x = {}",
            a_old, b, c_old, a
        );
    }
}

fn demo_integer_sub_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_integer_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.sub_mul(b, c)
        );
    }
}

fn demo_integer_sub_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, &c)
        );
    }
}

fn demo_integer_sub_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.sub_mul(&b, c)
        );
    }
}

fn demo_integer_sub_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, &{}) = {}", a_old, b, c, a.sub_mul(&b, &c));
    }
}

fn demo_integer_sub_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integers(gm).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

fn benchmark_limbs_overflowing_sub_mul_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_overflowing_sub_mul_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "Malachite",
            &mut (|(a, b, c)| no_out!(limbs_overflowing_sub_mul_limb(&a, &b, c))),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul_limb_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_overflowing_sub_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "Malachite",
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
    run_benchmark_old(
        "limbs_overflowing_sub_mul_limb_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "Malachite",
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
    run_benchmark_old(
        "limbs_overflowing_sub_mul_limb_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "Malachite",
            &mut (|(mut a, mut b, c)| {
                no_out!(limbs_overflowing_sub_mul_limb_in_place_either(
                    &mut a, &mut b, c
                ))
            }),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_overflowing_sub_mul(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_29(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, ref c)| max!(a.len(), b.len(), c.len())),
        "max(a.len(), b.len(), c.len())",
        &mut [(
            "Malachite",
            &mut (|(a, b, c)| no_out!(limbs_overflowing_sub_mul(&a, &b, &c))),
        )],
    );
}

fn benchmark_limbs_overflowing_sub_mul_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_overflowing_sub_mul_in_place_left(&mut Vec<Limb>, &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_29(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, ref c)| max!(a.len(), b.len(), c.len())),
        "max(a.len(), b.len(), c.len())",
        &mut [(
            "Malachite",
            &mut (|(mut a, b, c)| no_out!(limbs_overflowing_sub_mul_in_place_left(&mut a, &b, &c))),
        )],
    );
}

triple_significant_bits_fn!(Integer, bucketing_function);

fn benchmark_integer_sub_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul_assign(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul_assign(Integer, Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer.sub_mul_assign(Integer, &Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, &c)),
            ),
            (
                "Integer.sub_mul_assign(&Integer, Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Integer.sub_mul_assign(&Integer, &Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, &c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.sub_mul_assign(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul_assign(Integer, Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Integer -= Integer * Integer",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul_assign(Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul_assign(Integer, &Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, &c)),
            ),
            (
                "Integer -= Integer * &Integer",
                &mut (|(mut a, b, c)| a -= b * &c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul_assign(&Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul_assign(&Integer, Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Integer -= &Integer * Integer",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_assign_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul_assign(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul_assign(&Integer, &Integer)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, &c)),
            ),
            (
                "Integer -= &Integer * &Integer",
                &mut (|(mut a, b, c)| a -= &b * &c),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul(Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer.sub_mul(Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, &c))),
            ),
            (
                "Integer.sub_mul(&Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Integer.sub_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, &c))),
            ),
            (
                "(&Integer).sub_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, &c))),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.sub_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul(Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Integer - Integer * Integer",
                &mut (|(a, b, c)| no_out!(a - b * c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_val_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul(Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul(Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, &c))),
            ),
            (
                "Integer - Integer * &Integer",
                &mut (|(a, b, c)| no_out!(a - b * &c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_val_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul(&Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul(&Integer, Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Integer - &Integer * Integer",
                &mut (|(a, b, c)| no_out!(a - &b * c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_val_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.sub_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Integer.sub_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, &c))),
            ),
            (
                "Integer - &Integer * &Integer",
                &mut (|(a, b, c)| no_out!(a - &b * &c)),
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_ref_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "(&Integer).sub_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "(&Integer).sub_mul(&Integer, &Integer)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, &c))),
            ),
            (
                "(&Integer) - &Integer * &Integer",
                &mut (|(a, b, c)| no_out!((&a) - &b * &c)),
            ),
        ],
    );
}
