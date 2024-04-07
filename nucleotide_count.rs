use std::collections::HashMap;
// solution without hashmap here is also possible. But that way I would have to iterate through string twice.
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"AGTC".contains(nucleotide){return Err(nucleotide);}
    let mut mpp = HashMap::new();
    for ch in dna.chars(){
        match ch {
            _nuc if nucleotide == ch =>{
                mpp.entry(ch).and_modify(|cnt| *cnt+=1).or_insert(1);
            }
            'A' | 'G' | 'T' | 'C' => continue,
            _ => return Err(ch)
        }
    }
    let ans = mpp.get(&nucleotide);
    if ans.is_none() {return Ok(0);}
    Ok(*ans.unwrap())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut mpp = HashMap::from([('A',0),('G',0),('T',0),('C',0)]);
    for ch in dna.chars(){
        match ch {
            'A' | 'G' | 'T' | 'C' => {
                mpp.entry(ch).and_modify(|cnt| *cnt+=1);
            }
            _ => return Err(ch)
        }
    }
    Ok(mpp)
}
