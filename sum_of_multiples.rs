// Most Sane Solution
// Rust community seems to love iterators
// filters, maps etc etc. I'm not too fond of it.
use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut st: HashSet<u32> = HashSet::new();
    for factor in factors {
        if *factor == 0 {continue;}
        let mut i = 1;
        while factor * i < limit {
            st.insert(factor*i);
            i += 1;
        }
    }
    st.iter().sum()
}
