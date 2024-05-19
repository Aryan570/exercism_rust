use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut mpp: HashSet<[u32;3]> = HashSet::new();
    for a in 3..=(sum-3)/3{
        for b in (a + 1)..(sum-1-a){
            if a + b >= sum {break;}
            let c = sum - a - b;
            if c < b {break;}
            if c*c == a*a + b*b {
                mpp.insert([a,b,c]);
            }
        }
    }
    mpp
}
