pub fn swap_pairs<A: 'static, B: 'static>(
    it: Box<Iterator<Item = (A, B)>>,
) -> Box<Iterator<Item = (B, A)>> {
    Box::new(it.map(|(a, b)| (b, a)))
}

pub fn reshape_2_1_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<Iterator<Item = ((A, B), C)>>,
) -> Box<Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|((a, b), c)| (a, b, c)))
}

pub fn reshape_1_2_to_3<A: 'static, B: 'static, C: 'static>(
    it: Box<Iterator<Item = (A, (B, C))>>,
) -> Box<Iterator<Item = (A, B, C)>> {
    Box::new(it.map(|(a, (b, c))| (a, b, c)))
}
