// 1st try was like this
use std::collections::HashMap;

pub struct Allergies{
    int_to_enum : HashMap<u32,Allergen>,
    score : u32
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mpp: HashMap<u32, Allergen> = HashMap::from([
        (1,Allergen::Eggs),
        (2,Allergen::Peanuts),
        (4,Allergen::Shellfish),
        (8,Allergen::Strawberries),
        (16,Allergen::Tomatoes),
        (32,Allergen::Chocolate),
        (64,Allergen::Pollen),
        (128,Allergen::Cats)]);
        Allergies{
            int_to_enum : mpp,
            score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let mut score = (self.score)%256;
        let mut i = 128;
        while i > 0{
            let m = score % i;
            if m == score {i = i/2; continue;}
            if self.int_to_enum.get(&i).unwrap() == allergen {return true;}
            score = m;
            i = i/2;
        }
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = (self.score)%256;
        let mut i = 128;
        let mut v: Vec<Allergen> = Vec::new();
        while i > 0{
            let m = score % i;
            if m == score {i = i/2; continue;}
            v.push(self.int_to_enum.get(&i).unwrap().clone());
            score = m;
            i = i/2;
        }
        v
    }
}
