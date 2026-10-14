// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::polynomial::{Evaluate, Polynomial, TaylorShift, TaylorShiftAssign};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::taylor_shift::{
    taylor_shift_divide_and_conquer, taylor_shift_horner,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_integer_triple_gen, integer_polynomial_integer_pair_gen,
    integer_polynomial_integer_pair_gen_var_2, integer_polynomial_integer_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::taylor_shift::taylor_shift_naive;

fn check(p: &IntegerPolynomial, c: &Integer) -> IntegerPolynomial {
    let q = p.taylor_shift(c);
    assert_eq!(p.clone().taylor_shift(c.clone()), q);
    assert_eq!(p.clone().taylor_shift(c), q);
    assert_eq!(p.taylor_shift(c.clone()), q);
    let mut r = p.clone();
    r.taylor_shift_assign(c);
    assert_eq!(r, q);
    let mut r = p.clone();
    r.taylor_shift_assign(c.clone());
    assert_eq!(r, q);
    let mut xs = p.coefficients_asc().to_vec();
    taylor_shift_horner(&mut xs, c);
    assert_eq!(xs, q.coefficients_asc());
    let mut xs = p.coefficients_asc().to_vec();
    taylor_shift_divide_and_conquer(&mut xs, c);
    assert_eq!(xs, q.coefficients_asc());
    q
}

#[test]
fn test_taylor_shift() {
    let test = |s, c, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        let q = check(&p, &c);
        assert_eq!(q.to_string(), out);
        assert_eq!(taylor_shift_naive(&p, &c), q);
    };
    test("0", "0", "0");
    test("0", "5", "0");
    test("7", "0", "7");
    test("7", "-4", "7");
    test("x", "3", "x+3");
    test("x", "-3", "x-3");
    // - c = 1, small coefficients
    test("x^2", "1", "x^2+2*x+1");
    // - c = -1, n <= 4
    test("x^2", "-1", "x^2-2*x+1");
    test("x^2+3*x+2", "0", "x^2+3*x+2");
    // - n <= 10, c not -1
    test("3*x^3-x+5", "2", "3*x^3+18*x^2+35*x+27");
    // - c = -1, 4 < n <= 10
    test(
        "6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "-1",
        "6*x^5-25*x^4+44*x^3-39*x^2+18*x-3",
    );
    // - c = -1, n > 10
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "-1",
        "11*x^10-100*x^9+414*x^8-1024*x^7+1673*x^6-1884*x^5+1480*x^4-800*x^3+285*x^2-60*x+6",
    );
    // - c not -1, n > 10
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "3",
        "11*x^10+340*x^9+4734*x^8+39104*x^7+212233*x^6+790908*x^5+2049800*x^4+3648736*x^3+4269981*x\
        ^2+2967212*x+930022",
    );
    test(
        "11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "-5",
        "11*x^10-540*x^9+11934*x^8-156352*x^7+1344777*x^6-7934004*x^5+32517480*x^4-91416096*x^3+168\
        705693*x^2-184552228*x+90874566",
    );
    // - c = 1, two-word sums
    test(
        "576460752303423488*x^4+3*x^3-1152921504606846976*x^2+x+1152921504606846976",
        "1",
        "576460752303423488*x^4+2305843009213693955*x^3+2305843009213693961*x^2+10*x+57646075230342\
        3492",
    );
    // - c = 1, three-limb sums
    test(
        "1361129467683753853853498429727072845834*x^10+1361129467683753853853498429727072845833*x^9\
        +1361129467683753853853498429727072845832*x^8+1361129467683753853853498429727072845831*x^7+\
        1361129467683753853853498429727072845830*x^6+1361129467683753853853498429727072845829*x^5+1\
        361129467683753853853498429727072845828*x^4+1361129467683753853853498429727072845827*x^3+13\
        61129467683753853853498429727072845826*x^2+1361129467683753853853498429727072845825*x+13611\
        29467683753853853498429727072845824",
        "1",
        "1361129467683753853853498429727072845834*x^10+14972424144521292392388482726997801304173*x^\
        9+74862120722606461961942413634989006520859*x^8+224586362167819385885827240904967019562555*\
        x^7+449172724335638771771654481809934039125055*x^6+6288418140698942804803162745339076547749\
        78*x^5+628841814069894280480316274533907654774846*x^4+4491727243356387717716544818099340391\
        24758*x^3+224586362167819385885827240904967019562280*x^2+7486212072260646196194241363498900\
        6520705*x+14972424144521292392388482726997801304119",
    );
    // - c = 1, more than three limbs
    test(
        "203703597633448608626844568840937816105146839366593625063614044935438129976333670618339736\
        5*x^11+203703597633448608626844568840937816105146839366593625063614044935438129976333670618\
        3397366*x^10+203703597633448608626844568840937816105146839366593625063614044935438129976333\
        6706183397367*x^9+2037035976334486086268445688409378161051468393665936250636140449354381299\
        763336706183397368*x^8+20370359763344860862684456884093781610514683936659362506361404493543\
        81299763336706183397369*x^7+203703597633448608626844568840937816105146839366593625063614044\
        9354381299763336706183397370*x^6+2037035976334486086268445688409378161051468393665936250636\
        140449354381299763336706183397371*x^5+20370359763344860862684456884093781610514683936659362\
        50636140449354381299763336706183397372*x^4+203703597633448608626844568840937816105146839366\
        5936250636140449354381299763336706183397373*x^3+2037035976334486086268445688409378161051468\
        393665936250636140449354381299763336706183397374*x^2+20370359763344860862684456884093781610\
        51468393665936250636140449354381299763336706183397375*x+20370359763344860862684456884093781\
        61051468393665936250636140449354381299763336706183397376",
        "1",
        "203703597633448608626844568840937816105146839366593625063614044935438129976333670618339736\
        5*x^11+244444317160138330352213482609125379326176207239912350076336853922525755971600404742\
        00768381*x^10+13444437443807608169371741543501895862939691398195179254198526965738916578438\
        0222608104226102*x^9+4481479147935869389790580514500631954313230466065059751399508988579638\
        85947934075360347420366*x^8+100833280828557061270288061576264218972047685486463844406488952\
        2430418743382851669560781695895*x^7+1613332493256912980324608985220227503552762967783421510\
        503823235888669989412562671297250713575*x^6+18822212421330651437120438160902654208115567957\
        47325095587793775203448320981323116513459166052*x^5+161333249325691298032460898522022750355\
        2762967783421510503823235888669989412562671297250714004*x^4+1008332808285570612702880615762\
        642189720476854864638444064889522430418743382851669560781696467*x^3+44814791479358693897905\
        8051450063195431323046606505975139950898857963885947934075360347420795*x^2+1344443744380760\
        81693717415435018958629396913981951792541985269657389165784380222608104226310*x+24444431716\
        013833035221348260912537932617620723991235007633685392252575597160040474200768446",
    );
    // - c = 1, few terms with large coefficients
    test(
        "1606938044258990275541962092341162602522202993782792835301376*x^3+1",
        "1",
        "1606938044258990275541962092341162602522202993782792835301376*x^3+482081413277697082662588\
        6277023487807566608981348378505904128*x^2+4820814132776970826625886277023487807566608981348\
        378505904128*x+1606938044258990275541962092341162602522202993782792835301377",
    );
    test(
        "x+3",
        "1000000000000000000000000000000",
        "x+1000000000000000000000000000003",
    );
    test(
        "x+1000000000000000000000000000000",
        "-1000000000000000000000000000000",
        "x",
    );
}

// (x + a)^n shifted by -a is x^n, a short result for a long input.
fn test_power(a: i32, n: u64, c: i32) {
    let x_plus_a = IntegerPolynomial::from_coefficients_asc(vec![Integer::from(a), Integer::ONE]);
    let p = (&x_plus_a).pow(n);
    let q = check(&p, &Integer::from(c));
    assert_eq!(q.to_string(), format!("x^{n}"));
}

#[test]
fn test_taylor_shift_long() {
    // - (x + 1)^63 shifted by -1, Horner
    test_power(1, 63, -1);
    // - length 100, divide-and-conquer dispatch but below the cutoff
    test_power(1, 99, -1);
    // - (x - 2)^150 shifted by 2
    test_power(-2, 150, 2);
    // - c = 1, length 121, blocked limb arrays
    test_power(-1, 120, 1);
    // - length 301, past the divide-and-conquer cutoff
    test_power(-1, 300, 1);
    // - (x + 3)^200 shifted by -3
    test_power(3, 200, -3);
    // - divide and conquer splits, c = -1
    test_power(1, 300, -1);
    // - divide and conquer splits, c neither 1 nor -1
    test_power(2, 500, -2);
}

#[test]
fn test_taylor_shift_huge_coefficients() {
    // - c = 1, length at least 64, coefficients too large for limb arrays
    let p = IntegerPolynomial::from_coefficients_asc(
        (0..70u32)
            .map(|i| (Integer::ONE << 7000u32) + Integer::from(i))
            .collect::<Vec<_>>(),
    );
    let q = check(&p, &Integer::ONE);
    assert_eq!(taylor_shift_naive(&p, &Integer::ONE), q);
    assert_eq!(check(&q, &Integer::NEGATIVE_ONE), p);
}

// Shifts `p` by `c`, checks the result against the binomial expansion, and checks that shifting
// back recovers `p`.
fn test_round_trip(p: &IntegerPolynomial, c: &Integer) {
    let q = check(p, c);
    assert_eq!(taylor_shift_naive(p, c), q);
    assert_eq!(check(&q, &-c), *p);
}

#[test]
fn test_taylor_shift_cutoffs() {
    // The polynomial whose coefficients are all `x`.
    let constant = |len: usize, x: Integer| IntegerPolynomial::from_coefficients_asc(vec![x; len]);
    // - word-sized coefficients, c = 1, below the two-word cutoff
    test_round_trip(&constant(100, Integer::ONE), &Integer::ONE);
    // - word-sized coefficients, c = 1, past the two-word cutoff
    test_round_trip(&constant(130, Integer::ONE), &Integer::ONE);
    // - word-sized coefficients, c = 1, at the floor of 64
    test_round_trip(&constant(70, Integer::ONE << 62u32), &Integer::ONE);
    // - word-sized coefficients, c = 3, below and past 160
    test_round_trip(&constant(130, Integer::from(5)), &Integer::from(3));
    test_round_trip(&constant(170, Integer::from(5)), &Integer::from(3));
    // - larger coefficients, c of at least a word, past 160
    test_round_trip(
        &constant(170, Integer::ONE << 70u32),
        &((Integer::ONE << 63u32) + Integer::ONE),
    );
    // - larger coefficients, smaller c, below 256
    test_round_trip(&constant(200, Integer::ONE << 70u32), &Integer::from(3));
}

fn taylor_shift_properties_helper(p: &IntegerPolynomial, c: &Integer) {
    let q = check(p, c);
    // It is the binomial expansion.
    assert_eq!(taylor_shift_naive(p, c), q);
    // The degree and the leading coefficient are unchanged.
    assert_eq!(q.len(), p.len());
    assert_eq!(q.leading_coefficient(), p.leading_coefficient());
    // Shifting back by -c recovers p.
    assert_eq!(check(&q, &-c), *p);
    // q(0) is p(c).
    assert_eq!(*q.coefficient(0), p.evaluate(c));
}

#[test]
fn taylor_shift_properties() {
    integer_polynomial_integer_pair_gen().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
        // Shifting by 0 changes nothing.
        assert_eq!(check(&p, &Integer::ZERO), p);
    });

    integer_polynomial_integer_pair_gen_var_2().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
    });

    integer_polynomial_integer_pair_gen_var_4().test_properties(|(p, c)| {
        taylor_shift_properties_helper(&p, &c);
    });

    integer_polynomial_integer_integer_triple_gen().test_properties(|(p, a, b)| {
        // Shifting by a and then by b is shifting by a + b, and q(x) evaluates as p(x + c).
        let q = (&p).taylor_shift(&a);
        assert_eq!((&q).taylor_shift(&b), (&p).taylor_shift(&a + &b));
        assert_eq!(q.evaluate(&b), p.evaluate(&a + &b));
    });
}
