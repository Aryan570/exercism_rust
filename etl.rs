use std::collections::BTreeMap;
pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut mpp: BTreeMap<char, i32> = BTreeMap::new();
    for (key,val) in h{
        for ch in val{
            mpp.insert(ch.to_ascii_lowercase(), *key);
        }
    }
    mpp
}
