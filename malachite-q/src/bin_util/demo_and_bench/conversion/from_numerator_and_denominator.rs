// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    pair_max_bit_bucketer, triple_1_2_max_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_pair_gen_var_6, unsigned_pair_gen_var_12, unsigned_pair_gen_var_27,
    unsigned_unsigned_bool_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::bench::bucketers::{
    pair_integer_max_bit_bucketer, pair_natural_max_bit_bucketer,
    triple_1_2_natural_max_bit_bucketer, triple_3_pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_pair_gen_var_1, integer_pair_gen_var_1_nrm, natural_natural_bool_triple_gen_var_1,
    natural_pair_gen_var_5,
};
use malachite_q::Rational;
use num::BigRational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_from_naturals);
    register_demo!(runner, demo_from_naturals_ref);
    register_unsigned_demos!(runner, demo_from_unsigneds);
    register_demo!(runner, demo_from_integers);
    register_demo!(runner, demo_from_integers_ref);
    register_signed_demos!(runner, demo_from_signeds);
    register_demo!(runner, demo_from_sign_and_naturals);
    register_demo!(runner, demo_from_sign_and_naturals_ref);
    register_unsigned_demos!(runner, demo_from_sign_and_unsigneds);
    register_demo!(runner, demo_const_from_unsigneds);
    register_demo!(runner, demo_const_from_signeds);

    register_bench!(runner, benchmark_from_naturals_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_from_unsigneds);
    register_bench!(runner, benchmark_from_integers_evaluation_strategy);
    register_bench!(runner, benchmark_from_integers_library_comparison);
    register_signed_benches!(runner, benchmark_from_signeds);
    register_bench!(runner, benchmark_from_sign_and_naturals_evaluation_strategy);
    register_unsigned_benches!(runner, benchmark_from_sign_and_unsigneds);
    register_bench!(runner, benchmark_const_from_unsigneds);
    register_bench!(runner, benchmark_const_from_signeds);
}

fn demo_from_naturals(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_naturals({}, {}) = {}",
            n_old,
            d_old,
            Rational::from_naturals(n, d)
        );
    }
}

fn demo_from_naturals_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        println!(
            "Rational::from_naturals_ref({}, {}) = {}",
            n,
            d,
            Rational::from_naturals_ref(&n, &d)
        );
    }
}

fn demo_from_unsigneds<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: From<T>,
{
    for (n, d) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Rational::from_unsigneds({}, {}) = {}",
            n,
            d,
            Rational::from_unsigneds(n, d)
        );
    }
}

fn demo_from_integers(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_integers({}, {}) = {}",
            n_old,
            d_old,
            Rational::from_integers(n, d)
        );
    }
}

fn demo_from_integers_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "Rational::from_naturals_ref({}, {}) = {}",
            n,
            d,
            Rational::from_integers_ref(&n, &d)
        );
    }
}

fn demo_from_signeds<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Integer: From<T>,
{
    for (n, d) in signed_pair_gen_var_6::<T>().get(gm, config).take(limit) {
        println!(
            "Rational::from_signeds({}, {}) = {}",
            n,
            d,
            Rational::from_signeds(n, d)
        );
    }
}

fn demo_from_sign_and_naturals(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d, sign) in natural_natural_bool_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        let d_old = d.clone();
        println!(
            "Rational::from_sign_and_naturals({}, {}, {}) = {}",
            sign,
            n_old,
            d_old,
            Rational::from_sign_and_naturals(sign, n, d)
        );
    }
}

fn demo_from_sign_and_naturals_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d, sign) in natural_natural_bool_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Rational::from_sign_and_naturals_ref({}, {}, {}) = {}",
            sign,
            n,
            d,
            Rational::from_sign_and_naturals_ref(sign, &n, &d)
        );
    }
}

fn demo_from_sign_and_unsigneds<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: From<T>,
{
    for (n, d, sign) in unsigned_unsigned_bool_triple_gen_var_2::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Rational::from_sign_and_unsigneds({}, {}, {}) = {}",
            sign,
            n,
            d,
            Rational::from_sign_and_unsigneds(sign, n, d)
        );
    }
}

fn demo_const_from_unsigneds(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in unsigned_pair_gen_var_27().get(gm, config).take(limit) {
        println!(
            "Rational::const_from_unsigneds({}, {}) = {}",
            n,
            d,
            Rational::const_from_unsigneds(n, d)
        );
    }
}

fn demo_const_from_signeds(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d) in signed_pair_gen().get(gm, config).take(limit) {
        println!(
            "Rational::const_from_signed({}, {}) = {}",
            n,
            d,
            Rational::const_from_signeds(n, d)
        );
    }
}

fn benchmark_from_naturals_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_naturals(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("n", "d"),
        &mut [
            ("from_naturals", &mut |(n, d)| {
                no_out!(Rational::from_naturals(n, d))
            }),
            ("from_naturals_ref", &mut |(n, d)| {
                no_out!(Rational::from_naturals_ref(&n, &d))
            }),
        ],
    );
}

fn benchmark_from_unsigneds<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("Rational::from_unsigneds({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "d"),
        &mut [("from_unsigneds", &mut |(n, d)| {
            no_out!(Rational::from_unsigneds(n, d))
        })],
    );
}

fn benchmark_from_integers_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_integers(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("n", "d"),
        &mut [
            ("from_integers", &mut |(n, d)| {
                no_out!(Rational::from_integers(n, d))
            }),
            ("from_integers_ref", &mut |(n, d)| {
                no_out!(Rational::from_integers_ref(&n, &d))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_from_integers_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_integers(Integer, Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("n", "d"),
        &mut [
            ("Malachite", &mut |(_, _, (n, d))| {
                no_out!(Rational::from_integers(n, d))
            }),
            ("num", &mut |((n, d), _, _)| no_out!(BigRational::new(n, d))),
            ("rug", &mut |(_, (n, d), _)| {
                no_out!(rug::Rational::from((n, d)))
            }),
        ],
    );
}

fn benchmark_from_signeds<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("Rational::from_signeds({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "d"),
        &mut [("from_unsigneds", &mut |(n, d)| {
            no_out!(Rational::from_signeds(n, d))
        })],
    );
}

fn benchmark_from_sign_and_naturals_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sign_and_naturals(bool, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_bool_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("n", "d"),
        &mut [
            ("from_sign_and_naturals", &mut |(n, d, sign)| {
                no_out!(Rational::from_sign_and_naturals(sign, n, d))
            }),
            ("from_sign_and_naturals_ref", &mut |(n, d, sign)| {
                no_out!(Rational::from_sign_and_naturals_ref(sign, &n, &d))
            }),
        ],
    );
}

fn benchmark_from_sign_and_unsigneds<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!(
            "Rational::from_sign_and_unsigneds({}, {})",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_unsigned_bool_triple_gen_var_2::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("n", "d"),
        &mut [("from_sign_and_unsigneds", &mut |(n, d, sign)| {
            no_out!(Rational::from_sign_and_unsigneds(sign, n, d))
        })],
    );
}

fn benchmark_const_from_unsigneds(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        &format!(
            "Rational::const_from_unsigneds({}, {})",
            Limb::NAME,
            Limb::NAME
        ),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "d"),
        &mut [("const_from_unsigneds", &mut |(n, d)| {
            no_out!(Rational::const_from_unsigneds(n, d))
        })],
    );
}

fn benchmark_const_from_signeds(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        &format!(
            "Rational::const_from_signeds({}, {})",
            SignedLimb::NAME,
            SignedLimb::NAME
        ),
        BenchmarkType::Single,
        signed_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "d"),
        &mut [("const_from_signeds", &mut |(n, d)| {
            no_out!(Rational::const_from_signeds(n, d))
        })],
    );
}
