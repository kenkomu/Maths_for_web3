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

fn fermat_little_theorem(a: u64, p: u64) -> bool {
    if p <= 1 || a % p == 0 {
        return false;
    }

    // Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p)
    let result = mod_exp(a, p - 1, p);
    result == 1
}

fn main() {
    let a = 2;
    let p = 6;

    if fermat_little_theorem(a, p) {
        println!("Fermat's Little Theorem holds: {}^(p-1) ≡ 1 (mod {})", a, p);
    } else {
        println!("Fermat's Little Theorem does not hold for a = {} and p = {}", a, p);
    }
}
