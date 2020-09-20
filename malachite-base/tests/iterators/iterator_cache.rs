use itertools::__std_iter::empty;

use malachite_base::iterators::iterator_cache::IteratorCache;
use malachite_base::nevers::Never;
use malachite_base::num::exhaustive::exhaustive_signeds;

#[test]
fn test_iterator_cache() {
    let mut xs = IteratorCache::new(empty::<Never>());
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(1), None);
    assert_eq!(xs.known_len(), Some(0));
    assert_eq!(xs.get(0), None);

    let mut xs = IteratorCache::new([1, 2, 3].iter().cloned());
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(1), Some(&2));
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(0), Some(&1));
    assert_eq!(xs.get(3), None);
    assert_eq!(xs.known_len(), Some(3));
    assert_eq!(xs.get(2), Some(&3));

    let mut xs = IteratorCache::new(exhaustive_signeds::<i64>());
    assert_eq!(xs.get(1), Some(&1));
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(0), Some(&0));
    assert_eq!(xs.get(3), Some(&2));
    assert_eq!(xs.get(100), Some(&-50));
    assert_eq!(xs.known_len(), None);
}
