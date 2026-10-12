//! Stable name IDs and sparse node properties.
#[derive(Debug, Clone, Default)]
pub(super) struct Names {
    values: Vec<String>,
    sorted: Vec<usize>,
}

impl Names {
    pub(super) fn find(&self, name: &str) -> Option<usize> {
        self.sorted
            .binary_search_by(|&id| self.values[id].as_str().cmp(name))
            .ok()
            .map(|index| self.sorted[index])
    }

    pub(super) fn intern(&mut self, name: &str) -> usize {
        match self
            .sorted
            .binary_search_by(|&id| self.values[id].as_str().cmp(name))
        {
            Ok(index) => self.sorted[index],
            Err(index) => {
                let id = self.values.len();
                self.values.push(name.to_owned());
                self.sorted.insert(index, id);
                id
            }
        }
    }

    pub(super) fn name(&self, id: usize) -> &str {
        &self.values[id]
    }

    pub(super) fn classes(&mut self, value: &str) -> Vec<usize> {
        let mut ids: Vec<_> = value.split_whitespace().map(|s| self.intern(s)).collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }
}

/// Sparse node properties. Keys are interned IDs, values preserve the input text.
#[derive(Debug, Clone, Default)]
pub(super) struct Attributes {
    entries: Vec<(usize, String)>,
}

impl Attributes {
    /// The external matcher supplies strings instead of compiled name IDs.
    pub(super) fn get_named(&self, names: &Names, name: &str) -> Option<&String> {
        self.entries
            .iter()
            .find(|(id, _)| names.name(*id) == name)
            .map(|(_, value)| value)
    }

    pub(super) fn get(&self, id: usize) -> Option<&String> {
        self.entries
            .binary_search_by_key(&id, |entry| entry.0)
            .ok()
            .map(|index| &self.entries[index].1)
    }

    pub(super) fn insert(&mut self, id: usize, value: String) {
        match self.entries.binary_search_by_key(&id, |entry| entry.0) {
            Ok(index) => self.entries[index].1 = value,
            Err(index) => self.entries.insert(index, (id, value)),
        }
    }

    pub(super) fn remove(&mut self, id: usize) {
        if let Ok(index) = self.entries.binary_search_by_key(&id, |entry| entry.0) {
            self.entries.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_names_preserve_existing_ids_and_attribute_values() {
        let mut names = Names::default();
        let z = names.intern("z");
        let a = names.intern("a");
        assert_eq!(names.intern("z"), z);
        assert_eq!(names.find("a"), Some(a));
        assert_eq!(names.find("missing"), None);
        assert_eq!(names.name(z), "z");
        assert_eq!(names.classes("z a z"), vec![z, a]);
        let mut attributes = Attributes::default();
        attributes.insert(a, "z a z".into());
        attributes.insert(z, "old".into());
        attributes.insert(z, "new".into());
        assert_eq!(attributes.get(a).map(String::as_str), Some("z a z"));
        assert_eq!(attributes.get(z).map(String::as_str), Some("new"));
        attributes.remove(a);
        assert_eq!(attributes.get(a), None);
        assert_eq!(attributes.get(z).map(String::as_str), Some("new"));
    }
}
