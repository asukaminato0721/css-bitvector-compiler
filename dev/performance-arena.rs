// Diagnostic storage, appended to the copied engine by analyze-performance.py.
// IDs are monotone. Removed slots stay empty until the next DOM clear.
#[derive(Debug, Clone, Default)]
struct NodeArena {
    base: u64,
    slots: Vec<Option<DomNode>>,
    live: usize,
}

impl NodeArena {
    fn clear(&mut self) {
        self.slots.clear();
        self.live = 0;
    }

    fn insert(&mut self, id: NodeId, node: DomNode) {
        if self.slots.is_empty() {
            self.base = id.0;
        }
        assert_eq!(id.0.checked_sub(self.base), Some(self.slots.len() as u64));
        self.slots.push(Some(node));
        self.live += 1;
    }

    fn slot(&self, id: &NodeId) -> Option<usize> {
        usize::try_from(id.0.checked_sub(self.base)?).ok()
    }

    fn get(&self, id: &NodeId) -> Option<&DomNode> {
        self.slots.get(self.slot(id)?)?.as_ref()
    }

    fn get_mut(&mut self, id: &NodeId) -> Option<&mut DomNode> {
        let slot = self.slot(id)?;
        self.slots.get_mut(slot)?.as_mut()
    }

    fn contains_key(&self, id: &NodeId) -> bool {
        self.get(id).is_some()
    }

    fn remove(&mut self, id: &NodeId) -> Option<DomNode> {
        let slot = self.slot(id)?;
        let node = self.slots.get_mut(slot)?.take()?;
        self.live -= 1;
        Some(node)
    }

    fn len(&self) -> usize {
        self.live
    }

    fn values(&self) -> impl Iterator<Item = &DomNode> {
        self.slots.iter().filter_map(Option::as_ref)
    }

    fn keys(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.slots
            .iter()
            .enumerate()
            .filter_map(|(i, node)| node.as_ref().map(|_| NodeId(self.base + i as u64)))
    }
}

impl std::ops::Index<&NodeId> for NodeArena {
    type Output = DomNode;
    fn index(&self, id: &NodeId) -> &DomNode {
        self.get(id).expect("live node ID")
    }
}

#[cfg(test)]
mod arena_tests {
    use super::*;

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
}
