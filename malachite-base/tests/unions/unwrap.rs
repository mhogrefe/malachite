use extra_variadic::Union3;

#[test]
fn test_unwrap() {
    let test = |u: Union3<char, char, char>, out| {
        assert_eq!(u.unwrap(), out);
    };
    test(Union3::A('a'), 'a');
    test(Union3::B('b'), 'b');
    test(Union3::C('c'), 'c');
}
