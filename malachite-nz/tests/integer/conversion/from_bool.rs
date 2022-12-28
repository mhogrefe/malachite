use malachite_nz::integer::Integer;

#[test]
fn test_from_bool() {
    let test = |b, s| {
        let n = Integer::from(b);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), s);
    };
    test(false, "0");
    test(true, "1");
}
