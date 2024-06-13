use std::collections::HashSet;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.len() == 0 {return Some(Vec::new());}
    let mut rever = Vec::new();
    for (start, end) in input {
        rever.push((*end, *start));
    }
    for i in 0..input.len() {
        let mut vis: HashSet<usize> = HashSet::new();
        vis.insert(i);
        let mut vec = Vec::new();
        vec.push((input[i].0,input[i].1));
        let chain = f(input[i].0, input[i].1, input,&rever, &mut vis,&mut vec);
        if chain.is_some(){return chain;}
    }
    None
}
fn get_candidate(candidate_idx : &mut Vec<usize>,input: &[(u8, u8)],end: u8,vis: &HashSet<usize>){
    let mut idx = 0;
    for (start,_) in input{
        if *start == end && !vis.contains(&idx){candidate_idx.push(idx);}
        idx += 1;
    }
}
fn f(first: u8, end: u8, input: &[(u8, u8)],rever : &Vec<(u8,u8)>, vis: &mut HashSet<usize>,vec: &mut Vec<(u8,u8)>) -> Option<Vec<(u8, u8)>> {
    if vis.len() == input.len(){
        if first == end && is_right(vec){return Some(vec.to_vec());}
        return None;
    }
    let mut candidate_idx_input = Vec::new();
    let mut candidate_idx_rever = Vec::new();
    get_candidate(&mut candidate_idx_input,input,end,vis);
    get_candidate(&mut candidate_idx_rever,rever,end,vis);
    let mut first_ans ;
    for idx in candidate_idx_input{
        vec.push(input[idx]);
        vis.insert(idx);
        first_ans = f(first, input[idx].1, input, rever, vis, vec);
        if first_ans.is_some(){return first_ans;}
        vis.remove(&idx);
        vec.pop();
    }
    first_ans = None;
    for idx in candidate_idx_rever{
        vec.push(rever[idx]);
        vis.insert(idx);
        first_ans = f(first, rever[idx].1, input, rever, vis, vec);
        if first_ans.is_some(){return first_ans;}
        vis.remove(&idx);
        vec.pop();
    }
    first_ans
}
fn is_right(input: &Vec<(u8, u8)>) -> bool {
    let (first, mut equal) = input[0];
    for i in 1..input.len() {
        let (f, e) = input[i];
        if equal != f {return false;}
        equal = e;
    }
    if equal != first {return false;}
    true
}
// PaulDance's solution is so much better.. check that out.
// concept - Eulerian cycle in graph
