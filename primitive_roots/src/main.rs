use std::collections::HashSet;

// Function to calculate (base^exp) % mod
fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }

    result
}

// Function to check if 'α' is a primitive root of prime 'p'
fn is_primitive_root(alpha: u64, p: u64) -> bool {
    let mut seen = HashSet::new();

    for exp in 1..p {
        let result = mod_exp(alpha, exp, p);
        if seen.contains(&result) {
            return false; // Not a primitive root if duplicate value is found
        }
        seen.insert(result);
    }

    true
}

// Function to find all primitive roots of a prime 'p'
fn find_primitive_roots(p: u64) -> Vec<u64> {
    let mut roots = Vec::new();
    for alpha in 2..p {
        if is_primitive_root(alpha, p) {
            roots.push(alpha);
        }
    }
    roots
}

fn main() {
    let p: u64 = 5; // Example prime number
    let primitive_roots = find_primitive_roots(p);

    println!("Primitive roots of {}: {:?}", p, primitive_roots);
}
