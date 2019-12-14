use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

use common::{test_properties, test_properties_custom_scale};
use malachite_test::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::{
    integers, nonzero_integers, pairs_of_integer_and_nonzero_integer,
    pairs_of_integer_and_nonzero_integer_var_1,
};

#[test]
fn test_div() {
    let test = |u, v, quotient| {
        let mut x = Integer::from_str(u).unwrap();
        x /= Integer::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Integer::from_str(u).unwrap();
        x /= &Integer::from_str(v).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let q = Integer::from_str(u).unwrap() / Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Integer::from_str(u).unwrap() / &Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Integer::from_str(u).unwrap() / Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Integer::from_str(u).unwrap() / &Integer::from_str(v).unwrap();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = BigInt::from_str(u).unwrap() / &BigInt::from_str(v).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = rug::Integer::from_str(u).unwrap() / rug::Integer::from_str(v).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = Integer::from_str(u)
            .unwrap()
            .div_rem(Integer::from_str(v).unwrap())
            .0;
        assert_eq!(q.to_string(), quotient);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "1");
    test("123", "1", "123");
    test("123", "123", "1");
    test("123", "456", "0");
    test("456", "123", "3");
    test("4294967295", "1", "4294967295");
    test("4294967295", "4294967295", "1");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "3", "333333333333");
    test("1000000000000", "123", "8130081300");
    test("1000000000000", "4294967295", "232");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
    );
    test("1000000000000000000000000", "3", "333333333333333333333333");
    test("1000000000000000000000000", "123", "8130081300813008130081");
    test("1000000000000000000000000", "4294967295", "232830643708079");
    test("1000000000000000000000000", "1234567890987", "810000006723");
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290113",
    );
    test("3768477692975601", "11447376614057827956", "0");
    test("3356605361737854", "3081095617839357", "1");
    test("1098730198198174614195", "953382298040157850476", "1");
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
    );
    test("0", "1000000000000000000000000", "0");
    test("123", "1000000000000000000000000", "0");
    test(
        "915607705283450388306561139234228660872677067256472842161753852459689688332903348325308112\
        7923093090598913",
        "11669177832462215441614364516705357863717491965951",
        "784637716923245892498679555408392159158150581185689944063"
    );

    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "-1");
    test("123", "-1", "-123");
    test("123", "-123", "-1");
    test("123", "-456", "0");
    test("456", "-123", "-3");
    test("4294967295", "-1", "-4294967295");
    test("4294967295", "-4294967295", "-1");
    test("1000000000000", "-1", "-1000000000000");
    test("1000000000000", "-3", "-333333333333");
    test("1000000000000", "-123", "-8130081300");
    test("1000000000000", "-4294967295", "-232");
    test(
        "1000000000000000000000000",
        "-1",
        "-1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-3",
        "-333333333333333333333333",
    );
    test(
        "1000000000000000000000000",
        "-123",
        "-8130081300813008130081",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        "-232830643708079",
    );
    test(
        "1000000000000000000000000",
        "-1234567890987",
        "-810000006723",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-1234567890987654321234567890987654321",
        "-810000006723000055638900467181273922269593923137018654",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-316049380092839506236049380092839506176",
        "-3164062526261718967339454949926851258865601262253979",
    );
    test(
        "253640751230376270397812803167",
        "-2669936877441",
        "-94998781946290113",
    );
    test("3768477692975601", "-11447376614057827956", "0");
    test("3356605361737854", "-3081095617839357", "-1");
    test("1098730198198174614195", "-953382298040157850476", "-1");
    test(
        "69738658860594537152875081748",
        "-69738658860594537152875081748",
        "-1",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        "-1",
    );
    test("0", "-1000000000000000000000000", "0");
    test("123", "-1000000000000000000000000", "0");
    test(
        "915607705283450388306561139234228660872677067256472842161753852459689688332903348325308112\
        7923093090598913",
        "-11669177832462215441614364516705357863717491965951",
        "-784637716923245892498679555408392159158150581185689944063"
    );

    test("-1", "1", "-1");
    test("-123", "1", "-123");
    test("-123", "123", "-1");
    test("-123", "456", "0");
    test("-456", "123", "-3");
    test("-4294967295", "1", "-4294967295");
    test("-4294967295", "4294967295", "-1");
    test("-1000000000000", "1", "-1000000000000");
    test("-1000000000000", "3", "-333333333333");
    test("-1000000000000", "123", "-8130081300");
    test("-1000000000000", "4294967295", "-232");
    test(
        "-1000000000000000000000000",
        "1",
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "3",
        "-333333333333333333333333",
    );
    test(
        "-1000000000000000000000000",
        "123",
        "-8130081300813008130081",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "1234567890987",
        "-810000006723",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "1234567890987654321234567890987654321",
        "-810000006723000055638900467181273922269593923137018654",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "316049380092839506236049380092839506176",
        "-3164062526261718967339454949926851258865601262253979",
    );
    test(
        "-253640751230376270397812803167",
        "2669936877441",
        "-94998781946290113",
    );
    test("-3768477692975601", "11447376614057827956", "0");
    test("-3356605361737854", "3081095617839357", "-1");
    test("-1098730198198174614195", "953382298040157850476", "-1");
    test(
        "-69738658860594537152875081748",
        "69738658860594537152875081748",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        "-1",
    );
    test("-123", "1000000000000000000000000", "0");
    test(
        "-91560770528345038830656113923422866087267706725647284216175385245968968833290334832530811\
        27923093090598913",
        "11669177832462215441614364516705357863717491965951",
        "-784637716923245892498679555408392159158150581185689944063"
    );

    test("-1", "-1", "1");
    test("-123", "-1", "123");
    test("-123", "-123", "1");
    test("-123", "-456", "0");
    test("-456", "-123", "3");
    test("-4294967295", "-1", "4294967295");
    test("-4294967295", "-4294967295", "1");
    test("-1000000000000", "-1", "1000000000000");
    test("-1000000000000", "-3", "333333333333");
    test("-1000000000000", "-123", "8130081300");
    test("-1000000000000", "-4294967295", "232");
    test(
        "-1000000000000000000000000",
        "-1",
        "1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-3",
        "333333333333333333333333",
    );
    test(
        "-1000000000000000000000000",
        "-123",
        "8130081300813008130081",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        "232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "-1234567890987",
        "810000006723",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
    );
    test(
        "-253640751230376270397812803167",
        "-2669936877441",
        "94998781946290113",
    );
    test("-3768477692975601", "-11447376614057827956", "0");
    test("-3356605361737854", "-3081095617839357", "1");
    test("-1098730198198174614195", "-953382298040157850476", "1");
    test(
        "-69738658860594537152875081748",
        "-69738658860594537152875081748",
        "1",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        "1",
    );
    test("-123", "-1000000000000000000000000", "0");
    test(
        "-91560770528345038830656113923422866087267706725647284216175385245968968833290334832530811\
        27923093090598913",
        "-11669177832462215441614364516705357863717491965951",
        "784637716923245892498679555408392159158150581185689944063"
    );
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut x = Integer::from(10);
    x /= Integer::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail() {
    let mut x = Integer::from(10);
    x /= &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_fail() {
    Integer::from(10) / Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_val_ref_fail() {
    Integer::from(10) / &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_ref_val_fail() {
    &Integer::from(10) / Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn div_ref_ref_fail() {
    &Integer::from(10) / &Integer::ZERO;
}

fn div_properties_helper(x: &Integer, y: &Integer) {
    let mut mut_x = x.clone();
    mut_x /= y;
    assert!(mut_x.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.clone() / y.clone();
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = x.div_rem(y).0;
    assert_eq!(quotient_alt, quotient);

    let num_quotient = integer_to_bigint(x) / &integer_to_bigint(y);
    assert_eq!(bigint_to_integer(&num_quotient), quotient);

    let rug_quotient = integer_to_rug_integer(x) / integer_to_rug_integer(y);
    assert_eq!(rug_integer_to_integer(&rug_quotient), quotient);

    let remainder = x - &quotient * y;
    assert!(remainder.lt_abs(y));
    assert!(remainder == Integer::ZERO || (remainder > Integer::ZERO) == (*x > Integer::ZERO));
    assert_eq!(&quotient * y + &remainder, *x);
    assert_eq!((-x) / y, -&quotient);
    assert_eq!(x / (-y), -quotient);
}

#[test]
fn div_properties() {
    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties_custom_scale(
        512,
        pairs_of_integer_and_nonzero_integer_var_1,
        |&(ref x, ref y)| {
            div_properties_helper(x, y);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x / Integer::ONE, *x);
        assert_eq!(x / Integer::NEGATIVE_ONE, -x);
    });

    test_properties(nonzero_integers, |x| {
        assert_eq!(Integer::ZERO / x, Integer::ZERO);
        if *x > Integer::ONE {
            assert_eq!(Integer::ONE / x, Integer::ZERO);
        }
        assert_eq!(x / Integer::ONE, *x);
        assert_eq!(x / Integer::NEGATIVE_ONE, -x);
        assert_eq!(x / x, Integer::ONE);
        assert_eq!(x / -x, Integer::NEGATIVE_ONE);
    });
}
