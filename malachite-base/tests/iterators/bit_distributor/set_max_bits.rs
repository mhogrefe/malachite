use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_set_max_bits() {
    let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(2); 3]);
    assert_eq!(
        bd.bit_map_as_slice(),
        &[
            2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0,
            0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 2, 1, 1,
            0, 0, 2, 2, 1, 1
        ][..]
    );

    bd.set_max_bits(&[0, 2], 5);
    assert_eq!(
        bd.bit_map_as_slice(),
        &[
            2, 2, 1, 1, 0, 0, 2, 2, 1, 1, 0, 0, 2, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1
        ][..]
    );
}

#[test]
#[should_panic]
fn set_max_bits_fail_1() {
    let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(2); 3]);
    bd.set_max_bits(&[0], 0);
}

#[test]
#[should_panic]
fn set_max_bits_fail_2() {
    let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(2); 3]);
    bd.set_max_bits(&[0, 3], 2);
}
