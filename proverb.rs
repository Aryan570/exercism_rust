pub fn build_proverb(list: &[&str]) -> String {
    let mut v = Vec::new();
    for i in 0..list.len(){
        match (list.get(i),list.get(i+1)){
            (Some(&a),Some(&b)) => {
                v.push(format!("For want of a {a} the {b} was lost."));
            },
            (Some(&_a),None) => {
                v.push(format!("And all for the want of a {0}.",list.get(0).unwrap()));
            }
            _ => println!("30/03/24")
        }
    }
    v.join("\n")
}
