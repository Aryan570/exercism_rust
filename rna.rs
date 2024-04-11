use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    dna : String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna{
    rna : String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i,ch) in dna.char_indices(){
            match ch {
                'A' | 'C' | 'G' | 'T' => continue,
                _ => return Err(i)
            }
        }
        Ok(Dna{dna : dna.to_string()})
    }

    pub fn into_rna(self) -> Rna {
        let mapping = HashMap::from([('G','C'),('C','G'),('T','A'),('A','U')]);
        let check : String = self.dna.chars().map(|c| {
            if mapping.contains_key(&c){
                return *mapping.get(&c).unwrap();
            }
            c
        }).collect();
        Rna { rna: check }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i,ch) in rna.char_indices(){
            match ch {
                'A' | 'C' | 'G' | 'U' => continue,
                _ => return Err(i)
            }
        }
        Ok(Rna{rna : rna.to_string()})
    }
}
