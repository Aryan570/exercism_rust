use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mpp = HashMap::from([('R',"radishes"),('C',"clover"),('G',"grass"),('V',"violets")]);
    let mut ans = Vec::new();
    let v : Vec<&str> = diagram.split('\n').collect();
    let first_row : Vec<char> = v[0].chars().collect();
    let second_row : Vec<char> = v[1].chars().collect();
    let idx = (student.as_bytes()[0] - b'A') * 2;
    match first_row[idx as usize] {
        'R' => ans.push(*mpp.get(&'R').unwrap()),
        'C' => ans.push(*mpp.get(&'C').unwrap()),
        'G' => ans.push(*mpp.get(&'G').unwrap()),
        'V' => ans.push(*mpp.get(&'V').unwrap()),
        _ => {}
    }
    match first_row[(idx + 1) as usize] {
        'R' => ans.push(*mpp.get(&'R').unwrap()),
        'C' => ans.push(*mpp.get(&'C').unwrap()),
        'G' => ans.push(*mpp.get(&'G').unwrap()),
        'V' => ans.push(*mpp.get(&'V').unwrap()),
        _ => {}
    }
    match second_row[idx as usize] {
        'R' => ans.push(*mpp.get(&'R').unwrap()),
        'C' => ans.push(*mpp.get(&'C').unwrap()),
        'G' => ans.push(*mpp.get(&'G').unwrap()),
        'V' => ans.push(*mpp.get(&'V').unwrap()),
        _ => {}
    }
    match second_row[(idx + 1) as usize] {
        'R' => ans.push(*mpp.get(&'R').unwrap()),
        'C' => ans.push(*mpp.get(&'C').unwrap()),
        'G' => ans.push(*mpp.get(&'G').unwrap()),
        'V' => ans.push(*mpp.get(&'V').unwrap()),
        _ => {}
    }
    ans
}
