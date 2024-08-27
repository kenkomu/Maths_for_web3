fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        factors.push(n);
    }
    factors.sort();
    factors.dedup();
    factors
}

fn euler_totient(n: u64) -> u64 {
    // Case 1: If n is a prime number
    if is_prime(n) {
        return n - 1;
    }

    // Check if n is a product of two distinct prime numbers
    let factors = prime_factors(n);
    if factors.len() == 2 && n == factors[0] * factors[1] {
        return (factors[0] - 1) * (factors[1] - 1);
    }

    // Case 3: If n is a product of composite numbers or more than two prime factors
    let mut result = n;
    for &p in &factors {
        result = result * (p - 1) / p;
    }
    result
}

fn main() {
    let n1 = 5;  // Case 1: prime number
    let n2 = 35;  // Case 2: product of two distinct primes (3 and 5)
    let n3 = 7000;  // Case 3: product of composite numbers (2^2 * 3)
    
    println!("Euler's Totient of {} is {}", n1, euler_totient(n1));
    println!("Euler's Totient of {} is {}", n2, euler_totient(n2));
    println!("Euler's Totient of {} is {}", n3, euler_totient(n3));
}
