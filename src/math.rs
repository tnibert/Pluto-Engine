use agb::rng;

/// Returns the absolute value of x.
/// Thanks Hacker's Delight!
fn abs(x: i32) -> i32 {
    let y = x >> 31;
    (x ^ y) - y
}

/// Returns a pseudorandom positive number less than max.
pub fn random_constrained_positive(max: i32) -> i32 {
    abs(rng::next_i32() % max)
}
