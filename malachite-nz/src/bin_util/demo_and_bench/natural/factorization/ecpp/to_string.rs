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
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::ecpp::EcppCertificate;
use malachite_nz::test_util::generators::ecpp_certificate_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_ecpp_certificate_to_string);
    register_demo!(runner, demo_ecpp_certificate_to_pari_string);
    register_bench!(runner, benchmark_ecpp_certificate_to_string_format);
}

// The output names FLINT's function, as the FLINT oracle's ecpp_cert_get_str mode expects.
fn demo_ecpp_certificate_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for (cert, _) in ecpp_certificate_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "ecpp_cert_get_str({}, Flint) = {:?}",
            cert.to_pari_string(),
            cert.to_string()
        );
    }
}

fn demo_ecpp_certificate_to_pari_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for (cert, _) in ecpp_certificate_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("{}", cert.to_pari_string());
    }
}

fn benchmark_ecpp_certificate_to_string_format(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "EcppCertificate.to_string()",
        BenchmarkType::Algorithms,
        ecpp_certificate_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, n): &(EcppCertificate, Natural)| {
                usize::exact_from(n.significant_bits())
            },
            bucketing_label: "n.significant_bits()".to_string(),
        },
        &mut [
            ("to_string", &mut |(cert, _)| {
                std::hint::black_box(cert.to_string());
            }),
            ("to_pari_string", &mut |(cert, _)| {
                std::hint::black_box(cert.to_pari_string());
            }),
        ],
    );
}
