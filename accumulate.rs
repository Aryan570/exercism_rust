pub fn map<Q,T,F: FnMut(T) -> Q>(input: Vec<T>,mut function: F) -> Vec<Q>{
    let mut v: Vec<Q> = Vec::new();
    for ele in input{
        v.push(function(ele));
    }
    v
}
