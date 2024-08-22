fn main() {
    test_prime();
    test_non_prime();

    for n in 0..=100 {
        if is_prime(n) {
            println!("{} is a prime number.", n);
        } else {
            println!("{} is not a prime number.", n);
        }
    }
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn test_prime() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    for &num in primes.iter() {
        assert!(is_prime(num), "Test failed: {} is a prime number.", num);
    }
}

fn test_non_prime() {
    let non_primes = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30, 32, 33, 34, 35];
    for &num in non_primes.iter() {
        assert!(!is_prime(num), "Test failed: {} is not a prime number.", num);
    }
}
