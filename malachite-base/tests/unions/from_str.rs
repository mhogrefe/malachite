use malachite_base::unions::{Union3, UnionFromStrError};
use std::result::Result;
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(Union3::from_str(s), out);
    };
    test("A(a)", Ok(Union3::A('a')));
    test("B(5)", Ok(Union3::B(5)));
    test("C(false)", Ok(Union3::C(false)));

    test("", Err(UnionFromStrError::Generic(String::new())));
    test("xyz", Err(UnionFromStrError::Generic("xyz".to_string())));
    test("D(a)", Err(UnionFromStrError::Generic("D(a)".to_string())));
    test("A(a", Err(UnionFromStrError::Generic("A(a".to_string())));

    let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("A(ab)");
    if let Err(UnionFromStrError::Specific(Union3::A(_e))) = result {
    } else {
        panic!("wrong error variant")
    }

    let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("B(-1)");
    if let Err(UnionFromStrError::Specific(Union3::B(_e))) = result {
    } else {
        panic!("wrong error variant")
    }

    let result: Result<Union3<char, u32, bool>, _> = Union3::from_str("C(tralse)");
    if let Err(UnionFromStrError::Specific(Union3::C(_e))) = result {
    } else {
        panic!("wrong error variant")
    }
}
