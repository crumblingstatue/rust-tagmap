//! A container that allows item lookup based on tag matching.

#![warn(missing_docs)]

use std::collections::BTreeMap;
use std::collections::btree_map::Iter;

/// A container that allows item lookup based on tag matching.
#[derive(Debug)]
pub struct TagMap<T: Ord, TAG: Eq> {
    /// The inner BTreeMap used for the implementation.
    pub entries: BTreeMap<T, Vec<TAG>>,
}

/// Iterator over entries matching a rule.
#[derive(Clone)]
pub struct Matching<'hi, 'r, T: 'static, TAG: 'static> {
    iter: Iter<'hi, T, Vec<TAG>>,
    rule: &'r MatchRule<TAG>,
}

/// Iterator over entries matching a rule. Yields both T and its tags.
#[derive(Clone)]
pub struct MatchingEntries<'hi, 'r, T: 'static, TAG: 'static> {
    iter: Iter<'hi, T, Vec<TAG>>,
    rule: &'r MatchRule<TAG>,
}

fn tags_match_rule<TAG: Eq>(tags: &[TAG], rule: &MatchRule<TAG>) -> bool {
    use MatchRule::*;
    match *rule {
        Tags(ref m_tags) => {
            let mut count = 0;
            for m_tag in m_tags {
                for tag in tags {
                    if *tag == *m_tag {
                        count += 1;
                    }
                }
            }
            count == m_tags.len()
        }
        NotTags(ref m_tags) => {
            for m_tag in m_tags {
                for tag in tags {
                    if *tag == *m_tag {
                        return false;
                    }
                }
            }
            true
        }
        AnyTag(ref m_tags) => {
            for m_tag in m_tags {
                for tag in tags {
                    if *tag == *m_tag {
                        return true;
                    }
                }
            }
            false
        }
        Rules(ref rules) => {
            let mut count = 0;
            for rule in rules {
                if tags_match_rule(tags, rule) {
                    count += 1;
                }
            }
            count == rules.len()
        }
        NotRules(ref rules) => {
            for rule in rules {
                if tags_match_rule(tags, rule) {
                    return false;
                }
            }
            true
        }
        AnyRule(ref rules) => {
            for rule in rules {
                if tags_match_rule(tags, rule) {
                    return true;
                }
            }
            false
        }
    }
}

impl<'a, 'b, T: 'a, TAG: 'a + Eq> Iterator for Matching<'a, 'b, T, TAG> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((v, tags)) => {
                    if tags_match_rule(tags, self.rule) {
                        return Some(v);
                    } else {
                        continue;
                    }
                }
                None => return None,
            }
        }
    }
}

impl<'a, 'b, T: 'a, TAG: 'a + Eq> Iterator for MatchingEntries<'a, 'b, T, TAG> {
    type Item = (&'a T, &'a [TAG]);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((v, tags)) => {
                    if tags_match_rule(tags, self.rule) {
                        return Some((v, tags));
                    } else {
                        continue;
                    }
                }
                None => return None,
            }
        }
    }
}

impl<T: Ord, TAG: Eq> TagMap<T, TAG> {
    /// Creates a new empty TagMap.
    pub fn new() -> Self {
        TagMap { entries: BTreeMap::new() }
    }
    /// Returns the entries matching the given rule.
    pub fn matching<'s, 'r>(&'s self, rule: &'r MatchRule<TAG>) -> Matching<'s, 'r, T, TAG> {
        Matching {
            iter: self.entries.iter(),
            rule: rule,
        }
    }
    /// Returns the entries matching the given rule. Yields both T and its tags.
    pub fn matching_entries<'s, 'r>(&'s self,
                                    rule: &'r MatchRule<TAG>)
                                    -> MatchingEntries<'s, 'r, T, TAG> {
        MatchingEntries {
            iter: self.entries.iter(),
            rule: rule,
        }
    }
}

/// A rule of how to match against tags.
#[derive(Debug, PartialEq)]
pub enum MatchRule<TAG> {
    /// Match all given tags.
    Tags(Vec<TAG>),
    /// Don't match any given tag.
    NotTags(Vec<TAG>),
    /// Match any given tag.
    AnyTag(Vec<TAG>),
    /// Match all given rules.
    Rules(Vec<MatchRule<TAG>>),
    /// Don't match any given rule.
    NotRules(Vec<MatchRule<TAG>>),
    /// Match any given rule.
    AnyRule(Vec<MatchRule<TAG>>),
}

#[test]
fn test() {
    use MatchRule::*;
    let mut map = TagMap::new();
    map.entries.insert("elephant",
                       vec!["mammal", "herbivore", "large", "intelligent", "friendly"]);
    map.entries.insert("mouse",
                       vec!["mammal", "herbivore", "small", "furry", "neutral"]);
    map.entries.insert("snake",
                       vec!["reptile", "carnivore", "poisonous", "hostile"]);
    map.entries.insert("shark", vec!["fish", "carnivore", "large", "hostile"]);
    map.entries.insert("human",
                       vec!["mammal", "omnivore", "intelligent", "friendly", "primate"]);
    map.entries.insert("lion",
                       vec!["feline", "mammal", "carnivore", "hostile", "furry"]);
    map.entries.insert("dog",
                       vec!["canine", "mammal", "carnivore", "friendly", "furry"]);
    map.entries.insert("chimpanzee",
                       vec!["mammal", "primate", "neutral", "omnivore", "furry"]);
    map.entries.insert("goldfish", vec!["fish", "friendly"]);
    map.entries.insert("carp", vec!["fish", "neutral"]);
    map.entries.insert("blowfish", vec!["fish", "poisonous"]);
    macro_rules! check {
        ($tags:expr, $expected:expr) => {{
            let mut v: Vec<_> = map.matching($tags).collect();
            v.sort();
            let mut expected = $expected;
            expected.sort();
            assert_eq!(&v[..], expected);
        }}
    }
    check!(&Tags(vec!["mammal"]),
           [&"human", &"elephant", &"mouse", &"dog", &"lion", &"chimpanzee"]);
    check!(&Tags(vec!["carnivore", "mammal", "friendly"]), [&"dog"]);
    check!(&NotTags(vec!["mammal"]),
           [&"snake", &"shark", &"goldfish", &"carp", &"blowfish"]);
    check!(&Rules(vec![Tags(vec!["fish"]), NotTags(vec!["poisonous"])]),
           [&"goldfish", &"carp", &"shark"]);
    check!(&AnyTag(vec!["canine", "reptile"]), [&"dog", &"snake"]);
    check!(&AnyRule(vec![
            Rules(vec![
                Tags(vec!["carnivore"]), NotTags(vec!["friendly"]),
            ]),
            Rules(vec![
                Tags(vec!["fish"]), AnyTag(vec!["friendly", "neutral", "poisonous"])
            ]),
        ]),
           [&"shark", &"lion", &"goldfish", &"carp", &"blowfish", &"snake"])
}
