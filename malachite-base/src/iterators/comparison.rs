//TODO

pub fn is_strictly_ascending<I: Iterator>(xs: I) -> bool
where
    I::Item: Ord,
{
    let mut previous = None;
    for x in xs {
        if let Some(previous) = previous {
            if previous >= x {
                return false;
            }
        }
        previous = Some(x);
    }
    true
}
