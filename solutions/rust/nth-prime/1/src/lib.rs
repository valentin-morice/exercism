pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn is_prime(x: u32) -> bool {
    (2..=x.isqrt()).all(|i| x % i != 0)
}
