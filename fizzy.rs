
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{fmt::Display, ops::{Add, Rem}};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
// pub struct Matcher<T>(std::marker::PhantomData<T>);
pub struct Matcher<T> {
    v: (String, Box<dyn Fn(T) -> bool>),
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: Display>(matcher: F, subs: S) -> Matcher<T> {
        Matcher {
            v: (subs.to_string(), Box::new(matcher)),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
// pub struct Fizzy<T>(std::marker::PhantomData<T>);
pub struct Fizzy<T> {
    check: Vec<Matcher<T>>,
}

impl<T : Clone + Display + 'static> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy { check: Vec::new() }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.check.push(matcher);
        self  
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I : Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |c| {
            let mut ans = String::new();
            for tup in self.check.iter() {
                if tup.v.1(c.clone()) == true{
                    ans.push_str(&tup.v.0)
                }
            }
            if ans.is_empty() {ans.push_str(&c.to_string());}
            return ans
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T : Clone + Display + Add<Output=T> + PartialEq + Rem<Output = T> + From<u8> + 'static>() -> Fizzy<T> {
    Fizzy::new().add_matcher(Matcher::new(|n : T| n % 3.into() == 0.into(), "fizz")).add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
