//! Direct indexing for monotone instance IDs.
//! Removed slots remain empty until the next DOM clear.
use super::NodeId;

#[derive(Debug, Clone, Default)]
pub(super) struct NodeArena<T> {
    base: u64,
    slots: Vec<Option<T>>,
    live: usize,
}

impl<T> NodeArena<T> {
    pub(super) fn clear(&mut self) {
        self.slots.clear();
        self.live = 0;
    }

    pub(super) fn insert(&mut self, id: NodeId, node: T) {
        if self.slots.is_empty() {
            self.base = id.0;
        }
        assert_eq!(id.0.checked_sub(self.base), Some(self.slots.len() as u64));
        self.slots.push(Some(node));
        self.live += 1;
    }

    pub(super) fn slot(&self, id: &NodeId) -> Option<usize> {
        usize::try_from(id.0.checked_sub(self.base)?).ok()
    }

    pub(super) fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub(super) fn get(&self, id: &NodeId) -> Option<&T> {
        self.slots.get(self.slot(id)?)?.as_ref()
    }

    pub(super) fn get_mut(&mut self, id: &NodeId) -> Option<&mut T> {
        let slot = self.slot(id)?;
        self.slots.get_mut(slot)?.as_mut()
    }

    pub(super) fn contains_key(&self, id: &NodeId) -> bool {
        self.get(id).is_some()
    }

    pub(super) fn remove(&mut self, id: &NodeId) -> Option<T> {
        let slot = self.slot(id)?;
        let node = self.slots.get_mut(slot)?.take()?;
        self.live -= 1;
        Some(node)
    }

    pub(super) fn len(&self) -> usize {
        self.live
    }

    pub(super) fn values(&self) -> impl Iterator<Item = &T> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    pub(super) fn keys(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, node)| node.as_ref().map(|_| NodeId(self.base + i as u64)))
    }
}

impl<T> std::ops::Index<&NodeId> for NodeArena<T> {
    type Output = T;
    fn index(&self, id: &NodeId) -> &T {
        self.get(id).expect("live node ID")
    }
}

#[cfg(test)]
mod arena_tests {
    use super::*;
    use crate::clean::engine::DomNode;
    use std::collections::BTreeMap;

    #[test]
    fn removed_and_cleared_ids_stay_invalid() {
        let mut nodes = NodeArena::default();
        nodes.insert(NodeId(10), DomNode::default());
        nodes.insert(NodeId(11), DomNode::default());
        assert!(nodes.remove(&NodeId(10)).is_some());
        assert!(nodes.remove(&NodeId(10)).is_none());
        nodes.insert(NodeId(12), DomNode::default());
        assert!(nodes.get(&NodeId(10)).is_none());
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes.values().count(), 2);
        assert_eq!(nodes.keys().collect::<Vec<_>>(), [NodeId(11), NodeId(12)]);
        nodes.clear();
        nodes.insert(NodeId(13), DomNode::default());
        for id in 0..13 {
            assert!(nodes.get(&NodeId(id)).is_none());
        }
        assert!(nodes.get(&NodeId(u64::MAX)).is_none());
        assert!(nodes.get(&NodeId(13)).is_some());
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn map_parity_through_insert_remove_mutate_and_clear() {
        for seed in 1..=16u64 {
            let mut random = seed;
            let mut next = 17u64;
            let mut arena = NodeArena::default();
            let mut reference = BTreeMap::new();
            for _ in 0..1000 {
                random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
                let id = NodeId(random % (next + 10));
                match (random >> 32) % 8 {
                    0..=3 => {
                        arena.insert(NodeId(next), random);
                        reference.insert(next, random);
                        next += 1;
                    }
                    4 => assert_eq!(arena.remove(&id), reference.remove(&id.0)),
                    5 => {
                        if let Some(value) = arena.get_mut(&id) {
                            *value = seed;
                        }
                        if let Some(value) = reference.get_mut(&id.0) {
                            *value = seed;
                        }
                    }
                    6 => {
                        arena.clear();
                        reference.clear();
                    }
                    _ => assert_eq!(arena.get(&id), reference.get(&id.0)),
                }
                assert_eq!(arena.len(), reference.len());
                let actual: BTreeMap<_, _> = arena.keys().map(|key| (key.0, arena[&key])).collect();
                assert_eq!(actual, reference);
                assert_eq!(arena.values().count(), reference.len());
            }
        }
    }
}
