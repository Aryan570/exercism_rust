use std::collections::{BTreeMap, BTreeSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    mpp : BTreeMap<u32,BTreeSet<String>>
}

impl School {
    pub fn new() -> School {
        School { mpp: BTreeMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let mut b = BTreeSet::new();
        b.insert(student.to_string());
        self.mpp.entry(grade).and_modify(|st| {st.insert(student.to_string());}).or_insert(b);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for (key,_) in self.mpp.iter(){v.push(*key);}
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v = Vec::new();
        let st = self.mpp.get(&grade);
        if st.is_some(){
            for i in st.unwrap().iter(){v.push(i.to_string());}
        }
        v
    }
}





[package]
edition = "2021"
name = "grade-school"
version = "0.0.0"

[lints.clippy]
new_without_default = "allow"
