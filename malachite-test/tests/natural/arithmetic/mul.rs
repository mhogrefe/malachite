use common::test_properties_custom_scale;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_to_out_basecase, _limbs_mul_to_out_toom_22, _limbs_mul_to_out_toom_22_scratch_size,
    MUL_TOOM22_THRESHOLD,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_unsigned_vec_var_10, triples_of_unsigned_vec_var_11,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_naturals,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_limbs_mul_to_out() {
    let test = |xs, ys, out_before: &[u32], highest_result_limb, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(
            _limbs_mul_to_out_basecase(&mut out, xs, ys),
            highest_result_limb
        );
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10], 0, vec![6, 0, 10]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5, 5, 5, 5, 5],
        0,
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0, 0, 0],
        0,
        vec![6, 19, 32, 21, 0],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10, 10, 10, 10],
        0,
        vec![10_200, 20_402, 30_605, 20_402, 10_200, 0, 10],
    );
    test(
        &[0xffff_ffff],
        &[1],
        &[10, 10, 10],
        0,
        vec![0xffff_ffff, 0, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        0xffff_fffe,
        vec![1, 0xffff_fffe, 10, 10],
    );
    test(
        &[0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        &[0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        &[10, 10, 10, 10, 10, 10],
        0xffff_ffff,
        vec![1, 0, 0, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7, 8], &[1, 2]);
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7], &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left != right)")]
fn limbs_mul_to_out_fail_3() {
    let mut out = vec![10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7], &[]);
}

#[test]
fn test_limbs_mul_to_out_toom_22() {
    let test = |xs: Vec<u32>, ys: Vec<u32>, out_before: Vec<u32>, out_after| {
        let mut scratch = vec![0; _limbs_mul_to_out_toom_22_scratch_size(xs.len())];
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // s != n
    // !(xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less)
    // t != n
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) != Ordering::Less
    // s <= t
    // v_neg_1_neg == 0
    // carry <= 2
    test(
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 31, 20, 0],
    );
    // xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less
    // v_neg_1_neg != 0
    //test(&[2, 0, 4], &[3, 4, 5], &[10, 10, 10, 10, 10, 10], vec![6, 8, 22, 16, 20, 0]);
    test(
        vec![1, 1, 1],
        vec![1, 2, 3],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    // s == n
    // limbs_cmp_same_length(ys0, ys1) != Ordering::Less
    // t == n
    // limbs_cmp_same_length(ys0, ys1) == Ordering::Less
    test(
        vec![1, 1, 1, 1],
        vec![1, 2, 3, 4],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_cmp_same_length(&a0[..n], &a1[..n]) == Ordering::Less
    // limbs_cmp_same_length(&b0[..n], &b1[..n]) != Ordering::Less
    test(
        vec![1, 2, 3, 4],
        vec![1, 1, 1, 1],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
    test(
        vec![1, 2, 3, 4, 5],
        vec![1, 0, 0, 4],
        vec![5, 5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 2, 3, 8, 13, 12, 16, 20, 0],
    );
    // s > t
    // limbs_mul_to_out_basecase in limbs_mul_to_out_toom_22_recursive
    test(
        vec![1, 1, 1, 1],
        vec![1, 2, 3],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 6, 5, 3, 0, 5],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10, 10, 10, 10, 10, 10, 10],
        vec![10_200, 20_402, 30_605, 20_402, 10_200, 0, 10],
    );
    // limbs_mul_to_out_basecase in limbs_mul_same_length_to_out_toom_22_recursive
    test(
        vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        vec![10, 10, 10, 10, 10, 10],
        vec![1, 0, 0, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
    );
    let xs = vec![
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294950911, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 536870911, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let ys = vec![
        4294967295, 4294967295, 4294963199, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        268435455, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];
    let out_len = xs.len() + ys.len();
    // carry > 2
    test(
        xs,
        ys,
        vec![10; out_len],
        vec![
            1, 0, 4096, 0, 0, 0, 0, 0, 16384, 0, 67108864, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4026531840, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 3758095359, 4294967295, 4294967295, 4294966783, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 33554431, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    let limit = 2 * MUL_TOOM22_THRESHOLD;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as u32 + 1);
        long_ys.push((limit - i) as u32);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // limbs_mul_to_out_toom_22 in limbs_mul_same_length_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            60, 179, 356, 590, 880, 1225, 1624, 2076, 2580, 3135, 3740, 4394, 5096, 5845, 6640,
            7480, 8364, 9291, 10260, 11270, 12320, 13409, 14536, 15700, 16900, 18135, 19404, 20706,
            22040, 23405, 24800, 26224, 27676, 29155, 30660, 32190, 33744, 35321, 36920, 38540,
            40180, 41839, 43516, 45210, 46920, 48645, 50384, 52136, 53900, 55675, 57460, 59254,
            61056, 62865, 64680, 66500, 68324, 70151, 71980, 73810, 71980, 70151, 68324, 66500,
            64680, 62865, 61056, 59254, 57460, 55675, 53900, 52136, 50384, 48645, 46920, 45210,
            43516, 41839, 40180, 38540, 36920, 35321, 33744, 32190, 30660, 29155, 27676, 26224,
            24800, 23405, 22040, 20706, 19404, 18135, 16900, 15700, 14536, 13409, 12320, 11270,
            10260, 9291, 8364, 7480, 6640, 5845, 5096, 4394, 3740, 3135, 2580, 2076, 1624, 1225,
            880, 590, 356, 179, 60, 0,
        ],
    );
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit + 2 {
        long_xs.push(i as u32 + 1);
    }
    for i in 0..limit + 1 {
        long_ys.push((limit + 1 - i) as u32);
    }
    let long_out_limbs = vec![10; 2 * limit + 3];
    // limbs_mul_to_out_toom_22 in limbs_mul_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            61, 182, 362, 600, 895, 1246, 1652, 2112, 2625, 3190, 3806, 4472, 5187, 5950, 6760,
            7616, 8517, 9462, 10450, 11480, 12551, 13662, 14812, 16000, 17225, 18486, 19782, 21112,
            22475, 23870, 25296, 26752, 28237, 29750, 31290, 32856, 34447, 36062, 37700, 39360,
            41041, 42742, 44462, 46200, 47955, 49726, 51512, 53312, 55125, 56950, 58786, 60632,
            62487, 64350, 66220, 68096, 69977, 71862, 73750, 75640, 77531, 79422, 77470, 75520,
            73573, 71630, 69692, 67760, 65835, 63918, 62010, 60112, 58225, 56350, 54488, 52640,
            50807, 48990, 47190, 45408, 43645, 41902, 40180, 38480, 36803, 35150, 33522, 31920,
            30345, 28798, 27280, 25792, 24335, 22910, 21518, 20160, 18837, 17550, 16300, 15088,
            13915, 12782, 11690, 10640, 9633, 8670, 7752, 6880, 6055, 5278, 4550, 3872, 3245, 2670,
            2148, 1680, 1267, 910, 610, 368, 185, 62, 0,
        ],
    );
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    assert_eq!(MUL_TOOM22_THRESHOLD, 30);
    // xs_len == 76, ys_len == 68 satisfy s > t && t >= MUL_TOOM22_THRESHOLD && 4 * s >= 5 * t
    for i in 0..76 {
        long_xs.push(i as u32 + 1);
    }
    for i in 0..68 {
        long_ys.push((68 - i) as u32);
    }
    let long_out_limbs = vec![10; 144];
    // limbs_mul_to_out_toom_32 in limbs_mul_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            68, 203, 404, 670, 1000, 1393, 1848, 2364, 2940, 3575, 4268, 5018, 5824, 6685, 7600,
            8568, 9588, 10659, 11780, 12950, 14168, 15433, 16744, 18100, 19500, 20943, 22428,
            23954, 25520, 27125, 28768, 30448, 32164, 33915, 35700, 37518, 39368, 41249, 43160,
            45100, 47068, 49063, 51084, 53130, 55200, 57293, 59408, 61544, 63700, 65875, 68068,
            70278, 72504, 74745, 77000, 79268, 81548, 83839, 86140, 88450, 90768, 93093, 95424,
            97760, 100100, 102443, 104788, 107134, 109480, 111826, 114172, 116518, 118864, 121210,
            123556, 125902, 123012, 120131, 117260, 114400, 111552, 108717, 105896, 103090, 100300,
            97527, 94772, 92036, 89320, 86625, 83952, 81302, 78676, 76075, 73500, 70952, 68432,
            65941, 63480, 61050, 58652, 56287, 53956, 51660, 49400, 47177, 44992, 42846, 40740,
            38675, 36652, 34672, 32736, 30845, 29000, 27202, 25452, 23751, 22100, 20500, 18952,
            17457, 16016, 14630, 13300, 12027, 10812, 9656, 8560, 7525, 6552, 5642, 4796, 4015,
            3300, 2652, 2072, 1561, 1120, 750, 452, 227, 76, 0,
        ],
    );
}

#[test]
#[should_panic(expected = "assertion failed: s > 0 && (s == n || s == n - 1)")]
fn limbs_mul_to_out_toom_22_fail_1() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_mul_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[test]
fn test_mul() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n *= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n *= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n._mul_assign_basecase_mem_opt(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() * BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("1", "123", "123");
    test("123", "1", "123");
    test("123", "456", "56088");
    test("0", "1000000000000", "0");
    test("1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test(
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099789",
        "577397114388109712462006371470162814529304445639807296878809567953200969820156259914159240\
        9106139481288193067515284601342023565222679498917484131095648263181800618990427694244342686\
        4412105186059052689237237088193855584354278755933606296018800151986520872701706693002473648\
        4330061421236425747083307907706860804054565348593527605104495080560663025897787060638154303\
        7631781316565710346299551930891169154491973589315700505458672804104869879731391323700304",
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
    );
}

fn limbs_mul_helper(out_limbs: &Vec<u32>, xs: &Vec<u32>, ys: &Vec<u32>) -> Vec<u32> {
    let mut out_limbs = out_limbs.to_vec();
    let old_out_limbs = out_limbs.clone();
    let highest_result_limb = _limbs_mul_to_out_basecase(&mut out_limbs, xs, ys);
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let len = xs.len() + ys.len();
    let mut limbs = n.into_limbs_asc();
    assert_eq!(highest_result_limb, out_limbs[len - 1]);
    assert_eq!(highest_result_limb == 0, limbs.len() < len);
    limbs.resize(len, 0);
    assert_eq!(limbs, &out_limbs[..len]);
    assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
    out_limbs
}

#[test]
fn limbs_mul_to_out_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_10,
        |&(ref out_limbs, ref xs, ref ys)| {
            limbs_mul_helper(out_limbs, xs, ys);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_22_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_11,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_22_scratch_size(xs.len())];
            _limbs_mul_to_out_toom_22(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn mul_properties() {
    test_properties_custom_scale(2_048, pairs_of_naturals, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = x.clone();
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = x.clone();
        mut_x._mul_assign_basecase_mem_opt(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = natural_to_rug_integer(x);
        mut_x *= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), product);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) * natural_to_biguint(y))),
            product
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) * natural_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        //TODO assert_eq!((product / x).unwrap(), *y);
        //TODO assert_eq!((product / y).unwrap(), *x);

        if *x != 0 && *y != 0 {
            assert!(product >= *x);
            assert!(product >= *y);
        }
    });

    test_properties_custom_scale(
        2_048,
        pairs_of_natural_and_unsigned,
        |&(ref x, y): &(Natural, u32)| {
            let product = x * Natural::from(y);
            assert_eq!(x * y, product);
            assert_eq!(y * x, product);
        },
    );

    test_properties_custom_scale(
        2_048,
        pairs_of_natural_and_unsigned::<u32>,
        |&(ref x, y)| {
            let product = x * Natural::from(y);
            assert_eq!(x * y, product);
            assert_eq!(y * x, product);
        },
    );

    test_properties_custom_scale(2_048, pairs_of_unsigneds::<u32>, |&(x, y)| {
        assert_eq!(
            Natural::from(u64::from(x) * u64::from(y)),
            Natural::from(x) * Natural::from(y)
        );
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties_custom_scale(2_048, naturals, |x| {
        assert_eq!(x * Natural::ZERO, 0);
        assert_eq!(Natural::ZERO * 0, 0);
        assert_eq!(x * Natural::ONE, *x);
        assert_eq!(Natural::ONE * x, *x);
        //TODO assert_eq!(x * x, x.pow(2));
    });

    test_properties_custom_scale(2_048, triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });
}
