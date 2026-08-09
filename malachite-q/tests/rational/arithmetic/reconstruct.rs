// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivisibleBy, FloorSqrt, Gcd, ModInverse, Parity, Pow,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    natural_pair_gen_var_1, natural_quadruple_gen_var_1, rational_gen,
};
use std::str::FromStr;

#[test]
fn test_reconstruct() {
    let test = |a, m, out| {
        let a = Natural::from_str(a).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(format!("{:?}", Rational::reconstruct_ref(&a, &m)), out);
        assert_eq!(format!("{:?}", Rational::reconstruct(a, m)), out);
    };
    // - a <= N (zero)
    test("0", "3", "Some(0)");
    // - a <= N (positive integer)
    test("1", "4", "Some(1)");
    // - m - a <= N (negative integer)
    test("2", "3", "Some(-1)");
    // - denominator bound exceeded
    test("2", "4", "None");
    // - genuine fraction
    test("33", "97", "Some(2/3)");
    // - 1/25 is a solution for looser bounds, but not for the balanced ones
    test("444", "1009", "None");
    // - multi-limb success
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "Some(22/7)",
    );
    test(
        "3527336853615520289241622543",
        "12345678987654321012345678901",
        "Some(-1/7)",
    );
    // - multi-limb negative integer fast path
    test(
        "12345678987654321012345678884",
        "12345678987654321012345678901",
        "Some(-17)",
    );
    // - multi-limb failure
    test(
        "4052555153018976267",
        "12345678987654321012345678901",
        "None",
    );
    // - gcd(n, d) != 1 with a nonzero remainder
    test("4", "10", "None");
    // - modulus at the one-limb boundary on 64-bit platforms (two limbs on 32-bit)
    test("2635249153387078797", "18446744073709551557", "Some(22/7)");
    test("6048575297970826480", "18446744073709551557", "None");
    // - modulus too wide for the word kernels on any platform
    test(
        "229562577751284325077423156048737514646028999111827547900189",
        "1606938044258990275541962092341162602522202993782792835301301",
        "Some(22/7)",
    );
    // - modulus at the top of the array-kernel range (twelve limbs on 64-bit platforms)
    test(
        "83170612087537978668695329739062636888761750916965889882449216322824158179627333196297057\
        615820347033061842715917736064616334645220436485255943961162920418171667230858259816368764\
        211730264202401353374956806239397369259652883003055",
        "19406476153758861689362243605781281940711075213958707639238150475325636908579711079135980\
        110358080974381096633714138415077144750551435179893053590938014764240055687200260623819378\
        3160703949805603157874899214558593861605856727007121",
        "Some(22/7)",
    );
    // - modulus just past the array-kernel range (thirteen limbs on 64-bit platforms)
    test(
        "47628674520570387671998941362294698551033988016542201145687555798314551698697371939438801\
        434796816362449499361381666003184920011815138322002595400373791643186393982710814389703307\
        70204222724303377832154382776144272636248598197472350831340911",
        "66680144328798542740798517907212577971447583223159081603962578117640372378176320715214322\
        008715542907429299105934332404458888016541193650803633560523308300460951575795140145584630\
        78285911814024728965016135886601981690748037476461291163877271",
        "Some(22/7)",
    );
    // - a 2000-bit modulus, deep in the Lehmer range
    test(
        "82009335376732466016630942941262998858736978720621085748403052630411875813740736704047106\
        165464733565603283188499104483817651354347361095101852239726189513242235706532365914286206\
        270820171460622449233564331061603415439616890277402819512161457475967122074997541544570577\
        817592575445222606811545872835951915245834934988898932573128203434224327786381983214592144\
        797647589878944291523025915208400905630940861856425283155805015557041717172971323702322529\
        206738501494228967826988278337170828629969440450416253843161131232868234534847442451467506\
        368054131344732268168234857730601979155689609830132036535020941",
        "11481306952742545242328332011776819840223177020886952004776427368257662613923703138566594\
        863165062699184459646389874627734471189608630553314259313561666531853912998914531228000068\
        877914824004487142892699006348624478161546364638836394731702604046635397090499655816239880\
        894462960562331164953616422197033268134416890898445850560237948480791405890093477650042900\
        271670662583052200813223628129176126788331720659899539641812702177985840404215985318325154\
        088943390209192055495778358967203916008195721663058275538042558372601552834878641943205450\
        8915275783882625175435528800822842770817965453762184851149029313",
        "Some(22/7)",
    );
}

#[test]
fn test_reconstruct_with_bounds() {
    let test = |a, m, n_bound, d_bound, out| {
        let a = Natural::from_str(a).unwrap();
        let m = Natural::from_str(m).unwrap();
        let n_bound = Natural::from_str(n_bound).unwrap();
        let d_bound = Natural::from_str(d_bound).unwrap();
        assert_eq!(
            format!(
                "{:?}",
                Rational::reconstruct_with_bounds_ref(&a, &m, &n_bound, &d_bound)
            ),
            out
        );
        assert_eq!(
            format!(
                "{:?}",
                Rational::reconstruct_with_bounds(a, m, &n_bound, &d_bound)
            ),
            out
        );
    };
    // - a <= N (zero)
    test("0", "5", "1", "1", "Some(0)");
    // - m - a <= N
    test("4", "5", "1", "1", "Some(-1)");
    // - balanced bounds
    test("33", "97", "6", "6", "Some(2/3)");
    // - asymmetric bounds
    test("33", "97", "2", "30", "Some(2/3)");
    // - asymmetric bounds succeed where the balanced ones fail
    test("444", "1009", "1", "30", "Some(1/25)");
    // - flipped asymmetric bounds fail
    test("444", "1009", "30", "1", "None");
    // - the loop ends with a zero remainder, so the gcd check requires d = 1 and fails
    test("2", "10", "1", "5", "None");
    // - tight bounds at the one-limb boundary on 64-bit platforms
    test(
        "2635249153387078797",
        "18446744073709551557",
        "22",
        "7",
        "Some(22/7)",
    );
    test(
        "2635249153387078797",
        "18446744073709551557",
        "21",
        "7",
        "None",
    );
    // - non-unique regime (2 * N * D >= m): pinned outputs, which agree with FLINT's reference
    //   implementation
    test("5", "11", "5", "5", "Some(5)");
    test("7", "11", "5", "5", "Some(-4)");
    // - non-unique regime with a two-limb m and a three-limb d_bound: FLINT 3.6.0's two-limb kernel
    //   misreads the bound here (fmpz_get_uiui drops limbs beyond the second) and spuriously fails,
    //   disagreeing with its own reference implementation, which returns this
    test(
        "778029533528",
        "39510926782646445715540418031384",
        "32098388",
        "157980302531428379809519276806140673520080351",
        "Some(21455872/8785515252104296477493)",
    );
    // - a word window lands B at or below N directly (the Lehmer lucky finish)
    test(
        "55342368964022834690839453240591532760368525382209141533507898859499473655258466041616426\
        930698809813545578709058541569640445755603312260956350004321318482279991392867325596474191\
        836945875503199332493719988886678191963247213103535103585462471664206777364337335344195249\
        28",
        "76986668302532326870893132111865480170790727591240687947546124708230213557270004529889293\
        813411259644160641016154334415750657393397237481606538002212872355659016607963138944343528\
        212899790595637095147879625933199058185050275226899968537700342708456604586304475667459858\
        01",
        "10300262336800850607124150785475061357099008046312634293839474793571350412351114298018129\
        258138538302796648450300569510541695",
        "76986668302532326870893132111865480170790727591240687947546124708230213557270004529889293\
        813411259644160641016154334415750657393397237481606538002212872355659016607963138944343528\
        212899790595637095147879625933199058185050275226899968537700342708456604586304475667459858\
        01",
        "Some(-98104735215333403172800089401665982120766731438395399168740629431276122541343487074\
        48183393971437598713438174856895837764816/139384045672386205382901851801614666162854093863\
        145676269595737248694779507351023011691377451765755462589624599413090422401158557728050224\
        591054243)",
    );
    // - the modulus and numerator bound are within two limbs of each other, so the plain Euclidean
    //   loop runs without Lehmer acceleration
    test(
        "47628674520570387671998941362294698551033988016542201145687555798314551698697371939438801\
        434796816362449499361381666003184920011815138322002595400373791643186393982710814389703307\
        70204222724303377832154382776144272636248598197472350831340911",
        "66680144328798542740798517907212577971447583223159081603962578117640372378176320715214322\
        008715542907429299105934332404458888016541193650803633560523308300460951575795140145584630\
        78285911814024728965016135886601981690748037476461291163877271",
        "24494416553286712184739252007008198022611158913109329806167077753081601831582751228072059\
        044044411048418762948625619605709889122518635372925213606891463887179061880666429384603126\
        27143172697498123763711",
        "66680144328798542740798517907212577971447583223159081603962578117640372378176320715214322\
        008715542907429299105934332404458888016541193650803633560523308300460951575795140145584630\
        78285911814024728965016135886601981690748037476461291163877271",
        "Some(22/7)",
    );
    // - multi-limb, tightest bounds that still succeed
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "22",
        "7",
        "Some(22/7)",
    );
    // - either bound one tighter fails
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "21",
        "7",
        "None",
    );
    test(
        "8818342134038800723104056361",
        "12345678987654321012345678901",
        "22",
        "6",
        "None",
    );
}

#[test]
#[should_panic]
fn reconstruct_fail_small_m() {
    Rational::reconstruct(Natural::ONE, Natural::TWO);
}

#[test]
#[should_panic]
fn reconstruct_fail_unreduced_a() {
    Rational::reconstruct(Natural::from(5u32), Natural::from(4u32));
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_unreduced_a() {
    Rational::reconstruct_with_bounds(
        Natural::from(5u32),
        Natural::from(4u32),
        &Natural::ONE,
        &Natural::ONE,
    );
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_zero_n_bound() {
    Rational::reconstruct_with_bounds(
        Natural::ONE,
        Natural::from(10u32),
        &Natural::ZERO,
        &Natural::ONE,
    );
}

#[test]
#[should_panic]
fn reconstruct_with_bounds_fail_zero_d_bound() {
    Rational::reconstruct_with_bounds(
        Natural::ONE,
        Natural::from(10u32),
        &Natural::ONE,
        &Natural::ZERO,
    );
}

#[test]
fn test_reconstruct_split_tier() {
    // The subquadratic splitter engages when the modulus-to-bound limb gap reaches 500. The inputs
    // are constructed rather than written out, and uniqueness under 2 * N * D < m pins the expected
    // values without a reference computation.
    //
    // A tiny numerator and denominator make the continued fraction of a/m a handful of terms around
    // one astronomical quotient, which the half-gcd hands to a single division.
    let m = (Natural::ONE << 66000u64) - Natural::from(63u32);
    let a = (Natural::from(22u32) * Natural::from(7u32).mod_inverse(&m).unwrap()) % &m;
    assert_eq!(
        Rational::reconstruct_ref(&a, &m).unwrap(),
        Rational::from_signeds(22, 7)
    );
    // A half-size numerator and denominator make the continued fraction generically long, which
    // drives the recursion to full depth.
    let n = Natural::from(3u32).pow(10405);
    let d = Natural::from(5u32).pow(7099);
    let a = (&n * (&d).mod_inverse(&m).unwrap()) % &m;
    assert_eq!(
        Rational::reconstruct_ref(&a, &m).unwrap(),
        Rational::from_naturals(n, d)
    );
    // A residue just below m, with the difference under one limb, makes the truncated tops of the
    // first splitter iteration equal, so it falls to a plain division.
    let a = &m - ((Natural::ONE << 60u64) + Natural::from(3u32));
    let bound = Natural::from(999983u32);
    assert_eq!(
        Rational::reconstruct_with_bounds_ref(&a, &m, &bound, &bound),
        None
    );
    // A numerator bound just below the half-size point where a splitter window lands lets the
    // window reach B <= N directly, while the gap is still above the cutoff.
    let n = Natural::from(3u32).pow(20870);
    let d = Natural::from(5u32).pow(14170);
    let a = (&n * (&d).mod_inverse(&m).unwrap()) % &m;
    assert_eq!(
        Rational::reconstruct_with_bounds_ref(
            &a,
            &m,
            &(Natural::ONE << 33090u64),
            &(Natural::ONE << 32908u64),
        )
        .unwrap(),
        Rational::from_naturals(n, d)
    );
    let m = (Natural::ONE << 33100u64) - Natural::from(121u32);
    let a = (Natural::from(12345u32) * Natural::from(617u32).mod_inverse(&m).unwrap()) % &m;
    let bound = Natural::from(999983u32);
    assert_eq!(
        Rational::reconstruct_with_bounds_ref(&a, &m, &bound, &bound).unwrap(),
        Rational::from_signeds(12345, 617)
    );
}

fn balanced_bound(m: &Natural) -> Natural {
    let mut b = m >> 1u32;
    if m.even() {
        b -= Natural::ONE;
    }
    b.floor_sqrt()
}

// Verifies that a returned rational actually satisfies the reconstruction constraints.
fn check_solution(x: &Rational, a: &Natural, m: &Natural, n_bound: &Natural, d_bound: &Natural) {
    assert!(x.is_valid());
    assert!(x.to_numerator() <= *n_bound);
    assert!(x.to_denominator() <= *d_bound);
    let signed_num = Integer::from_sign_and_abs(*x >= 0u32, x.to_numerator());
    let diff = signed_num - Integer::from(x.to_denominator()) * Integer::from(a);
    assert!(diff.divisible_by(Integer::from(m)));
}

// Exhaustively verifies that no solution exists. Only valid in the unique regime 2 * N * D < m,
// which also guarantees the loop is short.
fn assert_no_solution(a: u64, m: u64, n_bound: u64, d_bound: u64) {
    for d in 1..=d_bound {
        let n0 = a * d % m;
        assert!(!(n0 <= n_bound && n0.gcd(d) == 1));
        assert!(!(m - n0 <= n_bound && (m - n0).gcd(d) == 1));
    }
}

#[test]
fn reconstruct_properties() {
    natural_pair_gen_var_1().test_properties(|(a, m)| {
        let ox = Rational::reconstruct_ref(&a, &m);
        assert_eq!(Rational::reconstruct(a.clone(), m.clone()), ox);
        let b = balanced_bound(&m);
        assert_eq!(
            Rational::reconstruct_with_bounds_ref(&a, &m, &b, &b),
            ox,
            "balanced bounds disagree"
        );
        if let Some(x) = ox {
            check_solution(&x, &a, &m, &b, &b);
        } else if m <= 1_000u32 {
            // The balanced bounds always satisfy 2 * N * D < m.
            assert_no_solution(
                u64::exact_from(&a),
                u64::exact_from(&m),
                u64::exact_from(&b),
                u64::exact_from(&b),
            );
        }
    });

    natural_quadruple_gen_var_1().test_properties(|(a, m, n_bound, d_bound)| {
        let ox = Rational::reconstruct_with_bounds_ref(&a, &m, &n_bound, &d_bound);
        assert_eq!(
            Rational::reconstruct_with_bounds(a.clone(), m.clone(), &n_bound, &d_bound),
            ox
        );
        if let Some(x) = ox {
            check_solution(&x, &a, &m, &n_bound, &d_bound);
        } else if m <= 1_000u32 && (&n_bound * &d_bound) << 1u32 < m {
            assert_no_solution(
                u64::exact_from(&a),
                u64::exact_from(&m),
                u64::exact_from(&n_bound),
                u64::exact_from(&d_bound),
            );
        }
    });

    rational_gen().test_properties(|x| {
        // Round trip: reduce x modulo a large enough coprime modulus, then reconstruct it.
        let n = x.to_numerator();
        let d = x.to_denominator();
        let k = x.to_height();
        let mut m = ((&k * &k) << 1u32) + Natural::ONE;
        while (&d).gcd(&m) != 1u32 {
            m += Natural::ONE;
        }
        let inv = (&d).mod_inverse(&m).unwrap();
        let mut a = &n * inv % &m;
        if x < 0u32 {
            a = &m - a;
        }
        assert_eq!(Rational::reconstruct_ref(&a, &m), Some(x.clone()));
        let n_bound = if n == 0u32 { Natural::ONE } else { n };
        assert_eq!(
            Rational::reconstruct_with_bounds(a, m, &n_bound, &d),
            Some(x)
        );
    });
}
