use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() != second_list.len(){return false;}
    for i in 0..first_list.len(){
        if first_list[i] != second_list[i] {return false;}
    }
    true
}
fn is_sublist<T:PartialEq + std::cmp::Eq + std::hash::Hash>(first_list: &[T], second_list: &[T]) -> bool {
    // early return first_lest should be shorter than second
    if first_list.len() == 0 {return true};
    if first_list.len() >= second_list.len(){ return false;}
    // sliding window -- window size is first_list.len() , use the hashmap
    let mut first_map = HashMap::new();
    let mut second_map = HashMap::new();
    for num in first_list{
        first_map.entry(num).and_modify(|cnt| *cnt+=1).or_insert(1);
    }
    for i in 0..first_list.len(){
        second_map.entry(second_list.get(i).unwrap()).and_modify(|cnt| *cnt+=1).or_insert(1);
    }
    let mut last = first_list.len() - 1;
    let mut first = 0;
    while last < second_list.len()   {
        if first_map == second_map {
            return true;
        }
        last += 1;
        if last < second_list.len() {
            second_map.entry(second_list.get(last).unwrap()).and_modify(|cnt| *cnt+=1).or_insert(1);
        }
        second_map.entry(second_list.get(first).unwrap()).and_modify(|cnt| *cnt -= 1);
        if *second_map.get(&second_list[first]).unwrap() <= 0{
            second_map.remove(&second_list[first]);
        }
        first+=1;
    }
    false
}
pub fn sublist<T: PartialEq + std::cmp::Eq + std::hash::Hash>(first_list: &[T], second_list: &[T]) -> Comparison {
    // todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if is_equal(&first_list, &second_list) {
        return Comparison::Equal;
    }
    //early return as both are not equal but have same size
    if first_list.len() == second_list.len() {return Comparison::Unequal}
    if is_sublist(&first_list, &second_list) {
        return Comparison::Sublist;
    }
    if is_sublist(&second_list, &first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

// hey, so you are telling me there is in-built sliding window in RUST?? OMG
// community solutions are brilliant btw, check them out
