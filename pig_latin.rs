use std::collections::HashSet;
fn make_pig_latin(input: &str, vowels: &mut HashSet<char>) -> String {
    let v: Vec<char> = input.chars().collect();
    match (vowels.contains(&v[0]) && v[0] != 'y')
        || (v[0] == 'x' && v[1] == 'r')
        || (v[0] == 'y' && v[1] == 't'){
        // case 1
        true => {
            let mut s = input.to_string();
            s.push_str("ay");
            return s;
        }
        _ => {
            // case 3
            if (v.len() >= 3 && v[1] == 'q' && v[2] == 'u')
                || (v.len() >= 2 && v[0] == 'q' && v[1] == 'u'){
                vowels.remove(&'u');
            }
            // case 2 && case 4
            if v[0] == 'y' {vowels.remove(&'y');}
            let mut tmp = String::new();
            let mut i = 0;
            println!("{}",input);
            while !vowels.contains(&v[i]){
                tmp.push(v[i]);
                i += 1;
            }
            let s = &mut input[i..].to_string();
            s.push_str(&tmp);
            s.push_str("ay");
            vowels.insert('u'); vowels.insert('y');
            return s.to_string();
        }
    }
}
pub fn translate(input: &str) -> String {
    // let's collect the strings
    let v: Vec<_> = input.split_whitespace().collect();
    let mut vowels = HashSet::from(['a', 'e', 'i', 'o', 'u', 'y']);
    let mut ans = String::new();
    for s in v {
        // let's create a funcion that gives us the tranformed string
        ans.push_str(&make_pig_latin(s, &mut vowels));
        ans.push(' ');
    }
    ans.trim().to_string()
}
