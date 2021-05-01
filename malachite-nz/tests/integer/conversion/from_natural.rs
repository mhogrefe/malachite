use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_from_natural() {
    let test = |s| {
        let u = Natural::from_str(s).unwrap();

        let x = Integer::from(u.clone());
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);

        let x = Integer::from(&u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), s);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}
