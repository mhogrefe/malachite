// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::Factor;
use malachite_base::strings::latex::ToLatex;

#[test]
fn test_factors_to_latex() {
    let test = |n: u32, out: &str| assert_eq!(n.factor().to_latex().to_string(), out);
    // 1 has no prime factors, so its fragment is the empty product
    test(1, "1");
    test(2, "2");
    test(3, "3");
    // an exponent of 1 is left off
    test(6, r"2 \times 3");
    test(30, r"2 \times 3 \times 5");
    // and any other exponent is written
    test(4, r"2^2");
    test(8, r"2^3");
    test(12, r"2^2 \times 3");
    test(90, r"2 \times 3^2 \times 5");
    // an exponent of more than one digit is braced
    test(1024, r"2^{10}");
    test(3072, r"2^{10} \times 3");
    // a prime stands alone
    test(251, "251");
}

#[test]
fn test_factors_to_latex_multiplies_out() {
    // A fragment names the very number that was factored: reading the powers back and multiplying
    // them recovers it.
    for n in [1u32, 2, 3, 4, 6, 8, 12, 30, 90, 251, 1024, 3072, 999983, 1000000] {
        let product: u32 = n
            .factor()
            .into_iter()
            .map(|(f, e)| f.pow(u32::from(e)))
            .product();
        assert_eq!(product, n);
        let s = n.factor().to_latex().to_string();
        assert!(!s.is_empty());
        // The exponent 1 never appears, since it is left off.
        assert!(!s.contains("^1 ") && !s.ends_with("^1"));
    }
}
