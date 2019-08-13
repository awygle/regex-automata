use std::cmp;

use regex_syntax::utf8::Utf8Range;

const ROOT: usize = 0;

#[derive(Clone, Debug)]
pub struct RangeTrie {
    states: Vec<State>,
}

#[derive(Clone, Debug)]
struct State {
    is_final: bool,
    transitions: Vec<Transition>,
}

#[derive(Clone, Debug)]
struct Transition {
    range: Utf8Range,
    next_id: usize,
}

#[derive(Clone, Debug)]
struct Next<'a> {
    ranges: &'a [Utf8Range],
    state_id: usize,
}

impl RangeTrie {
    pub fn new() -> RangeTrie {
        let mut trie = RangeTrie { states: vec![] };
        trie.add_empty();
        trie
    }

    pub fn clear(&mut self) {
        self.states.clear();
        self.add_empty();
    }

    pub fn insert(&mut self, ranges: &[Utf8Range]) {
        if ranges.is_empty() {
            self.states[ROOT].is_final = true;
            return;
        }

        let mut stack: Vec<Next> = vec![Next { ranges, state_id: ROOT }];
        while let Some(Next { ranges, state_id }) = stack.pop() {
            if ranges.is_empty() {
                self.states[state_id].is_final = true;
                continue;
            }
            if self.states[state_id].transitions.is_empty() {
                let next_id = self.add_empty();
                self.states[state_id]
                    .transitions
                    .push(Transition { range: ranges[0], next_id });
                stack.push(Next { ranges: &ranges[1..], state_id: next_id });
                continue;
            }
        }
    }

    pub fn add_empty(&mut self) -> usize {
        let id = self.states.len();
        self.states.push(State { is_final: false, transitions: vec![] });
        id
    }
}

#[derive(Clone, Debug)]
enum Overlap {
    Equal(Utf8Range),
    None(Utf8Range, Utf8Range),
}

fn overlap(old: Utf8Range, new: Utf8Range) -> Overlap {
    if old == new {
        return Overlap::Equal(old);
    }
    let same = match intersect(old, new) {
        None => return Overlap::None(old, new),
        Some(same) => same,
    };
    unimplemented!()
}

fn intersect(r1: Utf8Range, r2: Utf8Range) -> Option<Utf8Range> {
    let start = cmp::max(r1.start, r2.start);
    let end = cmp::min(r1.end, r2.end);
    if start <= end {
        Some(Utf8Range { start, end })
    } else {
        None
    }
}

fn covers(r1: Utf8Range, r2: Utf8Range) -> bool {
    r1.start <= r2.start && r1.end >= r2.end
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use regex_syntax::utf8::Utf8Range;

    use super::*;

    fn r(range: RangeInclusive<u8>) -> Utf8Range {
        Utf8Range { start: *range.start(), end: *range.end() }
    }

    #[test]
    fn intersection() {
        assert_eq!(Some(r(0..=0)), intersect(r(0..=0), r(0..=0)));
        assert_eq!(Some(r(1..=1)), intersect(r(1..=1), r(1..=1)));
        assert_eq!(Some(r(5..=10)), intersect(r(5..=10), r(5..=10)));

        assert_eq!(Some(r(0..=0)), intersect(r(0..=0), r(0..=1)));
        assert_eq!(Some(r(0..=0)), intersect(r(0..=0), r(0..=2)));

        assert_eq!(None, intersect(r(0..=0), r(1..=1)));
        assert_eq!(None, intersect(r(1..=1), r(0..=0)));
    }

    #[test]
    fn covering() {
        assert!(covers(r(0..=0), r(0..=0)));
        assert!(covers(r(0..=3), r(1..=2)));
        assert!(covers(r(1..=3), r(1..=2)));
        assert!(covers(r(0..=2), r(1..=2)));
        assert!(covers(r(5..=10), r(5..=10)));
        assert!(covers(r(5..=10), r(6..=10)));
        assert!(covers(r(5..=10), r(5..=9)));
        assert!(covers(r(5..=10), r(6..=9)));
        assert!(covers(r(5..=10), r(7..=9)));
        assert!(covers(r(5..=10), r(7..=8)));
        assert!(covers(r(5..=10), r(7..=7)));

        assert!(!covers(r(0..=0), r(0..=1)));
        assert!(!covers(r(1..=2), r(0..=3)));
        assert!(!covers(r(1..=2), r(1..=3)));
        assert!(!covers(r(1..=2), r(0..=2)));
        assert!(!covers(r(6..=10), r(5..=10)));
        assert!(!covers(r(5..=9), r(5..=10)));
        assert!(!covers(r(6..=9), r(5..=10)));
        assert!(!covers(r(7..=9), r(5..=10)));
        assert!(!covers(r(7..=8), r(5..=10)));
        assert!(!covers(r(7..=7), r(5..=10)));
    }
}
