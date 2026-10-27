// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Compose, ComposeTruncated, ComposeTruncatedAssign, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_2,
    integer_polynomial_unsigned_pair_gen_var_1,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::compose_truncated::*;
use std::panic::catch_unwind;

// Checks the composition as power series of `p` and `q`, truncated to `len` coefficients, in every
// way of calling it, against each algorithm, the naive version, and the truncation of the full
// composition, and returns it.
fn verify(p: &IntegerPolynomial, q: &IntegerPolynomial, len: u64) -> IntegerPolynomial {
    let r = p.compose_truncated(q, len);
    assert!(r.is_valid());
    assert_eq!(p.clone().compose_truncated(q.clone(), len), r);
    assert_eq!(p.clone().compose_truncated(q, len), r);
    assert_eq!(p.compose_truncated(q.clone(), len), r);
    let mut s = p.clone();
    s.compose_truncated_assign(q, len);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.compose_truncated_assign(q.clone(), len);
    assert_eq!(s, r);
    assert_eq!(integer_polynomial_compose_truncated_naive(p, q, len), r);
    assert_eq!(integer_polynomial_compose_truncated_horner(p, q, len), r);
    assert_eq!(
        integer_polynomial_compose_truncated_brent_kung(p, q, len),
        r
    );
    assert_eq!(
        integer_polynomial_compose_truncated_kinoshita_li(p, q, len),
        r
    );
    assert_eq!(p.compose(q).truncate(len), r);
    r
}

#[test]
fn test_compose_truncated() {
    let test = |p, q, len, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!(verify(&p, &q, len).to_string(), out);
    };
    // - the length is 0
    test("3*x^2+2*x+1", "x", 0, "0");
    // - the outer polynomial is 0
    test("0", "x", 5, "0");
    // - the inner polynomial is 0: the constant coefficient
    test("3*x^2+2*x+1", "0", 5, "1");
    // - a constant outer polynomial
    test("7", "x^2+3*x", 5, "7");
    // - length 1: the constant coefficient
    test("6*x^2+5*x+4", "x^2+x", 1, "4");
    // - Horner's rule, with the whole composition shorter than the length
    test("3*x^2+2*x+1", "x^2+x", 10, "3*x^4+6*x^3+5*x^2+2*x+1");
    // - Horner's rule, truncating
    test("4*x^3+3*x^2+2*x+1", "x^2+x", 5, "15*x^4+10*x^3+5*x^2+2*x+1");
    // - Horner's rule, with a full product and then a truncated one
    test(
        "-243*x^5-189*x^4+229*x^3+122*x^2-123*x-13",
        "-17*x^2+9*x",
        7,
        "107846572*x^6-3192912*x^5-2150770*x^4+129609*x^3+11973*x^2-1107*x-13",
    );
    // - Brent-Kung, with more than 10 outer coefficients
    test(
        "-213*x^11-93*x^10-191*x^9-241*x^8+143*x^7-101*x^6-19*x^5-102*x^4+150*x^3+231*x^2+225*x-60",
        "228*x^3+19*x^2-225*x",
        20,
        "5007924465429109793427964265499*x^19+1796749154771025028163330449060*x^18-250912380605561\
        8589973223813443*x^17-667624546227001196444131354273*x^16+844217359401320022371505587466*x\
        ^15+147065960223495466817155353062*x^14-171612352646186802154699637412*x^13-14497915351844\
        739505364822108*x^12+15959897703745911457975422336*x^11-31126916809065962778096619*x^10+28\
        3375697591067181353459*x^9-1580440478081298383217*x^8-4168003872990237075*x^7-131079585919\
        06596*x^6+11049817122009*x^5-261005617209*x^4-1710517500*x^3+11698650*x^2-50625*x-60",
    );
    // - Brent-Kung, with the outer polynomial longer than the length
    test(
        "-9*x^29+2*x^28+x^27-3*x^26-10*x^25+4*x^24-6*x^23+x^22-15*x^21+5*x^20-2*x^19+10*x^18+6*x^1\
        7+8*x^16+16*x^15+10*x^14+3*x^13+11*x^12-3*x^10+15*x^9-8*x^8-14*x^7-10*x^6+7*x^5-8*x^4+12*x\
        ^3+9*x^2+11*x+8",
        "6*x^4-11*x^3+14*x^2+14*x",
        17,
        "64097936702360595922*x^16+4281979606616042147*x^15+177350308198567282*x^14+98124312569846\
        74*x^13+601331554271208*x^12+129160857192*x^11+1658526137448*x^10+192649647276*x^9-2292483\
        9460*x^8-1903495340*x^7-57426967*x^6+2555364*x^5-209486*x^4+36335*x^3+1918*x^2+154*x+8",
    );
    // - the inner polynomial is x: a truncation
    test(
        "-220*x^14-15*x^13+29*x^12+82*x^11+130*x^10-210*x^9-211*x^8-135*x^7+169*x^6+181*x^5+44*x^4\
        -236*x^3-102*x^2+164*x-188",
        "x",
        9,
        "-211*x^8-135*x^7+169*x^6+181*x^5+44*x^4-236*x^3-102*x^2+164*x-188",
    );
    // - a high power of x as the inner polynomial
    test(
        "91*x^11-213*x^10-97*x^9+13*x^8+42*x^7+161*x^6-54*x^5-224*x^4-146*x^3-178*x^2-249*x+61",
        "x^3",
        30,
        "-97*x^27+13*x^24+42*x^21+161*x^18-54*x^15-224*x^12-146*x^9-178*x^6-249*x^3+61",
    );
    // - Kinoshita-Li, from length 500
    test(
        "-7*x^11+2*x^10-6*x^9+6*x^8+8*x^7+2*x^6-8*x^5-3*x^4-4*x^3+3*x+6",
        "2*x^46+x^45",
        500,
        "-36960*x^499-9240*x^498-1540*x^497-154*x^496-7*x^495+2048*x^460+10240*x^459+23040*x^458+3\
        0720*x^457+26880*x^456+16128*x^455+6720*x^454+1920*x^453+360*x^452+40*x^451+2*x^450-3072*x\
        ^414-13824*x^413-27648*x^412-32256*x^411-24192*x^410-12096*x^409-4032*x^408-864*x^407-108*\
        x^406-6*x^405+1536*x^368+6144*x^367+10752*x^366+10752*x^365+6720*x^364+2688*x^363+672*x^36\
        2+96*x^361+6*x^360+1024*x^322+3584*x^321+5376*x^320+4480*x^319+2240*x^318+672*x^317+112*x^\
        316+8*x^315+128*x^276+384*x^275+480*x^274+320*x^273+120*x^272+24*x^271+2*x^270-256*x^230-6\
        40*x^229-640*x^228-320*x^227-80*x^226-8*x^225-48*x^184-96*x^183-72*x^182-24*x^181-3*x^180-\
        32*x^138-48*x^137-24*x^136-4*x^135+6*x^46+3*x^45+6",
    );
    // - a monomial inner polynomial: scaled and spread coefficients
    test(
        "-108*x^14-186*x^13+182*x^12+123*x^11-242*x^10-253*x^9-15*x^8-69*x^7+230*x^6-35*x^5+234*x^\
        4+32*x^3+172*x^2+172*x-88",
        "3*x^2",
        13,
        "167670*x^12-8505*x^10+18954*x^8+864*x^6+1548*x^4+516*x^2-88",
    );
    // - a monomial inner polynomial with coefficient 1
    test(
        "-108*x^14-186*x^13+182*x^12+123*x^11-242*x^10-253*x^9-15*x^8-69*x^7+230*x^6-35*x^5+234*x^\
        4+32*x^3+172*x^2+172*x-88",
        "x^3",
        20,
        "230*x^18-35*x^15+234*x^12+32*x^9+172*x^6+172*x^3-88",
    );
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "-108*x^14-186*x^13+182*x^12+123*x^11-242*x^10-253*x^9-15*x^8-69*x^7+230*x^6-35*x^5+234*x^\
        4+32*x^3+172*x^2+172*x-88",
        "x^5+2*x^4",
        12,
        "172*x^10+688*x^9+688*x^8+172*x^5+344*x^4-88",
    );
    // - the inner polynomial's valuation is at least the length: the constant coefficient
    test(
        "-108*x^14-186*x^13+182*x^12+123*x^11-242*x^10-253*x^9-15*x^8-69*x^7+230*x^6-35*x^5+234*x^\
        4+32*x^3+172*x^2+172*x-88",
        "x^6-x^5",
        5,
        "-88",
    );
}

#[test]
fn compose_truncated_fail() {
    // - the inner polynomial has a nonzero constant coefficient
    let p = IntegerPolynomial::from_str("x+1").unwrap();
    let q = IntegerPolynomial::from_str("x+1").unwrap();
    assert_panic!((&p).compose_truncated(&q, 3));
    let q = IntegerPolynomial::from_str("x+1").unwrap();
    assert_panic!(p.clone().compose_truncated(q, 3));
}

#[test]
#[should_panic]
fn compose_truncated_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.compose_truncated_assign(&IntegerPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn compose_truncated_properties() {
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_2().test_properties(
        |(p, q, len)| {
            let r = verify(&p, &q, len);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).compose_truncated(&q, len),
                (&p).compose_truncated((&q).compose_truncated(&q, len), len)
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(p.truncate(len).compose_truncated(q.truncate(len), len), r);
        },
    );

    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        // Composing with x truncates.
        assert_eq!(verify(&p, &IntegerPolynomial::x(), len), p.truncate(len));
        // Composing with 0 gives the constant coefficient.
        assert_eq!(
            verify(&p, &IntegerPolynomial::ZERO, len),
            IntegerPolynomial::from(p.coefficient(0).clone()).truncate(len)
        );
        // Composing x with a polynomial truncates it.
        if p != IntegerPolynomial::ZERO && *p.coefficient(0) == 0u32 {
            assert_eq!(verify(&IntegerPolynomial::x(), &p, len), p.truncate(len));
        }
    });
}
