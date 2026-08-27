// Dead stores are this lint's subject matter: every example row assigns a value that a later
// branch overwrites, so the rustc lint about it stays off.
#![allow(unused_assignments)]

fn main() {
    let c = std::hint::black_box(3u32);
    let mut x = 0;
    let mut y = 0;
    // Every branch assigns to the same target: flagged.
    if c == 0 {
        x = 1;
    } else {
        x = 2;
    }
    // A longer chain, with setup statements before the assignments: flagged.
    if c == 0 {
        x = 3;
    } else if c == 1 {
        let t = c + 1;
        x = t;
    } else {
        x = 4;
    }
    // Different targets: fine.
    if c == 0 {
        x = 5;
    } else {
        y = 6;
    }
    // No final else: fine.
    if c == 0 {
        x = 7;
    }
    // Compound assignment: fine.
    if c == 0 {
        x += 1;
    } else {
        x = 8;
    }
    std::hint::black_box((x, y));
}
