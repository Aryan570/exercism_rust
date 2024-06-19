use std::cmp::{min, Ordering};
use std::{cmp::max, iter};
use std::ops::{Add, Mul};
use std::ops::Sub;
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug,Clone)]
pub struct Decimal{
    digits : Vec<u8>,
    pwr_10 : isize,
    is_neg : bool
}
impl Decimal {
    fn new()-> Self{
        Decimal { digits: vec![], pwr_10: 0, is_neg: false }
    }
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut number = Decimal::new();
        let mut start = 0;
        for (i,c) in input.bytes().rev().enumerate(){
            match c {
                b'-' => {number.is_neg = true;start = 1;},
                b'+' => {start = 1; continue},
                _ => {
                    if c == b'.'{
                        number.pwr_10 = -(i as isize - start  as isize);
                        continue;
                    }
                    number.digits.push(c - b'0');
                }
            }
        }
        Some(number.clean())
    }
    fn make_pair(lhs : &Decimal,rhs : &Decimal) -> Vec<(u8,u8)>{
        let mut pairs = Vec::new();
        for i in 0..max(lhs.digits.len(), rhs.digits.len()){
            pairs.push((*lhs.digits.get(i).unwrap_or(&0),*rhs.digits.get(i).unwrap_or(&0)));
        }
        pairs
    }
    fn make_len_equal(lhs : &Decimal,rhs : &Decimal)-> Vec<(u8,u8)>{
        let mut left = Decimal::new();
        if lhs.pwr_10 < rhs.pwr_10 {
            return Decimal::make_len_equal(rhs, lhs).iter().map(|&(a,b)| (b,a)).collect();
        }
        left.digits = iter::repeat(0).take((lhs.pwr_10 - rhs.pwr_10) as usize).chain(lhs.digits.clone()).collect();
        left.pwr_10 = rhs.pwr_10;
        Decimal::make_pair(&left, rhs)
    }
    fn sign_change(&self)-> Self{
        let mut cloned = self.clone();
        cloned.is_neg = !self.is_neg;
        cloned
    }
    fn clean(&self) -> Self{
        let mut result = self.clone();
        let s : String = result.digits.iter().map(|c| *c as char).collect();
        let trimmed_end = s.trim_end_matches('0');
        let trimmed_start = s.trim_start_matches('0');
        if trimmed_start.len() == 0 { 
            result.digits = vec![0];
            result.pwr_10 = 0;
            result.is_neg = false;
        }
        result.pwr_10 += trimmed_end.len() as isize - trimmed_start.len() as isize;
        result.digits = trimmed_start.as_bytes().to_vec();
        result
    }
}
impl PartialEq for Decimal{
    fn eq(&self, other: &Self) -> bool {
        self.clean().partial_cmp(&other.clean()) == Some(Ordering::Equal)
    }
}
impl PartialOrd for Decimal{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_neg && !other.is_neg {return Some(Ordering::Less);}
        else if !self.is_neg && other.is_neg {return Some(Ordering::Greater);}
        for &(i,j) in Decimal::make_len_equal(self, other).iter().rev(){
            if i != j {
                if !self.is_neg {return i.partial_cmp(&j);}
                else {return j.partial_cmp(&i);}
            }
        }
        Some(Ordering::Equal)
    }
}
impl Add for Decimal{
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        let mut ans = Decimal::new();
        let (left , right) = (self.clean(),rhs.clean());
        ans.pwr_10 = min(left.pwr_10, right.pwr_10);
        match (left.is_neg,right.is_neg) {
            (true,true) => ans.is_neg = true,
            (true,false) => return right.sub(left.sign_change()),
            (false,true) => return left.sub(right.sign_change()),
            (_,_) => {}
        }
        let mut carry = 0;
        for (i,j) in Decimal::make_len_equal(&left, &right){
            ans.digits.push((i + j + carry)%10);
            carry = (i + j + carry) / 10;
        }
        if carry != 0 {ans.digits.push(carry);}
        ans.clean()
    }
}
impl Sub for Decimal{
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.is_neg {return self.add(rhs.sign_change());}
        else if self < rhs {return rhs.sub(self).sign_change();}
        let mut ans = Decimal::new();
        let (left,right) = (self.clean(),rhs.clean());
        ans.pwr_10 = min(left.pwr_10, right.pwr_10);
        let mut carry = 0;
        for (i,j) in Decimal::make_len_equal(&left, &right){
            match i >= j + carry {
                true => {
                    ans.digits.push(i - j - carry);
                    carry = 0;
                }
                _ => {
                    ans.digits.push(i + 10 - j - carry);
                    carry = 1;
                }
            }
        }
        ans.clean()
    }
}
impl Mul for Decimal{
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut ans = Decimal::new();
        let (left,right) = (self.clean(),rhs.clean());
        let pwr = left.pwr_10 + right.pwr_10;
        ans.pwr_10 = pwr;
        for (i,a) in left.digits.iter().enumerate(){
            let mut step = Decimal::new();
            step.pwr_10 = i as isize + pwr;
            let mut carry = 0;
            for b in &right.digits{
                step.digits.push((a * b + carry)%10);
                carry = (a * b + carry) / 10;
            }
            if carry > 0 {step.digits.push(carry);}
            ans = step.add(ans);
        }
        ans.is_neg = left.is_neg != right.is_neg;
        ans
    }
}
