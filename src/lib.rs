//! A container that allows item lookup based on tag matching.

#![warn(missing_docs)]

use std::collections::HashMap;
use std::hash::Hash;

/// A container that allows item lookup based on tag matching.
#[derive(Debug)]
pub struct TagMap<T: Eq + Hash, TAG: Eq> {
    /// The inner HashMap used for the implementation.
    pub entries: HashMap<T, Vec<TAG>>,
}

impl<T: Eq + Hash, TAG: Eq> TagMap<T, TAG> {
    /// Creates a new empty TagMap.
    pub fn new() -> Self {
        TagMap { entries: HashMap::new() }
    }
    /// Returns the entries matching the given tags.
    pub fn matching(&self, tags: &[TAG]) -> Vec<&T> {
        let mut vec = Vec::new();
        'entries: for (k, v) in &self.entries {
            for tag in tags.iter() {
                let mut has_tag = false;
                for entry_tag in v.iter() {
                    if tag == entry_tag {
                        has_tag = true;
                    }
                }
                if !has_tag {
                    continue 'entries;
                }
            }
            vec.push(k);
        }
        vec
    }
}
