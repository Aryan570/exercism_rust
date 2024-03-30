fn prime(num : &u64)-> bool{
    if *num <= 1 {return false;}
    let mut i = 2;
    while (i*i) <= *num {
        if *num % i ==0 {return false;}
        i+=1;
    }
    true
}
pub fn factors(n: u64) -> Vec<u64> {
    let mut ans = Vec::new();
    let mut num = n;
    for i in 2..=n{
        let mut flag = false;
        while (num % i) == 0 {
            ans.push(i);
            num = num/i;
            if prime(&num){
                flag = true;
                ans.push(num);
                break;
            }
        }
        if flag == true {break;}
    }
    ans
}
