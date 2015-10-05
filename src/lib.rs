//! A container that allows item lookup based on tag matching.

#![warn(missing_docs)]

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::hash::Hash;

/// A container that allows item lookup based on tag matching.
#[derive(Debug)]
pub struct TagMap<T: Eq + Hash, TAG: Eq> {
    /// The inner HashMap used for the implementation.
    pub entries: HashMap<T, Vec<TAG>>,
}

/// Iterator over entries matching requested tags.
pub struct Matching<'hi, 'rt, T: 'static, TAG: 'static> {
    iter: Iter<'hi, T, Vec<TAG>>,
    requested_tags: &'rt [TAG],
}

impl<'a, 'b, T: 'a, TAG: 'a + Eq> Iterator for Matching<'a, 'b, T, TAG> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((v, tags)) => {
                    let mut count = 0;
                    for req in self.requested_tags {
                        for tag in tags {
                            if tag == req {
                                count += 1;
                            }
                        }
                    }
                    if count == self.requested_tags.len() {
                        return Some(v);
                    }
                }
                None => return None,
            }
        }
    }
}

impl<T: Eq + Hash, TAG: Eq> TagMap<T, TAG> {
    /// Creates a new empty TagMap.
    pub fn new() -> Self {
        TagMap { entries: HashMap::new() }
    }
    /// Returns the entries matching the requested tags.
    pub fn matching<'s, 't>(&'s self, tags: &'t [TAG]) -> Matching<'s, 't, T, TAG> {
        Matching {
            iter: self.entries.iter(),
            requested_tags: tags,
        }
    }
}

#[test]
fn test() {
    let mut map = TagMap::new();
    map.entries.insert("elephant",
                       vec!["mammal", "herbivore", "large", "smart"]);
    map.entries.insert("mouse", vec!["mammal", "herbivore", "small"]);
    map.entries.insert("snake", vec!["reptile", "carnivore", "poisonous"]);
    map.entries.insert("shark", vec!["fish", "carnivore", "large"]);
    map.entries.insert("human", vec!["mammal", "omnivore", "smart"]);
    macro_rules! check {
        ($tags:expr, $expected:expr) => {{
            let mut v: Vec<_> = map.matching($tags).collect();
            v.sort();
            let mut expected = $expected;
            expected.sort();
            assert_eq!(&v[..], expected);
        }}
    }
    check!(&["mammal"], [&"human", &"elephant", &"mouse"]);
    check!(&["carnivore"], [&"snake", &"shark"]);
}
