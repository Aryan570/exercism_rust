use std::cmp;
// dynamic programming
#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}
fn f(idx : usize,max_weight: u32, items: &[Item],dp : &mut Vec<Vec<i32>>) -> u32 {
    if idx >= items.len() || max_weight <=0 {return 0;}
    if dp[idx][max_weight as usize] != -1 {return dp[idx][max_weight as usize] as u32;}
    let mut take = 0;
    if max_weight >= items[idx].weight {
        take = items[idx].value + f(idx + 1,max_weight - items[idx].weight,items,dp);
    }
    let not_take = 0 + f(idx + 1,max_weight,items,dp);
    dp[idx][max_weight as usize] = cmp::max(cmp::max(take, not_take) as i32,dp[idx][max_weight as usize]);
    dp[idx][max_weight as usize] as u32
}
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![vec![-1;1000];items.len()];
    f(0,max_weight,items,&mut dp)
}
