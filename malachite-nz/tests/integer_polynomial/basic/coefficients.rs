// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_unsigned_pair_gen_var_1, natural_polynomial_gen,
};

#[test]
fn test_coefficients_asc() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!(p.coefficients_asc().to_debug_string(), out);
        assert_eq!(p.into_coefficients_asc().to_debug_string(), out);
    };
    test("0", "[]");
    test("1", "[1]");
    test("5", "[5]");
    test("x", "[0, 1]");
    test("x^2+3*x+2", "[2, 3, 1]");
    test("x^3", "[0, 0, 0, 1]");
}

#[test]
fn test_degree() {
    let test = |s, out| {
        assert_eq!(IntegerPolynomial::from_str(s).unwrap().degree(), out);
    };
    test("0", None);
    test("1", Some(0));
    test("5", Some(0));
    test("x", Some(1));
    test("x^2+3*x+2", Some(2));
    test("x^100", Some(100));
}

#[test]
fn test_len() {
    let test = |s, out| {
        assert_eq!(IntegerPolynomial::from_str(s).unwrap().len(), out);
    };
    test("0", 0);
    test("1", 1);
    test("-5", 1);
    test("x", 2);
    test("x^2-3*x+2", 3);
    test("-x^100", 101);
    // The constant polynomials have no coefficients, or one, or two.
    assert_eq!(IntegerPolynomial::ZERO.len(), 0);
    assert_eq!(IntegerPolynomial::one().len(), 1);
    assert_eq!(IntegerPolynomial::two().len(), 1);
    assert_eq!(IntegerPolynomial::x().len(), 2);
}

#[test]
fn test_zero_coefficients() {
    let test = |s, start, end, out| {
        let mut p = IntegerPolynomial::from_str(s).unwrap();
        p.zero_coefficients(start, end);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
    };
    test("0", 0, 5, "0");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 1, 3, "5*x^4-4*x^3+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 2, 10, "-2*x+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 2, 5, "-2*x+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 2, 4, "5*x^4-2*x+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 3, 3, "5*x^4-4*x^3+3*x^2-2*x+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 5, 10, "5*x^4-4*x^3+3*x^2-2*x+1");
    test("5*x^4-4*x^3+3*x^2-2*x+1", 0, u64::MAX, "0");
    test(
        "5*x^4-4*x^3+3*x^2-2*x+1",
        u64::MAX,
        u64::MAX,
        "5*x^4-4*x^3+3*x^2-2*x+1",
    );
    test("-x^4+x^2", 3, 5, "x^2");
    test("-x^4+x^2", 2, 5, "0");
}

#[test]
#[should_panic]
fn zero_coefficients_fail() {
    IntegerPolynomial::from_str("x^2+x+1")
        .unwrap()
        .zero_coefficients(2, 1);
}

#[test]
fn test_reverse() {
    let test = |s, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = p.reverse(len);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);

        let mut q = p;
        q.reverse_assign(len);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 3, "0");
    test("0", u64::MAX, "0");
    test("x^2-2*x+3", 0, "0");
    test("x^2-2*x+3", 1, "3");
    test("x^2-2*x+3", 2, "3*x-2");
    test("x^2-2*x+3", 3, "3*x^2-2*x+1");
    test("x^2-2*x+3", 5, "3*x^4-2*x^3+x^2");
    test("x^2-2*x", 3, "-2*x+1");
    test("x^2-2*x", 1, "0");
    test("-x^3", 4, "-1");
    test("-x^3", 6, "-x^2");
    test("-7", 1, "-7");
    test("-7", 4, "-7*x^3");
}

#[test]
fn test_truncate() {
    let test = |s, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = p.truncate(len);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);

        let mut q = p;
        q.truncate_assign(len);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 3, "0");
    test("0", u64::MAX, "0");
    test("x^3-2*x^2+3*x-4", 0, "0");
    test("x^3-2*x^2+3*x-4", 1, "-4");
    test("x^3-2*x^2+3*x-4", 2, "3*x-4");
    test("x^3-2*x^2+3*x-4", 3, "-2*x^2+3*x-4");
    test("x^3-2*x^2+3*x-4", 4, "x^3-2*x^2+3*x-4");
    test("x^3-2*x^2+3*x-4", 10, "x^3-2*x^2+3*x-4");
    test("x^3-2*x^2+3*x-4", u64::MAX, "x^3-2*x^2+3*x-4");
    test("x^3+3*x-4", 3, "3*x-4");
    test("x^5+x", 5, "x");
    test("x^5+x", 1, "0");
}

#[test]
fn test_coefficient() {
    let test = |s, i, out| {
        assert_eq!(*IntegerPolynomial::from_str(s).unwrap().coefficient(i), out);
    };
    test("0", 0, 0);
    test("0", 100, 0);
    test("x^2+3*x+2", 0, 2);
    test("x^2+3*x+2", 1, 3);
    test("x^2+3*x+2", 2, 1);
    // A coefficient past the degree is zero, however far past.
    test("x^2+3*x+2", 3, 0);
    test("x^2+3*x+2", 1000000, 0);
    test("x^2+3*x+2", u64::MAX, 0);
}

#[test]
fn test_leading_coefficient() {
    let test = |s, out| {
        assert_eq!(
            *IntegerPolynomial::from_str(s)
                .unwrap()
                .leading_coefficient(),
            out
        );
    };
    test("0", 0);
    test("5", 5);
    test("x", 1);
    test("7*x^2+3*x+2", 7);
}

#[test]
fn coefficients_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let cs = p.coefficients_asc().to_vec();
        // The coefficients are what the polynomial is: they build it back.
        let q = IntegerPolynomial::from_coefficients_asc(cs.clone());
        assert!(q.is_valid());
        assert_eq!(q, p);
        assert_eq!(p.clone().into_coefficients_asc(), cs);

        // There are as many coefficients as the degree implies, and no trailing zero among them.
        assert_eq!(
            p.degree(),
            u64::try_from(cs.len()).ok().and_then(|l| l.checked_sub(1))
        );
        assert_ne!(cs.last(), Some(&Integer::ZERO));

        // The leading coefficient is the last one, and zero when there is none.
        assert_eq!(
            *p.leading_coefficient(),
            *cs.last().unwrap_or(&Integer::ZERO)
        );
        if let Some(d) = p.degree() {
            assert_eq!(*p.leading_coefficient(), *p.coefficient(d));
            assert_ne!(*p.leading_coefficient(), 0);
            // Nothing lives past the degree.
            assert_eq!(*p.coefficient(d + 1), 0);
            assert_eq!(*p.coefficient(u64::MAX), 0);
        }

        // Indexing agrees with the slice, for every index the slice has.
        for (i, c) in cs.iter().enumerate() {
            assert_eq!(p.coefficient(u64::exact_from(i)), c);
        }
    });

    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        let c = p.coefficient(i);
        // An index is in range exactly when it is at most the degree.
        assert_eq!(
            p.degree().is_some_and(|d| i <= d),
            p.coefficients_asc().len() > usize::exact_from(i)
        );
        if p.degree().is_none_or(|d| i > d) {
            assert_eq!(*c, 0);
        }
    });
}

#[test]
fn len_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let len = p.len();
        // The length is the number of coefficients held, which is one more than the degree.
        assert_eq!(len, u64::exact_from(p.coefficients_asc().len()));
        assert_eq!(
            len,
            u64::exact_from(p.clone().into_coefficients_asc().len())
        );
        assert_eq!(len, p.degree().map_or(0, |d| d + 1));
        // Only the zero polynomial has length 0.
        assert_eq!(len == 0, p == IntegerPolynomial::ZERO);
    });

    natural_polynomial_gen().test_properties(|p| {
        // A polynomial keeps its length when its coefficients are widened to `Integer`s.
        assert_eq!(IntegerPolynomial::from(p.clone()).len(), p.len());
    });
}

#[test]
fn zero_coefficients_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        let len = p.len();
        for (start, end) in
            [(i, i), (i >> 1, i), (0, i), (i, i + 1), (i, len.max(i)), (i, u64::MAX)]
        {
            let mut q = p.clone();
            q.zero_coefficients(start, end);
            assert!(q.is_valid());

            // It agrees with zeroing the coefficients one at a time and rebuilding.
            let mut cs = p.clone().into_coefficients_asc();
            for (j, c) in cs.iter_mut().enumerate() {
                if (start..end).contains(&u64::exact_from(j)) {
                    *c = Integer::ZERO;
                }
            }
            assert_eq!(q, IntegerPolynomial::from_coefficients_asc(cs));

            // Nothing outside the range changes, and nothing inside it survives.
            for j in 0..len {
                if (start..end).contains(&j) {
                    assert_eq!(*q.coefficient(j), 0);
                } else {
                    assert_eq!(q.coefficient(j), p.coefficient(j));
                }
            }
            assert!(q.len() <= len);

            // Zeroing is idempotent.
            let mut r = q.clone();
            r.zero_coefficients(start, end);
            assert_eq!(r, q);
        }

        // An empty range changes nothing, and the whole range leaves nothing.
        let mut q = p.clone();
        q.zero_coefficients(i, i);
        assert_eq!(q, p);
        q.zero_coefficients(0, u64::MAX);
        assert_eq!(q, IntegerPolynomial::ZERO);
    });
}

#[test]
fn reverse_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        let len = p.len();
        for n in [i, len, len + i] {
            let q = p.reverse(n);
            assert!(q.is_valid());
            let mut q_alt = p.clone();
            q_alt.reverse_assign(n);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            // It agrees with truncating or padding the coefficients to n of them and reversing.
            let n_usize = usize::exact_from(n);
            let mut cs = p.clone().into_coefficients_asc();
            cs.truncate(n_usize);
            cs.resize(n_usize, Integer::ZERO);
            cs.reverse();
            assert_eq!(IntegerPolynomial::from_coefficients_asc(cs), q);

            assert!(q.len() <= n);
            for j in 0..n {
                assert_eq!(q.coefficient(j), p.coefficient(n - 1 - j));
            }

            // Reversing twice with the same length leaves the polynomial truncated to that length.
            let mut truncated = p.clone().into_coefficients_asc();
            truncated.truncate(n_usize);
            assert_eq!(
                q.reverse(n),
                IntegerPolynomial::from_coefficients_asc(truncated)
            );
        }

        // Reversing to the polynomial's own length and back gives it back.
        assert_eq!(p.reverse(len).reverse(len), p);
        assert_eq!(p.reverse(0), IntegerPolynomial::ZERO);
    });
}

#[test]
fn truncate_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        let len = p.len();
        for n in [i, i >> 1, len, len + i, u64::MAX] {
            let q = p.truncate(n);
            assert!(q.is_valid());
            let mut q_alt = p.clone();
            q_alt.truncate_assign(n);
            assert!(q_alt.is_valid());
            assert_eq!(q_alt, q);

            // It agrees with keeping the first n coefficients and rebuilding.
            let mut cs = p.clone().into_coefficients_asc();
            cs.truncate(usize::try_from(n).unwrap_or(usize::MAX));
            assert_eq!(IntegerPolynomial::from_coefficients_asc(cs), q);

            // It is zeroing every coefficient from n up.
            let mut q_alt = p.clone();
            q_alt.zero_coefficients(n, u64::MAX);
            assert_eq!(q_alt, q);

            assert!(q.len() <= n);
            assert!(q.len() <= len);
            for j in 0..len.min(n) {
                assert_eq!(q.coefficient(j), p.coefficient(j));
            }
            assert_eq!(q.truncate(n), q);
            if n >= len {
                assert_eq!(q, p);
            }
        }

        // Reversing twice in a window of length i truncates to it.
        assert_eq!(p.reverse(i).reverse(i), p.truncate(i));
        // Truncating twice keeps the shorter of the two lengths.
        assert_eq!(p.truncate(i).truncate(i >> 1), p.truncate(i >> 1));
        assert_eq!(p.truncate(0), IntegerPolynomial::ZERO);
    });
}
