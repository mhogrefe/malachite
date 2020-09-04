use malachite_base::random::{Seed, EXAMPLE_SEED};

#[test]
fn test_next() {
    assert_eq!(
        EXAMPLE_SEED.next(),
        Seed::from_bytes([
            113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149,
            115, 189, 149, 217, 201, 117, 146, 31, 72, 151, 169, 174, 33
        ])
    );
}
