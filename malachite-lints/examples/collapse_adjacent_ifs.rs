fn early_outs(x: u64, y: u64) -> Option<u64> {
    // Consecutive ifs with identical diverging bodies: flagged.
    if x == 0 {
        return None;
    }
    if y == 0 {
        return None;
    }
    if x == y {
        return None;
    }
    Some(x + y)
}

fn loop_controls(xs: &[u64]) -> u64 {
    let mut sum = 0;
    for &x in xs {
        // Identical `continue` bodies: flagged.
        if x == 3 {
            continue;
        }
        if x == 5 {
            continue;
        }
        sum += x;
    }
    sum
}

fn different_bodies(x: u64) -> Option<u64> {
    // The bodies differ: not flagged.
    if x == 0 {
        return None;
    }
    if x == 1 {
        return Some(0);
    }
    Some(x)
}

fn non_diverging(x: &mut u64) {
    // The shared body does not diverge, so the chain is not equivalent to a single if (both
    // conditions can fire): not flagged.
    if *x > 100 {
        *x /= 2;
    }
    if *x > 10 {
        *x /= 2;
    }
}

fn separated(x: u64, y: u64) -> Option<u64> {
    // A statement between the ifs: not flagged.
    if x == 0 {
        return None;
    }
    let z = x + y;
    if y == 0 {
        return None;
    }
    Some(z)
}

fn with_else(x: u64) -> Option<u64> {
    // The first if has an else: not flagged.
    if x == 0 {
        return None;
    } else if x == 1 {
        return None;
    }
    if x == 2 {
        return None;
    }
    Some(x)
}

fn main() {
    let _ = early_outs(5, 6);
    let _ = loop_controls(&[1, 2, 3]);
    let _ = different_bodies(2);
    let mut x = 200;
    non_diverging(&mut x);
    let _ = separated(1, 2);
    let _ = with_else(3);
}
