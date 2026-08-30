pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut x = n;

    if n < 2 {
        return primes;
    }

    let mut i = 2;
    while i * i <= x {
        while x.is_multiple_of(i) {
            primes.push(i);
            x /= i;
        }
        i += 1;
    }

    if x > 1 {
        primes.push(x);
    }

    primes
}
