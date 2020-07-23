use std::cmp::max;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use malachite_nz::natural::arithmetic::add_mul::{
    limbs_add_mul, limbs_add_mul_in_place_left, limbs_add_mul_limb,
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7, triples_of_unsigned_vec_var_27,
};
use malachite_test::inputs::natural::triples_of_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_add_mul_limb);
    register_demo!(
        registry,
        demo_limbs_slice_add_mul_limb_same_length_in_place_left
    );
    register_demo!(
        registry,
        demo_limbs_slice_add_mul_limb_same_length_in_place_right
    );
    register_demo!(registry, demo_limbs_vec_add_mul_limb_in_place_left);
    register_demo!(registry, demo_limbs_vec_add_mul_limb_in_place_right);
    register_demo!(registry, demo_limbs_vec_add_mul_limb_in_place_either);
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
    register_bench!(registry, Small, benchmark_limbs_add_mul_limb);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_add_mul_limb_same_length_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_add_mul_limb_same_length_in_place_right
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_add_mul_limb_in_place_left
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_add_mul_limb_in_place_right
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_add_mul_limb_in_place_either
    );
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

fn demo_limbs_add_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        println!(
            "limbs_add_mul_limb({:?}, {:?}, {}) = {:?}",
            a,
            b,
            c,
            limbs_add_mul_limb(&a, &b, c),
        );
    }
}

fn demo_limbs_slice_add_mul_limb_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm).take(limit) {
        let a_old = a.clone();
        let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, &b, c);
        println!(
            "a := {:?}; limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, {:?}, {}) = {}; \
             a = {:?}",
            a_old, b, c, carry, a,
        );
    }
}

fn demo_limbs_slice_add_mul_limb_same_length_in_place_right(gm: GenerationMode, limit: usize) {
    for (a, mut b, c) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm).take(limit) {
        let b_old = b.clone();
        let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&a, &mut b, c);
        println!(
            "b := {:?}; limbs_slice_add_mul_limb_same_length_in_place_right({:?}, &mut b, {}) \
             = {}; b = {:?}",
            b_old, a, c, carry, b,
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_left(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let a_old = a.clone();
        limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c);
        println!(
            "a := {:?}; limbs_vec_add_mul_limb_in_place_left(&mut a, {:?}, {}); a = {:?}",
            a_old, b, c, a,
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_right(gm: GenerationMode, limit: usize) {
    for (a, mut b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let b_old = b.clone();
        limbs_vec_add_mul_limb_in_place_right(&a, &mut b, c);
        println!(
            "b := {:?}; limbs_vec_add_mul_limb_in_place_right({:?}, &mut b, {}); b = {:?}",
            b_old, a, c, b,
        );
    }
}

fn demo_limbs_vec_add_mul_limb_in_place_either(gm: GenerationMode, limit: usize) {
    for (mut a, mut b, c) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, c);
        println!(
            "a := {:?}; b := {:?}; limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, {}); \
             a = {:?}; b = {:?}",
            a_old, b_old, c, a, b,
        );
    }
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

fn benchmark_limbs_add_mul_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_add_mul_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(a, b, c)| no_out!(limbs_add_mul_limb(&a, &b, c))),
        )],
    );
}

fn benchmark_limbs_slice_add_mul_limb_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_mul_limb_same_length_in_place_left(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| {
                no_out!(limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut a, &b, c
                ))
            }),
        )],
    );
}

fn benchmark_limbs_slice_add_mul_limb_same_length_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_add_mul_limb_same_length_in_place_right(&[Limb], &mut [Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(a, mut b, c)| {
                no_out!(limbs_slice_add_mul_limb_same_length_in_place_right(
                    &a, &mut b, c
                ))
            }),
        )],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_left(&mut Vec<Limb>, &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, b, c)| limbs_vec_add_mul_limb_in_place_left(&mut a, &b, c)),
        )],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_right(&[Limb], &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(a, mut b, c)| limbs_vec_add_mul_limb_in_place_right(&a, &mut b, c)),
        )],
    );
}

fn benchmark_limbs_vec_add_mul_limb_in_place_either(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_add_mul_limb_in_place_either(&mut Vec<Limb>, &mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.len(), b.len())),
        "max(a.len(), b.len())",
        &mut [(
            "malachite",
            &mut (|(mut a, mut b, c)| limbs_vec_add_mul_limb_in_place_left(&mut a, &mut b, c)),
        )],
    );
}

fn benchmark_limbs_add_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
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
    run_benchmark(
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

triple_significant_bits_fn!(Natural, bucketing_function);

fn benchmark_natural_add_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul_assign(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul_assign(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul_assign(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul_assign(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "Natural.add_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
    run_benchmark(
        "(&Natural).add_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
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
