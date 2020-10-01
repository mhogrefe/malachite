use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_bit_distributor() {
    BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
}

#[test]
#[should_panic]
fn bit_distributor_fail_1() {
    BitDistributor::new(&[]);
}

#[test]
#[should_panic]
fn bit_distributor_fail_2() {
    BitDistributor::new(&[BitDistributorOutputType::tiny(); 2]);
}
