use common::test_properties;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed, pairs_of_integers, triples_of_integers,
};
use malachite_test::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};
use rug;
use std::str::FromStr;

#[test]
fn test_or() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n |= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() | rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "507");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "1000000004095");
    test("12345678987654321", "314159265358979", "12347506587071667");
    test("0", "-123", "-123");
    test("123", "-456", "-389");
    test("1000000000000", "-123", "-123");
    test("123", "-1000000000000", "-999999999877");
    test("1000000000000", "-999999999999", "-4095");
    test("12345678987654321", "-314159265358979", "-1827599417347");
    test("-123", "0", "-123");
    test("-123", "456", "-51");
    test("-1000000000000", "123", "-999999999877");
    test("-123", "1000000000000", "-123");
    test("-1000000000000", "999999999999", "-1");
    test(
        "-12345678987654321",
        "314159265358979",
        "-12033347321712689",
    );
    test("-123", "-456", "-67");
    test("-1000000000000", "-123", "-123");
    test("-123", "-1000000000000", "-123");
    test("-1000000000000", "-999999999999", "-999999999999");
    test("-12345678987654321", "-314159265358979", "-312331665941633");

    test(
        "17561442137713604341197",
        "-533163900219836",
        "-75045493870643",
    );
    test(
        "-18446744013580009457",
        "-18446673704965373937",
        "-18446673644835831793",
    );
    test(
        "-18446673704965373937",
        "-18446744013580009457",
        "-18446673644835831793",
    );
    test(
        "-324518553658389833295008601473024",
        "317057721155483154675232931839",
        "-324201495937234350140333368541185",
    );
    test(
        "317057721155483154675232931839",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324201495937234350140333368541185",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
        "-324201495937234350140333368541185",
    );
    test(
        "576458553284361984",
        "-10889035741470030830237691627457877114880",
        "-10889035741470030830237115168904592752896",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-26298808336",
    );
    test(
        "-4363947867655",
        "-158453907176889445928738488320",
        "-4363947867655",
    );
}

#[test]
fn or_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let result_val_val = x.clone() | y.clone();
        let result_val_ref = x.clone() | y;
        let result_ref_val = x | y.clone();
        let result = x | y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x |= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x |= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x |= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) | integer_to_rug_integer(y))),
            result
        );

        assert_eq!(integer_or_alt_1(&x, y), result);
        assert_eq!(integer_or_alt_2(&x, y), result);

        assert_eq!(y | x, result);
        assert_eq!(&result | x, result);
        assert_eq!(&result | y, result);
        assert_eq!(!(!x & !y), result);
    });

    test_properties(
        pairs_of_integer_and_signed,
        |&(ref x, y): &(Integer, i32)| {
            let result = x | Integer::from(y);
            assert_eq!(x | y, result);
            assert_eq!(y | x, result);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x | Integer::ZERO, *x);
        assert_eq!(Integer::ZERO | x, *x);
        assert_eq!(x | Integer::NEGATIVE_ONE, -1);
        assert_eq!(Integer::NEGATIVE_ONE | x, -1);
        assert_eq!(x | x, *x);
        assert_eq!(x | !x, -1);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x | y) | z, x | (y | z));
        assert_eq!(x & (y | z), (x & y) | (x & z));
        assert_eq!((x & y) | z, (x | z) & (y | z));
        assert_eq!(x | (y & z), (x | y) & (x | z));
        assert_eq!((x | y) & z, (x & z) | (y & z));
    });
}
