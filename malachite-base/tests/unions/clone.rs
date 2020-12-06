use malachite_base::unions::Union2;

#[test]
fn test_clone() {
    let test = |u: Union2<Vec<char>, u32>| {
        let cloned = u.clone();
        assert_eq!(cloned, u);
    };
    test(Union2::A(vec![]));
    test(Union2::A(vec!['a', 'b', 'c']));
    test(Union2::B(5));
}

#[test]
fn test_clone_from() {
    let test = |mut u: Union2<Vec<char>, u32>, v: Union2<Vec<char>, u32>| {
        u.clone_from(&v);
        assert_eq!(u, v);
    };
    test(Union2::A(vec!['a', 'b', 'c']), Union2::A(vec![]));
    test(Union2::A(vec![]), Union2::A(vec!['a', 'b', 'c']));
    test(Union2::B(5), Union2::B(6));
    test(Union2::A(vec!['a', 'b', 'c']), Union2::B(6));
    test(Union2::B(6), Union2::A(vec!['a', 'b', 'c']));
}
