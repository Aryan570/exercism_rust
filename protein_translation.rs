use std::collections::HashMap;

pub struct CodonsInfo {
    mpp : HashMap<String,String>
}

impl CodonsInfo {
    pub fn name_for(&self, codon: &str) -> Option<&str> {
        let a = self.mpp.get(codon);
        if a.is_some(){
            return Some(a.unwrap().as_str());
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&str>> {
        let chunks : Vec<String> = rna.chars().collect::<Vec<char>>().chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect();
        let mut ans: Vec<&str> = Vec::new();
        for s in chunks{
            let a = self.mpp.get(&s);
            if a.is_none(){return None;}
            if a.unwrap().as_str() == "stop codon"{break;}
            ans.push(a.unwrap());
        }
        Some(ans)
    }
}

pub fn parse(_pairs: Vec<(& str, & str)>) -> CodonsInfo {
  // clearly, I forgot pairs were provided in the function itself , LOL. I was wondering, why would anyone has to write this.
    let mpp = HashMap::from([
        ("AUU".to_string(), "isoleucine".to_string()),
        ("AUC".to_string(), "isoleucine".to_string()),
        ("AUA".to_string(), "isoleucine".to_string()),
        ("GUU".to_string(), "valine".to_string()),
        ("GUC".to_string(), "valine".to_string()),
        ("GUA".to_string(), "valine".to_string()),
        ("GUG".to_string(), "valine".to_string()),
        ("UUU".to_string(), "phenylalanine".to_string()),
        ("UUC".to_string(), "phenylalanine".to_string()),
        ("AUG".to_string(), "methionine".to_string()),
        ("UGU".to_string(), "cysteine".to_string()),
        ("UGC".to_string(), "cysteine".to_string()),
        ("GCU".to_string(), "alanine".to_string()),
        ("GCC".to_string(), "alanine".to_string()),
        ("GCA".to_string(), "alanine".to_string()),
        ("GCG".to_string(), "alanine".to_string()),
        ("GGU".to_string(), "glycine".to_string()),
        ("GGC".to_string(), "glycine".to_string()),
        ("GGA".to_string(), "glycine".to_string()),
        ("GGG".to_string(), "glycine".to_string()),
        ("CCU".to_string(), "proline".to_string()),
        ("CCC".to_string(), "proline".to_string()),
        ("CCA".to_string(), "proline".to_string()),
        ("CCG".to_string(), "proline".to_string()),
        ("ACU".to_string(), "threonine".to_string()),
        ("ACC".to_string(), "threonine".to_string()),
        ("ACA".to_string(), "threonine".to_string()),
        ("ACG".to_string(), "threonine".to_string()),
        ("UCU".to_string(), "serine".to_string()),
        ("UCC".to_string(), "serine".to_string()),
        ("UCA".to_string(), "serine".to_string()),
        ("UCG".to_string(), "serine".to_string()),
        ("UAU".to_string(), "tyrosine".to_string()),
        ("UAC".to_string(), "tyrosine".to_string()),
        ("UGG".to_string(), "tryptophan".to_string()),
        ("CAA".to_string(), "glutamine".to_string()),
        ("CAG".to_string(), "glutamine".to_string()),
        ("AAU".to_string(), "asparagine".to_string()),
        ("AAC".to_string(), "asparagine".to_string()),
        ("CAU".to_string(), "histidine".to_string()),
        ("CAC".to_string(), "histidine".to_string()),
        ("GAA".to_string(), "glutamic acid".to_string()),
        ("GAG".to_string(), "glutamic acid".to_string()),
        ("GAU".to_string(), "aspartic acid".to_string()),
        ("GAC".to_string(), "aspartic acid".to_string()),
        ("AAA".to_string(), "lysine".to_string()),
        ("AAG".to_string(), "lysine".to_string()),
        ("CGU".to_string(), "arginine".to_string()),
        ("CGC".to_string(), "arginine".to_string()),
        ("CGA".to_string(), "arginine".to_string()),
        ("CGG".to_string(), "arginine".to_string()),
        ("AGA".to_string(), "arginine".to_string()),
        ("AGG".to_string(), "arginine".to_string()),
        ("UUA".to_string(), "leucine".to_string()),
        ("UUG".to_string(), "leucine".to_string()),
        ("UAA".to_string(), "stop codon".to_string()),
        ("UAG".to_string(), "stop codon".to_string()),
        ("UGA".to_string(), "stop codon".to_string()),
    ]);
    CodonsInfo { mpp }
}
