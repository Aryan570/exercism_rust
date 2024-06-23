use std::collections::{BinaryHeap, HashMap};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: BinaryHeap<_> = hands.iter().map(|&s| (Hands::parse(s), s)).collect();
    let (winning, s) = hands.pop().unwrap();
    let mut ans = vec![s];
    while let Some((val, s)) = hands.pop() {
        if val < winning {
            break;
        }
        ans.push(s);
    }
    ans
}
fn parse_card(s: &str) -> (u8, u8) {
    // 4D -> 4 D
    // J = 11 K = 12 Q = 13 A = 14
    let (value, suit) = s.split_at(s.len() - 1);
    (
        match value.parse::<u8>() {
            Ok(i) => i,
            Err(_) => "JQKA".find(value).unwrap() as u8 + 11,
        },
        suit.as_bytes()[0],
    )
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hands {
    // no. cards with the same rank
    counts: Vec<usize>,
    //providing the ranks to the cards
    values: Vec<u8>,
}
impl Hands {
    fn parse(s: &str) -> Self {
        let (values, suits): (Vec<u8>, Vec<u8>) =
            s.split_ascii_whitespace().map(parse_card).unzip();
        let mut groups: HashMap<u8, usize> = HashMap::new();
        for &val in values.iter() {
            *groups.entry(val).or_default() += 1;
        }
        let mut groups: Vec<(usize, u8)> = groups.into_iter().map(|(v, c)| (c, v)).collect();
        groups.sort_unstable_by(|a, b| b.partial_cmp(&a).unwrap());
        let (mut counts, mut values): (Vec<_>, Vec<_>) = groups.iter().copied().unzip();
        if counts.len() == 5 {
            if values == [14, 5, 4, 3, 2] {
                values = vec![5, 4, 3, 2, 1];
            }
            let is_straight = values[0] - values[4] == 4;
            let is_flush = suits[1..].iter().all(|&c| c == suits[0]);
            match (is_straight, is_flush) {
                (true, true) => counts = vec![4, 2],
                (true, false) => counts = vec![3, 1, 2],
                (false, true) => counts = vec![3, 1, 3],
                _ => {}
            }
        }
        Hands { counts, values }
    }
}
