fn euclids_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let a = 900; // Change these values as needed
    let b = 750; // Change these values as needed
    
    if a <= b {
        eprintln!("Error: a should be greater than b.");
        return;
    }
    
    let gcd = euclids_gcd(a, b);
    println!("The GCD of {} and {} is {}", a, b, gcd);
}
