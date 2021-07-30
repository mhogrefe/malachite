use malachite_base::num::conversion::traits::IsInteger;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_is_integer() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().is_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
    test("-1", true);
    test("-100", true);
}

#[test]
fn is_integer_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 256);
    config.insert("mean_stripe_n", 128);
    integer_gen().test_properties_with_config(&config, |n| {
        assert!(n.is_integer());
    });
}
