// Most optimum on Exercism?
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct HighScores<'a>{
    latest : Option<u32>,
    mpp : BTreeMap<u32,u8>,
    scores : &'a[u32]
}
impl HighScores<'_> {
    pub fn new<'a>(scores: &'a[u32]) -> HighScores<'a> {
        let mut last: Option<u32> = None;
        let mut map = BTreeMap::new();
        for num in scores{
            last = Some(*num);
            map.entry(*num).and_modify(|cnt| *cnt+=1).or_insert(1);
        }
        HighScores{
            latest: last,
            mpp: map,
            scores
        }
    }
    pub fn scores(&self) -> &[u32] {
        self.scores
    }
    pub fn latest(&self) -> Option<u32> {
        self.latest
    }
    pub fn personal_best(&self) -> Option<u32> {
        let m = self.mpp.iter().rev().next();
        if m.is_some(){return Some(*m.unwrap().0);}
        None
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for (key,value) in self.mpp.iter().rev(){
            for _ in 0..*value {
                v.push(*key);
                if v.len() == 3 {break;}
            }
            if v.len() == 3 {break;}
        }
        v
    }
}
