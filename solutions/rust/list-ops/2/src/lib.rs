use std::iter;
/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    iter::from_fn(move || {
        if let Some(x) = a.next() {
            return Some(x);
        }
        if let Some(x) = b.next() {
            return Some(x);
        }
        None
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut curr = nested_iter.next();
    std::iter::from_fn(move || loop {
        let ele = curr.as_mut().and_then(|e| e.next());
        match ele {
            Some(x) => return Some(x),
            _ => {
                curr = nested_iter.next();
                curr.as_ref()?
            }
        };
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    std::iter::from_fn(move || {
        for x in iter.by_ref() {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    })
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut cnt = 0;
    for _ in iter {
        cnt += 1;
    }
    cnt
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || {
        if let Some(x) = iter.next() {
            return Some(function(x));
        }
        None
    })
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut cnt = initial;
    for e in iter {
        cnt = function(cnt, e);
    }
    cnt
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut cnt = initial;
    while let Some(x) = iter.next_back() {
        cnt = function(cnt, x);
    }
    cnt
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}