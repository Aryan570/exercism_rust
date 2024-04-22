use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T : Eq + Hash> {
    mpp : HashSet<T>
}

impl<T : Eq + Hash + Copy> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut mpp = HashSet::new();
        for ent in input{
            mpp.insert(*ent);
        }
        CustomSet { mpp }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.mpp.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.mpp.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for ele in self.mpp.iter(){
            if !other.contains(ele){
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.mpp.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for ele in self.mpp.iter(){
            if other.contains(ele){
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut mpp = HashSet::new();
        for ele in self.mpp.iter(){
            if other.contains(ele){
                mpp.insert(*ele);
            }
        }
        CustomSet { mpp }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut mpp = HashSet::new();
        for ele in self.mpp.iter(){
            if !other.contains(ele){
                mpp.insert(*ele);
            }
        }
        CustomSet { mpp }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut mpp = HashSet::new();
        for ele in self.mpp.iter(){
            mpp.insert(*ele);
        }
        for ele in other.mpp.iter(){
            mpp.insert(*ele);
        }
        CustomSet { mpp }
    }
}
