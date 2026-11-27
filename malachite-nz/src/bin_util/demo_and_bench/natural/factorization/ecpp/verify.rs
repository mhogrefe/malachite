// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::factorization::ecpp::EcppStep;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::{ecpp_certificate_natural_pair_gen, ecpp_step_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_ecpp_certificate_verify);
    register_demo!(runner, demo_ecpp_step_verify);
    register_bench!(runner, benchmark_ecpp_certificate_verify);
    register_bench!(runner, benchmark_ecpp_step_verify);
}

// The outputs name FLINT's functions, as the FLINT oracle's ecpp_verify and ecpp_verify_step modes
// and Azurite's az_ecpp_cert_check mode expect.
fn demo_ecpp_certificate_verify(gm: GenMode, config: &GenConfig, limit: usize) {
    for (cert, n) in ecpp_certificate_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "ecpp_verify({}, {}) = {}",
            n,
            cert.to_pari_string(),
            cert.verify(&n)
        );
    }
}

fn demo_ecpp_step_verify(gm: GenMode, config: &GenConfig, limit: usize) {
    for step in ecpp_step_gen().get(gm, config).take(limit) {
        println!("ecpp_verify_step({:?}) = {}", step, step.verify());
    }
}

fn benchmark_ecpp_certificate_verify(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "EcppCertificate.verify(&Natural)",
        BenchmarkType::Single,
        ecpp_certificate_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(cert, n)| {
            std::hint::black_box(cert.verify(&n));
        })],
    );
}

fn benchmark_ecpp_step_verify(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "EcppStep.verify()",
        BenchmarkType::Single,
        ecpp_step_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|step: &EcppStep| usize::exact_from(step.n.significant_bits()),
            bucketing_label: "step.n.significant_bits()".to_string(),
        },
        &mut [("Malachite", &mut |step| {
            std::hint::black_box(step.verify());
        })],
    );
}
