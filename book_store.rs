// i hate this one
use std::cmp::min;
const PRICES : [u32;6] = [0,800,1520, 2160, 2560, 3000];
pub fn lowest_price(books: &[u32]) -> u32 {
    let mut v = vec![(0,1),(0,2),(0,3),(0,4),(0,5)];
    for b in books{
        v[(b - 1) as usize].0 += 1;
    }
    custom_sort(&mut v);
    f(v)
}
fn f(v : Vec<(u32,u8)>) -> u32{
    let mut cost = v.iter().map(|(c,_)| c).sum::<u32>() * 800;
    for i in 2..=v.iter().filter(|&&(c,_)| c > 0).count(){
        let mut v_c = v.clone();
        for j in 0..i{
            v_c[j].0 -= 1;
        }
        custom_sort(&mut v_c);
        cost = min(cost , PRICES[i] + f(v_c));
    }
    cost
}
fn custom_sort(arr : &mut Vec<(u32,u8)>){
    // reverse sorting by occurances
    arr.sort_unstable_by(|(a,_),(b,_)| b.cmp(a));
}
