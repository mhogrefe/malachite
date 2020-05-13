#[test]
pub fn test_min() {
    assert_eq!(min!(4), 4);
    assert_eq!(min!(4, 5, 6), 4);
}

#[test]
pub fn test_max() {
    assert_eq!(max!(4), 4);
    assert_eq!(max!(4, 5, 6), 6);
}
