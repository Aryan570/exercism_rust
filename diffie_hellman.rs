use rand::Rng;
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}
fn work(p: u64,g: u64,mut a: u64) -> u64{
    if p == 1 {return 0;}
    let mut result = 1;
    let mut k = (g % p) as u128;
    while a > 0 {
        if a % 2 == 1{
            result = (result as u128 * k) % p as u128;
        }
        a = a >> 1;
        k = (k%p as u128 * k%p as u128) % p as u128;
    }
    result as u64
}
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    work(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    work(p, b_pub, a)
}
// there's a great recursive solution on exercism as well
