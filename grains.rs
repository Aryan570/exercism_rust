use core::panic;
pub fn square(s: u32) -> u64 {
    if s > 64 || s <=0 {panic!("Square must be between 1 and 64");}
    2_u64.pow(s - 1)
}
pub fn total() -> u64 {
    ((2_u128.pow(64)) - 1).try_into().unwrap()
}
