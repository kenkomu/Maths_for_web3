use std::cmp;
use std::collections::HashSet;

// Function to calculate the GCD of two numbers
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to calculate Euler's Totient function
fn euler_totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

// Function to calculate (base^exp) % mod using modular exponentiation
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut base = base % modulus;
    let mut exp = exp;
    let mut result = 1u64;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }

    result
}

// Function to apply Euler's Theorem
fn eulers_theorem(a: u64, n: u64) -> Option<u64> {
    if gcd(a, n) != 1 {
        return None; // a and n are not relatively prime
    }

    let totient = euler_totient(n);
    Some(mod_exp(a, totient, n))
}

fn main() {
    let a = 10;
    let n = 11;

    match eulers_theorem(a, n) {
        Some(result) => println!("{}^φ({}) ≡ {} (mod {})", a, n, result, n),
        None => println!("{} and {} are not relatively prime.", a, n),
    }
}
