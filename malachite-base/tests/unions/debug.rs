use malachite_base::strings::ToDebugString;
use malachite_base::unions::Union2;

#[test]
fn test_to_debug() {
    let test = |u: Union2<Vec<char>, u32>, out| {
        assert_eq!(u.to_debug_string(), out);
    };
    test(Union2::A(vec![]), "A([])");
    test(Union2::A(vec!['a', 'b', 'c']), "A(['a', 'b', 'c'])");
    test(Union2::B(5), "B(5)");
}
