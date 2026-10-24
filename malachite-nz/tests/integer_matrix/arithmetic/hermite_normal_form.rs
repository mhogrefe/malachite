// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ExtendedGcd, HermiteNormalForm, HermiteNormalFormAssign, Rank,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form::*;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form_pernet_stein::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::common::has_full_column_rank;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4, integer_pair_gen,
};

// The product of the pivots of `h`, a Hermite normal form with a pivot in every column: the
// determinant of the lattice spanned by its rows.
fn lattice_determinant(h: &IntegerMatrix) -> Natural {
    (0..usize::try_from(h.width()).unwrap())
        .map(|i| Natural::try_from(&h[(i, i)]).unwrap())
        .product()
}

// Checks the Hermite normal form H of `a`: the ways of computing it agree, including, when `a` has
// full column rank, the minors-based and modular algorithms; H is in Hermite normal form, is its
// own Hermite normal form, has the rank of `a`, and is unchanged by negating `a` or reversing its
// rows. Returns it.
fn verify_hnf(a: &IntegerMatrix) -> String {
    let h = a.hermite_normal_form();
    assert!(h.is_valid());
    assert_eq!(a.clone().hermite_normal_form(), h);
    let mut b = a.clone();
    b.hermite_normal_form_assign();
    assert_eq!(b, h);
    assert_eq!(h.height(), a.height());
    assert_eq!(h.width(), a.width());
    assert!(h.is_in_hermite_normal_form());
    assert_eq!(hnf_classical(a), h);
    // The xgcd algorithm's intermediate elements grow quickly, so it is only cross-checked on small
    // matrices.
    if a.height() <= 4 {
        assert_eq!(hnf_xgcd(a), h);
    }
    assert_eq!(hnf_pernet_stein(a), h);
    assert_eq!(hnf_pernet_stein_without_direct_extension(a), h);
    if has_full_column_rank(a) {
        assert_eq!(hnf_minors(a), h);
        let d = lattice_determinant(&h);
        assert_eq!(hnf_modular(a, &d), h);
        let mut e = a.clone();
        hnf_modular_eldiv(&mut e, &d);
        assert_eq!(e, h);
        let d = d * Natural::from(6u32);
        assert_eq!(hnf_modular(a, &d), h);
        let mut e = a.clone();
        hnf_modular_eldiv(&mut e, &d);
        assert_eq!(e, h);
    }
    assert_eq!((&h).hermite_normal_form(), h);
    assert_eq!((&h).rank(), a.rank());
    assert_eq!((-a).hermite_normal_form(), h);
    let mut reversed = a.clone();
    let m = a.height();
    for i in 0..m >> 1 {
        reversed.swap_rows(i, m - 1 - i);
    }
    assert_eq!(reversed.hermite_normal_form(), h);
    h.to_string()
}

// Checks the extended GCD (d, a, b) of f and g: d is the GCD, af + bg = d, and, when |f| ≠ |g|
// and neither is 0, the cofactor of the one with the smaller absolute value is smaller than the
// other in absolute value. Returns it.
fn verify_extended_gcd(f: &Integer, g: &Integer) -> String {
    let (d, a, b) = extended_gcd_reduced_cofactor(f, g);
    assert_eq!(d, Integer::from(f.clone().extended_gcd(g.clone()).0));
    assert_eq!(&a * f + &b * g, d);
    let (f_abs, g_abs) = (f.unsigned_abs_ref(), g.unsigned_abs_ref());
    if *f != 0u32 && *g != 0u32 && f_abs != g_abs {
        if f_abs < g_abs {
            assert!(a.unsigned_abs_ref() < g_abs);
        } else {
            assert!(b.unsigned_abs_ref() < f_abs);
        }
    }
    format!("({d}, {a}, {b})")
}

#[test]
fn test_hermite_normal_form() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_hnf(&a), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{2,0}", "[]_{2,0}");
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]");
    test("[3]", "[3]");
    test("[-3]", "[3]");
    test("[1, 2; 3, 4]", "[1, 0; 0, 2]");
    test("[2, 4; 1, 2]", "[1, 2; 0, 0]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1, 2, 3; 0, 3, 6; 0, 0, 0]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1, 2, 0; 0, 3, 0; 0, 0, 1]",
    );
    test("[0, 2, 1; 0, 4, 3]", "[0, 2, 0; 0, 0, 1]");
    test("[1, 2, 3, 4]", "[1, 2, 3, 4]");
    test("[1; 2; 3]", "[1; 0; 0]");
    test("[2, 1; 4, 2; 6, 3]", "[2, 1; 0, 0; 0, 0]");
    test("[6, 4; 4, 6; 9, 3]", "[1, 1; 0, 2; 0, 0]");
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "[1, 0, 0, 0; 0, 2, 0, 4; 0, 0, 1, 0; 0, 0, 0, 0]",
    );
    test(
        "[0, 0, 1, 2; 0, 0, 2, 4; 1, 1, 0, 0; 2, 2, 1, 2; 3, 3, 1, 2]",
        "[1, 1, 0, 0; 0, 0, 1, 2; 0, 0, 0, 0; 0, 0, 0, 0; 0, 0, 0, 0]",
    );
    test(
        "[1267650600228229401496703205376, 1; 2535301200456458802993406410752, 3]",
        "[1267650600228229401496703205376, 0; 0, 1]",
    );
    // - hnf_minors swaps in a row when a diagonal element is zero
    test("[0, 1; 1, 0]", "[1, 0; 0, 1]");
    // - hnf_pernet_stein finds rank 0 modulo the first prime, for both limb widths, and tries the
    //   next
    test(
        "[77371269172698101563002583, 154742538345396203126005166]",
        "[77371269172698101563002583, 154742538345396203126005166]",
    );
    // - hnf_pernet_stein finds the wrong pivots modulo the first prime, for both limb widths, and
    //   tries the next
    test(
        "[77371269172698101563002583, 1, 0, 0; 0, 0, 1, 0; 0, 0, 0, 1]",
        "[77371269172698101563002583, 1, 0, 0; 0, 0, 1, 0; 0, 0, 0, 1]",
    );
    // - double_det computes both determinants directly when the last solution element is zero
    test(
        "[1, 0, 0, 0; 0, 1, 0, 0; 2, 0, 1, 0]",
        "[1, 0, 0, 0; 0, 1, 0, 0; 0, 0, 1, 0]",
    );
    // - hermite_normal_form uses hnf_pernet_stein, which uses hnf_minors
    test(
        "[4149515568880992958512407863691161151012446232242436899995657329690652811412908146399707\
        048947103794288197886611300789182395151075411775307886874834113963687061181803401509523685\
        383, 1; 6224273353321489437768611795536741726518669348363655349993485994535979217119362219\
        599560573420655691432296829916951183773592726613117662961830312251170945530591772705102264\
        285528075, 5]",
        "[1, 7; 0, 1452330449108347535479342752291906402854356181284852914998480065391728483994517\
        851239897467131486328000869260313955276213838302876394121357760406191939887290471413631190\
        5283332898840]",
    );
    // - hermite_normal_form uses hnf_pernet_stein on a wide matrix
    test(
        "[4149515568880992958512407863691161151012446232242436899995657329690652811412908146399707\
        048947103794288197886611300789182395151075411775307886874834113963687061181803401509523685\
        383, 1, 2, 3; 6224273353321489437768611795536741726518669348363655349993485994535979217119\
        362219599560573420655691432296829916951183773592726613117662961830312251170945530591772705\
        102264285528075, 5, 7, 11; 13, 41495155688809929585124078636911611510124462322424368999956\
        573296906528114129081463997070489471037942881978866113007891823951510754117753078868748341\
        13963687061181803401509523685383, 17, 19]",
        "[1, 3, 8338508919130206274158415613811653661859460411317919018701001477124887001418958570\
        400039681554563241151261388462019133708937354725741401880027383402327743619740765832185605\
        646717735593610902762118485941658322331721318888131633948984717396798176738672970623926406\
        184553971798311715304940857653743041525171066921170076882931743209690025129988976334550251\
        705115688, 1355007699358658519550742537244393720052162316839161840538912740032794137730580\
        767690006448252616526687079975625078109227702320142932977805504449802878258338207874447730\
        160917591633556796470203764478372416801175547925645577331388307953195865542726494943833061\
        141705629365375260158431360574859093188941399132070751976613690985125741157915121831241314\
        3900880289271; 0, 4, 887997053725554434390896208224098182172046433413077090303223533927585\
        368982278704899744485568148293213510953056994245407964757256507525914496422648040097372491\
        873763947038523416693963470413737067090279630188754195373190813459534748366268185336532592\
        735172032766854975431271478429882221731714931604829250894410570442216705841482944797104098\
        1630050134191116999603, 144299521230402595588520633836415954602957545429625027174273824263\
        232622459620289546208478904824097647195529871761564878794273054182472961105668680306515823\
        029929486641393760055212931235949086044127367018581713238232986043534124358987184487440301\
        520044219672460013222372888834503797332758949403029245625769514545061545744043789084575456\
        92619441315870383064826366; 0, 0, 17218479456385750618067377696052635483579924745448689921\
        733236816400740691241745619397484537236046173286370919031961587788584927290816661024991609\
        882728717344659503471655990880884675228315040132789388737960209873675050796266229296468380\
        078651542064600322344254074734308284513096014127158174803059089817680011026874499049990158\
        039137996170970606384684442822115283, 2798002911662684475435948875608553266081737771135412\
        112281650982665120362326783663152091237300857503159035274342693758015645050684757707416561\
        136605943416568507169314144098518143762869155961064205752042602618307930255506012672631897\
        825040739945741695386029785741837853094263605196404688164353123481397656827216202592071131\
        4230165869124843837753729014283945115624]",
    );
    // - hnf_pernet_stein recurses when the determinant g has more than 62 bits
    test(
        "[820826077893909, 476091713060864, 854490954647371, 671204420793952, 1175637410292177, 14\
        3763178789888; 1795494240972896, 1011149097559921, 1432631338843136, 1367915850605415, 329\
        691910066464, 2034965846371181; 1721987097316615, 1704835264752416, 524032268262157, 73416\
        9430786048, 261412485345347, 2158548773743072; 1152398958166016, 915227742541795, 16487214\
        77977056, 1297021017659625, 853590719927296, 605376570543839; 1488056364923529, 1810541591\
        573504, 1335065615887487, 300955836886688, 667352869528069, 105553820909568]",
        "[1048583, 0, 3145749, 0, 7372547567987405916428916279371373781782991855571, -441105904291\
        7539470232344215857645272394022087296; 0, 1048583, 0, 3145749, 509462528173100350363022263\
        28918697400760533067840, -30481584163438047227105909675205993298161952227757; 0, 0, 419433\
        2, 0, 53431818803717076532645866392760251154215069782648, -3196871981363257486911072314474\
        7358889034974960192; 0, 0, 0, 4194332, 17336320213937453791311572924455761698261802665536\
        , -10372470485324859119800436270721235055409869848968; 0, 0, 0, 0, 54557366903838063413910\
        906088265236876114550471552, -32642144987155418181742406909792335611128493495296]",
    );
    // - add_rows stops reducing a row with a nonzero element before the next pivot, because the
    //   first prime, for both limb widths, loses a pivot
    test(
        "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 0, 77371269172698101563002583, 0, 0; 0, 0, 0, 1, 0]",
        "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 0, 77371269172698101563002583, 0, 0; 0, 0, 0, 1, 0]",
    );
}

#[test]
fn test_extended_gcd_reduced_cofactor() {
    let test = |f, g, out| {
        assert_eq!(
            verify_extended_gcd(
                &Integer::from_str(f).unwrap(),
                &Integer::from_str(g).unwrap()
            ),
            out
        );
    };
    test("0", "0", "(0, 0, 0)");
    test("0", "5", "(5, 0, 1)");
    test("0", "-5", "(5, 0, -1)");
    test("5", "0", "(5, 1, 0)");
    test("-5", "0", "(5, -1, 0)");
    test("3", "3", "(3, 1, 0)");
    test("-3", "3", "(3, -1, 0)");
    test("3", "-3", "(3, 1, 0)");
    test("4", "6", "(2, 5, -3)");
    test("6", "4", "(2, -3, 5)");
    test("-4", "6", "(2, -5, -3)");
    test("4", "-6", "(2, 5, 3)");
    test("2", "4", "(2, 1, 0)");
    test("4", "2", "(2, 0, 1)");
    test("240", "46", "(2, -9, 47)");
    test("46", "240", "(2, 47, -9)");
    test("-46", "-240", "(2, -47, 9)");
    test(
        "1267650600228229401496703205376",
        "1000000000000000000000000000057",
        "(1, -933280119559125392224331127453, 1183073103740199002044159487497)",
    );
}

#[test]
fn hermite_normal_form_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_hnf(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify_hnf(&(a * b));
    });

    integer_matrix_gen_var_3().test_properties(|a| {
        verify_hnf(&a);
    });
}

#[test]
fn extended_gcd_reduced_cofactor_properties() {
    integer_pair_gen().test_properties(|(f, g)| {
        verify_extended_gcd(&f, &g);
    });
}
