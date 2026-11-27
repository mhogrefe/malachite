// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsProbablePrime;
use malachite_base::oracle_row;
use malachite_nz::natural::Natural;
use malachite_nz::natural::factorization::ecpp::{EcppCertificate, EcppStep};
use malachite_nz::test_util::generators::{ecpp_certificate_natural_pair_gen, ecpp_step_gen};
use malachite_nz::test_util::natural::factorization::ecpp::ecpp_fixtures;
use std::str::FromStr;

#[allow(clippy::too_many_arguments)]
fn step(n: &str, d: i64, a: &str, b: &str, m: &str, q: &str, x: &str, y: &str) -> EcppStep {
    EcppStep {
        n: Natural::from_str(n).unwrap(),
        d,
        a: Natural::from_str(a).unwrap(),
        b: Natural::from_str(b).unwrap(),
        m: Natural::from_str(m).unwrap(),
        q: Natural::from_str(q).unwrap(),
        x: Natural::from_str(x).unwrap(),
        y: Natural::from_str(y).unwrap(),
    }
}

#[test]
fn test_step_verify() {
    #[allow(clippy::too_many_arguments)]
    let test = |n, d, a, b, m, q, x, y, out| {
        let s = step(n, d, a, b, m, q, x, y);
        oracle_row!(
            "flint",
            "ecpp_verify_step",
            "ecpp_verify_step({s:?}) = {out}"
        );
        assert_eq!(s.verify(), out);
    };
    // - a valid step
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        true,
    );
    // - n <= 3 in EcppStep::verify
    test(
        "3",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - n even in EcppStep::verify
    test(
        "18446744073709551630",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - q <= 1 in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "1",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - q does not divide m in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473567",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - m / q <= 1 in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "206166640403",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - floor(sqrt(q)) <= 1 in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "4",
        "2",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - (floor(sqrt(q)) - 1)^4 <= n in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18",
        "9",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - 3 | n in EcppStep::verify
    test(
        "18446744073709551627",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - gcd(4a^3 + 27b^2, n) != 1 in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "0",
        "0",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - P not on the curve in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744069060473566",
        "206166640403",
        "7685973744855724010",
        "12593611317027034730",
        false,
    );
    // - m P != O in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "18446744275227113969",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - (m / q) P = O in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "4837558977367648749",
        "5069635411140069722",
        "3803103251092163651742409087098",
        "206166640403",
        "7685973744855724010",
        "12593611317027034729",
        false,
    );
    // - a, b, x, and y not reduced in EcppStep::verify
    test(
        "18446744073709551629",
        -20,
        "23284303051077200378",
        "41963123558559172980",
        "18446744069060473566",
        "206166640403",
        "26132717818565275639",
        "67933843538155689616",
        true,
    );
    // - gcd(acc, n) != 1 in EcppStep::verify
    test(
        "381919", 0, "57716", "304975", "230207", "10009", "290675", "127087", false,
    );
}

#[test]
fn test_certificate_verify() {
    let test = |n, cert, out| {
        oracle_row!("flint", "ecpp_verify", "ecpp_verify({n}, {cert}) = {out}");
        oracle_row!(
            "azurite",
            "az_ecpp_cert_check",
            "ecpp_verify({n}, {cert}) = {out}"
        );
        let n = Natural::from_str(n).unwrap();
        assert_eq!(EcppCertificate::from_str(cert).unwrap().verify(&n), out);
    };
    // - a one-step certificate
    test(
        "18446744073709551629",
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]]]",
        true,
    );
    // - the first step is for another n in EcppCertificate::verify
    test(
        "18446744073709551631",
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]]]",
        false,
    );
    // - a two-step certificate
    test(
        "68818587741598470525044465741",
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458620, 26860660642440259748266721942]], [10295533310683143210\
            451, 73075650452, 33011640000, 10256222990173086439768, [7414706640842905801292, 325414\
            1724577753829968]]]",
        true,
    );
    // - the chain is broken in EcppCertificate::verify
    test(
        "68818587741598470525044465741",
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458620, 26860660642440259748266721942]], [10295533310683143210\
            453, 73075650454, 33011640000, 10256222990173086439768, [7414706640842905801292, 325414\
            1724577753829968]]]",
        false,
    );
    // - a step fails in EcppCertificate::verify
    test(
        "68818587741598470525044465741",
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458621, 26860660642440259748266721942]], [10295533310683143210\
            451, 73075650452, 33011640000, 10256222990173086439768, [7414706640842905801292, 325414\
            1724577753829968]]]",
        false,
    );
    // - the last q has more than 64 bits in EcppCertificate::verify
    test(
        "68818587741598470525044465741",
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458620, 26860660642440259748266721942]]]",
        false,
    );
    // - the last q is composite in EcppCertificate::verify
    test(
        "18446744073709551629",
        "[[18446744073709551629, 4649078064, 2, 4837558977367648749, [7685973744855724010, 12593611\
            317027034729]]]",
        false,
    );

    // - the empty certificate in EcppCertificate::verify
    let test_empty = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(EcppCertificate::default().verify(&n), out);
    };
    test_empty("0", false);
    test_empty("1", false);
    test_empty("97", true);
    test_empty("91", false);
    test_empty("18446744073709551557", true);
    test_empty("18446744073709551629", false);
}

#[test]
fn certificate_verify_properties() {
    // A certificate proves its number prime
    ecpp_certificate_natural_pair_gen().test_properties(|(cert, n)| {
        let valid = cert.verify(&n);
        if valid {
            assert!(n.is_probable_prime());
        }
        // the text formats carry everything verification uses
        for s in [cert.to_string(), cert.to_pari_string()] {
            let cert_2 = EcppCertificate::from_str(&s).unwrap();
            assert_eq!(cert_2.verify(&n), valid);
        }
    });

    for (n, cert, _) in ecpp_fixtures() {
        assert!(cert.verify(&n));
    }
}

#[test]
fn step_verify_properties() {
    // A valid step proves n prime when q is prime
    ecpp_step_gen().test_properties(|s| {
        if s.verify() && s.q.is_probable_prime() {
            assert!(s.n.is_probable_prime());
        }
        // D is informational
        let mut s_2 = s.clone();
        s_2.d += 1;
        assert_eq!(s_2.verify(), s.verify());
    });
}
