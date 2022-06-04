use extra_variadic::Union3;

#[test]
fn test_to_string() {
    let test = |u: Union3<char, u32, bool>, out| {
        assert_eq!(u.to_string(), out);
    };
    test(Union3::A('a'), "A(a)");
    test(Union3::B(5), "B(5)");
    test(Union3::C(false), "C(false)");
}
