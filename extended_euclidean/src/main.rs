fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);
    let mut k = 2;

    while r1 != 0 {
        let qk = r0 / r1;
        let (r2, s2, t2) = (r0 % r1, s0 - qk * s1, t0 - qk * t1);

        r0 = r1;
        r1 = r2;

        s0 = s1;
        s1 = s2;

        t0 = t1;
        t1 = t2;

        k += 1;
    }

    (r0, s0, t0)
}

fn main() {
    let a = 240;
    let b = 46;

    let (gcd, s, t) = extended_euclid(a, b);

    println!("gcd({}, {}) = {}", a, b, gcd);
    println!("s = {}, t = {}", s, t);
    println!("Verification: {}*{} + {}*{} = {}", s, a, t, b, s*a + t*b);
}
