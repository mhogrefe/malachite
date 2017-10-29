use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_pairs, lex_pairs, random_pairs};
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};
use std::cmp::Ordering;

pub fn demo_exhaustive_integer_from_sign_and_limbs_le(limit: usize) {
    for (limbs, sign) in lex_pairs(
        exhaustive_vecs(exhaustive_u::<u32>()),
        exhaustive_orderings(),
    ).filter(|&(ref limbs, sign)| {
        limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
    })
        .take(limit)
    {
        println!(
            "from_sign_and_limbs_le({:?}, {:?}) = {:?}",
            sign,
            limbs,
            gmp::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
        );
    }
}

pub fn demo_random_integer_from_sign_and_limbs_le(limit: usize) {
    for (sign, limbs) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_orderings(seed)),
        &(|seed| random_vecs(seed, 32, &(|seed_2| random_x::<u32>(seed_2)))),
    ).filter(|&(sign, ref limbs)| {
        limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
    })
        .take(limit)
    {
        println!(
            "from_sign_and_limbs_le({:?}, {:?}) = {:?}",
            sign,
            limbs,
            gmp::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
        );
    }
}

pub fn demo_exhaustive_integer_from_sign_and_limbs_be(limit: usize) {
    for (limbs, sign) in lex_pairs(
        exhaustive_vecs(exhaustive_u::<u32>()),
        exhaustive_orderings(),
    ).filter(|&(ref limbs, sign)| {
        limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
    })
        .take(limit)
    {
        println!(
            "from_sign_and_limbs_be({:?}, {:?}) = {:?}",
            sign,
            limbs,
            gmp::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
        );
    }
}

pub fn demo_random_integer_from_sign_and_limbs_be(limit: usize) {
    for (sign, limbs) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_orderings(seed)),
        &(|seed| random_vecs(seed, 32, &(|seed_2| random_x::<u32>(seed_2)))),
    ).filter(|&(sign, ref limbs)| {
        limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
    })
        .take(limit)
    {
        println!(
            "from_sign_and_limbs_be({:?}, {:?}) = {:?}",
            sign,
            limbs,
            gmp::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
        );
    }
}

pub fn benchmark_exhaustive_integer_from_sign_and_limbs_le(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from_sign_and_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(
            exhaustive_orderings(),
            exhaustive_vecs(exhaustive_u::<u32>()),
        ).filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        }),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          gmp::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
                      }),
        function_g: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          native::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_le(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_from_sign_and_limbs_le(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer::from_sign_and_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x::<u32>(seed_2)))),
        ).filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        }),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          gmp::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
                      }),
        function_g: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          native::Integer::from_sign_and_limbs_le(sign, limbs.as_slice())
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_le(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_from_sign_and_limbs_be(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from_sign_and_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_pairs(
            exhaustive_orderings(),
            exhaustive_vecs(exhaustive_u::<u32>()),
        ).filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        }),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          gmp::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
                      }),
        function_g: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          native::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_be(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_from_sign_and_limbs_be(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer::from_sign_and_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x::<u32>(seed_2)))),
        ).filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        }),
        function_f: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          gmp::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
                      }),
        function_g: &(|(sign, limbs): (Ordering, Vec<u32>)| {
                          native::Integer::from_sign_and_limbs_be(sign, limbs.as_slice())
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer::from\\\\_sign\\\\_and\\\\_limbs\\\\_be(Ordering, \\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
