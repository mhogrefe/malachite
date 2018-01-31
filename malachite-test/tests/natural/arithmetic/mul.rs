use common::LARGE_LIMIT;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rugint_integer,
                             rugint_integer_to_natural, GenerationMode};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned, pairs_of_naturals,
                                      triples_of_naturals};
use num::BigUint;
use rugint;
use std::str::FromStr;

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

        let n = rugint::Integer::from_str(u).unwrap() * rugint::Integer::from_str(v).unwrap();
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

#[test]
fn mul_properties() {
    // x * y is valid.
    // x * &y is valid.
    // &x * y is valid.
    // &x * &y is valid.
    // x * y is equivalent for malachite, num, and rugint.
    // x *= y, x *= &y, x * y, x * &y, &x * y, and &x * &y give the same result.
    // x * y == y * x
    //TODO x * y / y == x and x * y / x == y
    // if x != 0 and y != 0, x * y >= x and x * y >= y
    let two_naturals = |x: Natural, y: Natural| {
        let num_product = biguint_to_natural(&(natural_to_biguint(&x) * natural_to_biguint(&y)));
        let rugint_product = rugint_integer_to_natural(
            &(natural_to_rugint_integer(&x) * natural_to_rugint_integer(&y)),
        );

        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
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
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x._mul_assign_basecase_mem_opt(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product, "x: {}, y: {}", x, y);

        let mut mut_x = natural_to_rugint_integer(&x);
        mut_x *= natural_to_rugint_integer(&y);
        assert_eq!(rugint_integer_to_natural(&mut_x), product);

        let reverse_product = &y * &x;
        //TODO let inv_1 = (&product / &x).unwrap();
        //TODO let inv_2 = (&product / &y).unwrap();
        assert_eq!(num_product, product);
        assert_eq!(rugint_product, product);
        assert_eq!(reverse_product, product);
        //TODO assert_eq!(inv_1, y);
        //TODO assert_eq!(inv_2, x);

        if x != 0 && y != 0 {
            assert!(product >= x);
            assert!(product >= y);
        }
    };

    // x * (y: u32) == x * from(y)
    // (y: u32) * x == x * from(y)
    let natural_and_u32 = |x: Natural, y: u32| {
        let primitive_product_1 = &x * y;
        let primitive_product_2 = y * &x;
        let product = x * Natural::from(y);
        assert_eq!(primitive_product_1, product);
        assert_eq!(primitive_product_2, product);
    };

    // x * 0 == 0
    // 0 * x == 0
    // x * 1 == x
    // 1 * x == x
    //TODO x * x == x ^ 2
    #[allow(unknown_lints, erasing_op)]
    let one_natural = |x: Natural| {
        let x_old = x.clone();
        assert_eq!(&x * Natural::ZERO, 0);
        assert_eq!(Natural::ZERO * 0, 0);
        let id_1 = &x * Natural::ONE;
        let id_2 = Natural::ONE * &x;
        //TODO let double = &x * &x;
        assert_eq!(id_1, x_old);
        assert_eq!(id_2, x_old);
        //TODO assert_eq!(double, x_old.pow(2));
    };

    // (x * y) * z == x * (y * z)
    // x * (y + z) == x * y + x * z
    // (x + y) * z == x * z + y * z
    let three_naturals = |x: Natural, y: Natural, z: Natural| {
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, x * &z + y * z);
    };

    for (x, y) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_naturals(GenerationMode::Random(2048)).take(LARGE_LIMIT) {
        two_naturals(x, y);
    }

    for (x, y) in pairs_of_natural_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for (x, y) in pairs_of_natural_and_unsigned(GenerationMode::Random(2048)).take(LARGE_LIMIT) {
        natural_and_u32(x, y);
    }

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(2048)).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }

    for (x, y, z) in triples_of_naturals(GenerationMode::Random(2048)).take(LARGE_LIMIT) {
        three_naturals(x, y, z);
    }
}
