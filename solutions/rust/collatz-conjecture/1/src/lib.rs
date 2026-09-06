pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut steps = 0;

    if n == 0 {
        return None;
    }

    while n != 1 {
        steps += 1;
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    Some(steps)
}
