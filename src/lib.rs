pub mod matrix;
pub mod polynomials;
pub mod rational;

pub fn gcd(mut n: u32, mut m: u32) -> u32 {
    assert!(n != 0 && m != 0);
    if n == 0 {
        std::mem::swap(&mut n, &mut m);
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
