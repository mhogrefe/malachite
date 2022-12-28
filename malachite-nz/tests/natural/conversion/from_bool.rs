use malachite_nz::natural::Natural;

#[test]
fn test_from_bool() {
    let test = |b, s| {
        let n = Natural::from(b);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), s);
    };
    test(false, "0");
    test(true, "1");
}
