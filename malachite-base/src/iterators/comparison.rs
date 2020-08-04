use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct DeltaDirections<I: Iterator>
where
    I::Item: Clone + Ord,
{
    previous: Option<I::Item>,
    xs: I,
}

impl<I: Iterator> Iterator for DeltaDirections<I>
where
    I::Item: Clone + Ord,
{
    type Item = Ordering;

    fn next(&mut self) -> Option<Ordering> {
        if self.previous.is_none() {
            if let Some(x) = self.xs.next() {
                self.previous = Some(x);
            } else {
                return None;
            }
        }
        if let Some(x) = self.xs.next() {
            let previous = self.previous.clone().unwrap();
            let result = Some(x.cmp(&previous));
            self.previous = Some(x);
            result
        } else {
            None
        }
    }
}

#[inline]
pub fn delta_directions<I: Iterator>(xs: I) -> DeltaDirections<I>
where
    I::Item: Clone + Ord,
{
    DeltaDirections { previous: None, xs }
}

#[inline]
pub fn is_strictly_ascending<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    delta_directions(xs).all(|x| x == Ordering::Greater)
}

#[inline]
pub fn is_strictly_descending<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    delta_directions(xs).all(|x| x == Ordering::Less)
}

#[inline]
pub fn is_weakly_ascending<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    delta_directions(xs).all(|x| x != Ordering::Less)
}

#[inline]
pub fn is_weakly_descending<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    delta_directions(xs).all(|x| x != Ordering::Greater)
}

pub fn is_strictly_zigzagging<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    let mut previous = None;
    for direction in delta_directions(xs) {
        if direction == Ordering::Equal {
            return false;
        }
        if let Some(previous) = previous {
            if direction == previous {
                return false;
            }
        }
        previous = Some(direction);
    }
    true
}

pub fn is_weakly_zigzagging<I: Iterator>(xs: I) -> bool
where
    I::Item: Clone + Ord,
{
    let mut previous = None;
    for direction in delta_directions(xs).filter(|&d| d != Ordering::Equal) {
        if let Some(previous) = previous {
            if direction == previous {
                return false;
            }
        }
        previous = Some(direction);
    }
    true
}
