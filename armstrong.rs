pub fn is_armstrong_number(num: u32) -> bool {
    let mut to_count_digits = num.clone();
    let mut soup = num.clone();
    let mut sum:u64 = 0;
    let mut digits = 0;
    while to_count_digits != 0 {
        to_count_digits = to_count_digits/10;
        digits+=1;
    }
    while soup > 0 {
        sum += (soup%10).pow(digits) as u64;
        soup = soup/10;
    }
    sum == num as u64
}
