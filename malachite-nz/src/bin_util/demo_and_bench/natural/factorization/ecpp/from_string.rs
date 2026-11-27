// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::factorization::ecpp::EcppCertificate;
use malachite_nz::test_util::generators::ecpp_string_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_ecpp_certificate_from_str);
    register_bench!(runner, benchmark_ecpp_certificate_from_str);
}

// The output names FLINT's function, as the FLINT oracle's ecpp_cert_set_str mode expects.
fn demo_ecpp_certificate_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in ecpp_string_gen().get(gm, config).take(limit) {
        println!(
            "ecpp_cert_set_str({:?}) = {:?}",
            s,
            EcppCertificate::from_str(&s).ok()
        );
    }
}

fn benchmark_ecpp_certificate_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "EcppCertificate::from_str(&str)",
        BenchmarkType::Single,
        ecpp_string_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| {
            std::hint::black_box(EcppCertificate::from_str(&s).ok());
        })],
    );
}
