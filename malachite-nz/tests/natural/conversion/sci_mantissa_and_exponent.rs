use malachite_base::num::float::nice_float::NiceFloat;
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_sci_mantissa_and_exponent() {
    let test = |n: &str, mantissa: f32, exponent: u64| {
        let (actual_mantissa, actual_exponent) =
            Natural::from_str(n).unwrap().sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
    };
    test("3", 1.5, 1);
    test("123", 1.921875, 6);
    test("1000000000", 1.8626451, 29);
    test("16777216", 1.0, 24);
    test("16777218", 1.0000001, 24);
    test("16777217", 1.0, 24);
    test("33554432", 1.0, 25);
    test("33554436", 1.0000001, 25);
    test("33554433", 1.0, 25);
    test("33554434", 1.0, 25);
    test("33554435", 1.0000001, 25);
    test("340282346638528859811704183484516925439", 1.9999999, 127);
    test("340282346638528859811704183484516925440", 1.9999999, 127);
    test("340282346638528859811704183484516925441", 1.9999999, 127);
    test(
        "10000000000000000000000000000000000000000000000000000",
        1.670478,
        172,
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        1.8920966,
        458,
    );
}
