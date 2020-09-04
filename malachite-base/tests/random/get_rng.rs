use rand_chacha::rand_core::RngCore;

use malachite_base::random::EXAMPLE_SEED;

#[test]
fn test_get_rng() {
    let mut bytes = [0; 32];
    EXAMPLE_SEED.get_rng().fill_bytes(&mut bytes);
    assert_eq!(
        bytes,
        [
            113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149,
            115, 189, 149, 217, 201, 117, 146, 31, 72, 151, 169, 174, 33
        ]
    );
}
