use std::collections::{BTreeSet, HashMap};
use std::cmp::Ordering;
#[derive(PartialEq,Eq)]
struct U8Str<'a>{
    key: u8,
    value: &'a str,
}
impl Ord for U8Str<'_>  {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.key.cmp(&other.key) {
            Ordering::Equal => other.value.cmp(self.value),
            ordering => ordering,
        }
    }
}
impl PartialOrd for U8Str<'_>  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn tally(match_results: &str) -> String {
    // Team --->  MP | W | D | L | P
    // need to have BtreeSet of (pts,player)
    let mut mpp: HashMap<String, Vec<u8>> = HashMap::new();
    for line in match_results.lines(){
        let v : Vec<&str> = line.split(';').collect();
        match v[2] {
            "win" => {
                // player who won
                mpp.entry((&v[0]).to_string()).and_modify(|arr| {
                    arr[1] += 1; arr[0] += 1; arr[4] += 3;
                }).or_insert(vec![1,1,0,0,3]);
                // player who lost
                mpp.entry((&v[1]).to_string()).and_modify(|arr| {
                    arr[3] += 1; arr[0] += 1;
                }).or_insert(vec![1,0,0,1,0]);
            }
            "draw" => {
                mpp.entry((&v[0]).to_string()).and_modify(|arr| {
                    arr[2] += 1; arr[0] += 1; arr[4] += 1;
                }).or_insert(vec![1,0,1,0,1]);
                mpp.entry((&v[1]).to_string()).and_modify(|arr| {
                    arr[2] += 1; arr[0] += 1; arr[4] += 1;
                }).or_insert(vec![1,0,1,0,1]);
            }
            "loss" => {
                // player who loss
                mpp.entry((&v[0]).to_string()).and_modify(|arr| {
                    arr[3] += 1; arr[0] += 1;
                }).or_insert(vec![1,0,0,1,0]);
                // player who won
                mpp.entry((&v[1]).to_string()).and_modify(|arr| {
                    arr[1] += 1; arr[0] += 1; arr[4] += 3;
                }).or_insert(vec![1,1,0,0,3]);
            }
            _ => println!("Why are we here?")
        }
    }
    let mut ans = Vec::new();
    ans.push(format!("Team                           | MP |  W |  D |  L |  P"));
    let mut btree: BTreeSet<U8Str> = BTreeSet::new();
    for (key,val) in mpp.iter(){btree.insert(U8Str {key : val[4],value : key});}
    for tup in btree.into_iter().rev(){
        let val = mpp.get(tup.value).unwrap();
        ans.push(format!("{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",tup.value,val[0],val[1],val[2],val[3],val[4]));
    }
    ans.join("\n")
}
