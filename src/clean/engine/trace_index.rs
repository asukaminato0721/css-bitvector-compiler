//! Sparse external IDs use sorted storage, independent of the largest ID.
use super::NodeId;

#[derive(Debug, Clone, Default)]
pub(super) struct TraceIndex {
    entries: Vec<(u64, Option<NodeId>)>,
}

impl TraceIndex {
    pub(super) fn get(&self, key: &u64) -> Option<&NodeId> {
        self.entries
            .binary_search_by_key(key, |entry| entry.0)
            .ok()
            .and_then(|index| self.entries[index].1.as_ref())
    }

    pub(super) fn insert(&mut self, key: u64, value: NodeId) -> Option<NodeId> {
        match self.entries.binary_search_by_key(&key, |entry| entry.0) {
            Ok(index) => self.entries[index].1.replace(value),
            Err(index) => {
                self.entries.insert(index, (key, Some(value)));
                None
            }
        }
    }

    pub(super) fn remove(&mut self, key: &u64) -> Option<NodeId> {
        self.entries
            .binary_search_by_key(key, |entry| entry.0)
            .ok()
            .and_then(|index| self.entries[index].1.take())
    }

    pub(super) fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn sparse_ids_and_replacements_match_a_tree_map() {
        let mut index = TraceIndex::default();
        let mut reference = BTreeMap::new();
        let mut random = 7u64;
        for i in 0..2000 {
            random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
            let key = match i % 5 {
                0 => u64::MAX,
                1 => 0,
                _ => random % 100,
            };
            match random >> 61 {
                0 => {
                    index.clear();
                    reference.clear();
                }
                1..=3 => assert_eq!(index.remove(&key), reference.remove(&key)),
                _ => assert_eq!(
                    index.insert(key, NodeId(i)),
                    reference.insert(key, NodeId(i))
                ),
            }
            assert_eq!(
                index
                    .entries
                    .iter()
                    .filter_map(|&(key, value)| value.map(|value| (key, value)))
                    .collect::<Vec<_>>(),
                reference.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>()
            );
            assert_eq!(index.get(&key), reference.get(&key));
        }
    }
}
