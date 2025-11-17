use std::collections::{HashMap, HashSet};

use crate::{LayoutFrame, NFA, Selector, SelectorId, SelectorManager, json_value_to_attr_string};

/// Access to selector manager from a DOM implementation.
pub trait HasSelectorManager {
    fn selector_manager(&mut self) -> &mut SelectorManager;
}

/// Access to node arena.
pub trait HasNodes<N> {
    fn nodes_mut(&mut self) -> &mut HashMap<u64, N>;
}

/// Minimal attribute-bearing node surface used by shared helpers.
pub trait NodeAttributes {
    fn attributes(&mut self) -> &mut HashMap<String, String>;
    fn class_ids(&mut self) -> &mut HashSet<SelectorId>;
    fn id_selector_id(&mut self) -> &mut Option<SelectorId>;
}

/// Common update_attribute used by bit/tri/quad DOMs.
pub fn update_attribute_common<D, N>(
    dom: &mut D,
    node_idx: u64,
    key: &str,
    new_value: Option<String>,
) where
    D: HasSelectorManager + HasNodes<N>,
    N: NodeAttributes,
{
    let key_lower = key.to_lowercase();
    match key_lower.as_str() {
        "class" => {
            let mut new_class_ids = HashSet::new();
            if let Some(ref class_value) = new_value {
                for class_name in class_value
                    .split_whitespace()
                    .filter(|name| !name.is_empty())
                {
                    let class_id = dom
                        .selector_manager()
                        .get_or_create_id(Selector::Class(class_name.to_string()));
                    new_class_ids.insert(class_id);
                }
            }

            if let Some(node) = dom.nodes_mut().get_mut(&node_idx) {
                if let Some(ref val) = new_value {
                    node.attributes().insert(key_lower.clone(), val.clone());
                } else {
                    node.attributes().remove(key_lower.as_str());
                }
                *node.class_ids() = new_class_ids;
            }
        }
        "id" => {
            let new_selector_id = new_value.as_ref().map(|value| {
                dom.selector_manager()
                    .get_or_create_id(Selector::Id(value.to_string()))
            });

            if let Some(node) = dom.nodes_mut().get_mut(&node_idx) {
                if let Some(ref val) = new_value {
                    node.attributes().insert(key_lower.clone(), val.clone());
                } else {
                    node.attributes().remove(key_lower.as_str());
                }
                *node.id_selector_id() = new_selector_id;
            }
        }
        _ => {
            if let Some(node) = dom.nodes_mut().get_mut(&node_idx) {
                if let Some(ref val) = new_value {
                    node.attributes().insert(key_lower.clone(), val.clone());
                } else {
                    node.attributes().remove(key_lower.as_str());
                }
            }
        }
    }
}

/// Minimal DOM surface necessary for a shared apply_frame implementation.
pub trait FrameDom<N>: HasSelectorManager + HasNodes<N>
where
    N: NodeAttributes,
{
    fn reset_dom(&mut self);
    fn json_to_html_node(&mut self, node: &serde_json::Value, parent: Option<u64>, nfa: &NFA);
    fn add_node_by_path(&mut self, path: &[usize], node: &serde_json::Value, nfa: &NFA);
    fn remove_node_by_path(&mut self, path: &[usize]);
    fn node_id_by_path(&mut self, path: &[usize]) -> Option<u64>;
    fn set_node_dirty(&mut self, node_idx: u64);
    fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]);
}

/// Shared apply_frame logic for bit/tri/quad style DOMs.
pub fn apply_frame_common<D, N, FInput, FRecalcInput>(
    dom: &mut D,
    frame: &LayoutFrame,
    nfa: &NFA,
    make_input: FInput,
    make_recalc_input: FRecalcInput,
) where
    D: FrameDom<N>,
    N: NodeAttributes,
    FInput: Fn() -> Vec<bool>,
    FRecalcInput: Fn(&NFA) -> Vec<bool>,
{
    match frame.as_command() {
        crate::Command::Init { node } => {
            dom.reset_dom();
            dom.json_to_html_node(node, None, nfa);
            dom.recompute_styles(nfa, &make_input());
        }
        crate::Command::Add { path, node } => {
            dom.add_node_by_path(&path, node, nfa);
            dom.recompute_styles(nfa, &make_input());
        }
        crate::Command::ReplaceValue {
            path,
            key,
            value,
            old_value,
        } => {
            let node_idx = dom
                .node_id_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path for ReplaceValue {:?}", path));
            if let Some(old_value) = old_value {
                let expected = json_value_to_attr_string(old_value);
                let actual = dom
                    .nodes_mut()
                    .get_mut(&node_idx)
                    .and_then(|node| node.attributes().get(&key.to_lowercase()).cloned())
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            let new_value = value.map(json_value_to_attr_string);
            update_attribute_common(dom, node_idx, key, new_value);
            dom.set_node_dirty(node_idx);
            dom.recompute_styles(nfa, &make_input());
        }
        crate::Command::InsertValue { path, key, value } => {
            let node_idx = dom
                .node_id_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path for InsertValue {:?}", path));
            let new_value = value.map(json_value_to_attr_string);
            update_attribute_common(dom, node_idx, key, new_value);
            dom.set_node_dirty(node_idx);
            dom.recompute_styles(nfa, &make_input());
        }
        crate::Command::DeleteValue {
            path,
            key,
            old_value,
        } => {
            let node_idx = dom
                .node_id_by_path(&path)
                .unwrap_or_else(|| panic!("invalid path for DeleteValue {:?}", path));
            if let Some(old_value) = old_value {
                let expected = json_value_to_attr_string(old_value);
                let actual = dom
                    .nodes_mut()
                    .get_mut(&node_idx)
                    .and_then(|node| node.attributes().get(&key.to_lowercase()).cloned())
                    .unwrap_or_default();
                assert_eq!(
                    actual, expected,
                    "existing attribute value mismatch for key {} at path {:?}",
                    key, path
                );
            }
            update_attribute_common(dom, node_idx, key, None);
            dom.set_node_dirty(node_idx);
            dom.recompute_styles(nfa, &make_input());
        }
        crate::Command::Recalculate => {
            dom.recompute_styles(nfa, &make_recalc_input(nfa));
        }
        crate::Command::Remove { path } => {
            dom.remove_node_by_path(&path);
            dom.recompute_styles(nfa, &make_input());
        }
    }
}
