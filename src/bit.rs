use css_bitvector_compiler::rdtsc;
use cssparser::{Parser, ParserInput, Token};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fmt::Display,
};

static mut MISS_CNT: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Complex { parts: Vec<SelectorPart> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SelectorPart {
    selector: Selector,
    combinator: Combinator,
}
impl Display for SelectorPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.selector, self.combinator)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Combinator {
    Descendant, // 空格
    Child,      // >
    None,       // 最后一个选择器没有组合器
}
impl Display for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, ">"),
            Combinator::None => write!(f, ""),
        }
    }
}
#[derive(Debug, Clone)]
struct NfaStateInfo {
    selector: Selector,
    parent_state: Option<usize>, // None if it's the first selector in a chain
    combinator: Combinator,      // Combinator leading to this state
    is_final: bool,              // Corresponds to CssMatch::Done
}

#[derive(Debug, Default, Clone)]
struct NFA {
    states: Vec<Option<NfaStateInfo>>, // Indexed by state usize
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssMatch {
    Done { parts: Vec<SelectorPart> },
    Doing { parts: Vec<SelectorPart> },
}

impl Display for CssMatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = match self {
            CssMatch::Done { parts } | CssMatch::Doing { parts } => parts,
        };

        for part in parts {
            write!(f, "{}", part)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
}
impl Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Type(tag) => write!(f, "{}", tag),
            Selector::Class(class) => write!(f, ".{}", class),
            Selector::Id(id) => write!(f, "#{}", id),
        }
    }
}
#[derive(Default)]
struct BitVectorHtmlNode {
    tag_name: String,
    id: u64,
    html_id: Option<String>,
    class: HashSet<String>,
    children: Vec<BitVectorHtmlNode>,
    output_state: Vec<bool>,
    parent: Option<*mut BitVectorHtmlNode>, // TODO: use u64 in future
    dirty: bool,
    recursive_dirty: bool,
}

impl Debug for BitVectorHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitVectorHtmlNode")
            .field("tag_name", &self.tag_name)
            .field("id", &self.id)
            .field("html_id", &self.html_id)
            .field("class", &self.class)
            .field("parent", &self.parent.map_or(0, |x| unsafe { { &*x }.id }))
            .field("children", &self.children)
            //  .field("dirty", &self.dirty)
            //  .field("recursive_dirty", &self.recursive_dirty)
            .finish()
    }
}

impl BitVectorHtmlNode {
    fn set_dirty(&mut self) {
        self.dirty = true;
        self.recursive_dirty = true;
        unsafe {
            let mut cur: *mut BitVectorHtmlNode = self;
            while let Some(parent_ptr) = (*cur).parent {
                if (*parent_ptr).recursive_dirty {
                    break;
                } else {
                    (*parent_ptr).recursive_dirty = true;
                    cur = parent_ptr;
                }
            }
        }
    }
    fn json_to_html_node(&mut self, json_node: &serde_json::Value, num_states: usize) -> Self {
        let mut node = Self::default();
        node.tag_name = json_node["name"].as_str().unwrap().into();
        node.id = json_node["id"].as_u64().unwrap();
        node.html_id = {
            let attributes = json_node["attributes"].as_object().unwrap();
            attributes
                .get("id")
                .and_then(|x| x.as_str())
                .map(String::from)
        };
        node.class = json_node["attributes"]
            .as_object()
            .unwrap()
            .get("class")
            .map(|x| x.as_str().unwrap())
            .unwrap_or_default()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        // Add children recursively
        node.children = {
            let children = json_node["children"].as_array().unwrap();
            children
                .into_iter()
                .map(|x| self.json_to_html_node(x, num_states))
                .collect()
        };
        node.output_state = vec![false; num_states];
        node.set_dirty();
        node.fix_parent_pointers();
        node
    }
    fn fix_parent_pointers(&mut self) {
        let self_ptr = self as *mut Self;
        for child in self.children.iter_mut() {
            child.parent = Some(self_ptr);
            child.fix_parent_pointers();
        }
    }
    fn matches_simple_selector(&self, selector: &Selector) -> bool {
        match selector {
            Selector::Type(tag) => self.tag_name.to_lowercase() == tag.to_lowercase(),
            Selector::Class(class) => self.class.contains(class),
            Selector::Id(id) => {
                if let Some(ref html_id) = self.html_id {
                    html_id == id
                } else {
                    false
                }
            }
        }
    }
    fn add_node_by_path(
        &mut self,
        path: &[usize],
        json_node: &serde_json::Value,
        num_states: usize,
    ) {
        assert!(!path.is_empty());
        if path.len() > 1 {
            self.children[path[0]].add_node_by_path(&path[1..], json_node, num_states);
            return;
        }
        let new_n = self.json_to_html_node(json_node, num_states);
        self.children.insert(path[0], new_n);
        self.set_dirty();
        self.fix_parent_pointers();
    }
    fn remove_node_by_path(&mut self, path: &[usize]) {
        assert!(!path.is_empty());
        if path.len() > 1 {
            self.children[path[0]].remove_node_by_path(&path[1..]);
            return;
        }
        self.children.remove(path[0]);
        self.set_dirty();
    }
    fn recompute_styles(&mut self, nfa: &NFA, input: &[bool]) {
        if !self.recursive_dirty {
            return;
        }
        if self.dirty {
            unsafe {
                MISS_CNT += 1;
            }
            let new_output_state = self.new_output_state(input, nfa);
            if self.output_state != new_output_state {
                self.output_state = new_output_state;
                for c in self.children.iter_mut() {
                    c.set_dirty();
                }
            }
        } else {
            // Check: if not dirty, recomputing should not change output
            let original_output_state = self.output_state.clone();
            let new_output_state = self.new_output_state(input, nfa);
            assert_eq!(
                original_output_state, new_output_state,
                "Node ID {}: Output state changed when node was not dirty!",
                self.id
            );
        }
        for child in self.children.iter_mut() {
            child.recompute_styles(nfa, &self.output_state);
        }
        self.dirty = false;
        self.recursive_dirty = false;
    }
    fn new_output_state(&self, input: &[bool], nfa: &NFA) -> Vec<bool> {
        let mut new_state = vec![false; nfa.states.len()];

        // 1. Propagate states from parent
        for i in 0..nfa.states.len() {
            if !input[i] {
                continue;
            }
            let Some(info) = &nfa.states[i] else {
                continue;
            };
            if info.is_final {
                continue;
            }
            // For descendant combinator, propagate to all descendants
            // For child combinator, only direct children can match
            match info.combinator {
                Combinator::Descendant => {
                    new_state[i] = true;
                }
                Combinator::Child => {
                    // Child combinator states don't propagate to grandchildren
                    // They only apply to direct children
                }
                Combinator::None => {
                    new_state[i] = true;
                }
            }
        }

        // 2. Compute new matches
        for i in 0..nfa.states.len() {
            let Some(info) = &nfa.states[i] else {
                continue;
            };
            if !self.matches_simple_selector(&info.selector) {
                continue;
            }
            let parent_matched = match info.parent_state {
                Some(parent_idx) => {
                    // Check if parent state is active and combinator allows match
                    match info.combinator {
                        Combinator::Child => {
                            // For child combinator, parent must be direct parent
                            self.is_direct_child_of_state(input, parent_idx)
                        }
                        Combinator::Descendant => {
                            // For descendant combinator, any ancestor match is fine
                            input[parent_idx]
                        }
                        Combinator::None => input[parent_idx],
                    }
                }
                None => true, // No parent needed, this is the start of a chain
            };
            if parent_matched {
                new_state[i] = true;
            }
        }
        new_state
    }
    fn collect_all_matches(
        &self,
        reverse_state_map: &HashMap<usize, CssMatch>,
        final_matches: &mut HashMap<CssMatch, Vec<u64>>,
    ) {
        for (bit_index, &is_match) in self.output_state.iter().enumerate() {
            if is_match {
                if let Some(rule) = reverse_state_map.get(&bit_index) {
                    final_matches.entry(rule.clone()).or_default().push(self.id);
                }
            }
        }

        for child in &self.children {
            child.collect_all_matches(reverse_state_map, final_matches);
        }
    }
    fn is_direct_child_of_state(&self, parent_input: &[bool], parent_state_idx: usize) -> bool {
        // Check if this node is a direct child of a node that has the parent_state active
        let Some(parent_ptr) = self.parent else {
            return false;
        };
        let parent = unsafe { &*parent_ptr };
        parent_input[parent_state_idx]
            && parent
                .output_state
                .get(parent_state_idx)
                .copied()
                .unwrap_or(false)
    }
}

fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = vec![];
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut selector_parts: Vec<SelectorPart> = vec![];
    let mut current_selector: Option<Selector> = None;
    let mut pending_combinator = Combinator::None;

    #[derive(PartialEq, Eq)]
    enum NextSelector {
        Class,
        Type,
    }
    let mut next_selector = NextSelector::Type;

    loop {
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token,
            Err(_) => {
                // End of input, finalize any pending rule
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                break;
            }
        };

        match token {
            Token::Comment(_) => continue,
            Token::WhiteSpace(_) => {
                if current_selector.is_some() && pending_combinator == Combinator::None {
                    pending_combinator = Combinator::Descendant;
                }
            }
            Token::Delim('.') => {
                next_selector = NextSelector::Class;
            }
            Token::Delim('>') => {
                if current_selector.is_some() {
                    pending_combinator = Combinator::Child;
                }
            }
            Token::IDHash(id) => {
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                    pending_combinator = Combinator::None;
                }
                current_selector = Some(Selector::Id(id.to_string()));
                next_selector = NextSelector::Type;
            }
            Token::Ident(name) => {
                let s = match next_selector {
                    NextSelector::Class => Selector::Class(name.to_string()),
                    NextSelector::Type => Selector::Type(name.to_string().to_lowercase()),
                };
                if let Some(prev_selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector: prev_selector,
                        combinator: pending_combinator.clone(),
                    });
                    pending_combinator = Combinator::None;
                }
                current_selector = Some(s);
                next_selector = NextSelector::Type;
            }
            Token::CurlyBracketBlock => {
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                selector_parts = vec![];
                current_selector = None;
                pending_combinator = Combinator::None;
                next_selector = NextSelector::Type;
            }
            _ => {
                // Any other token (like a comma) finalizes the current rule
                if let Some(selector) = current_selector.take() {
                    selector_parts.push(SelectorPart {
                        selector,
                        combinator: Combinator::None,
                    });
                }
                if !selector_parts.is_empty() {
                    rules.push(CssRule::Complex {
                        parts: selector_parts,
                    });
                }
                selector_parts = vec![];
                current_selector = None;
                pending_combinator = Combinator::None;
                next_selector = NextSelector::Type;
            }
        }
    }

    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    rules
}

fn build_nfa(state_map: &HashMap<CssMatch, usize>) -> NFA {
    let num_states = state_map.len();
    let mut states = vec![None; num_states];

    for (mch, &idx) in state_map {
        let (parts, is_final) = match mch {
            CssMatch::Doing { parts } => (parts, false),
            CssMatch::Done { parts } => (parts, true),
        };

        if parts.is_empty() {
            continue;
        }

        let last_part = &parts[parts.len() - 1];
        let selector = last_part.selector.clone();

        let parent_state = if parts.len() > 1 {
            let parent_rule = CssMatch::Doing {
                parts: parts[..parts.len() - 1].to_vec(),
            };
            Some(*state_map.get(&parent_rule).unwrap())
        } else {
            None
        };

        // The combinator for this state is the one from the previous part (if any)
        let combinator = if parts.len() >= 2 {
            parts[parts.len() - 2].combinator.clone()
        } else {
            Combinator::None
        };

        states[idx] = Some(NfaStateInfo {
            selector,
            parent_state,
            combinator,
            is_final,
        });
    }

    NFA { states }
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct LayoutFrame {
    pub frame_id: usize,
    pub command_name: String,
    pub command_data: serde_json::Value,
}

fn parse_trace() -> Vec<LayoutFrame> {
    let content = std::fs::read_to_string(format!(
        "css-gen-op/{0}/command.json",
        std::env::var("WEBSITE_NAME").unwrap()
    ))
    .unwrap();

    let mut frames = vec![];
    for (frame_id, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let command_data = serde_json::from_str::<serde_json::Value>(line).unwrap();

        let command_name = command_data["name"].as_str().unwrap().to_string();
        if command_name.starts_with("layout_") {
            continue;
        }

        frames.push(LayoutFrame {
            frame_id,
            command_name,
            command_data,
        });
    }

    frames
}

fn extract_path_from_command(command_data: &serde_json::Value) -> Vec<usize> {
    command_data
        .get("path")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|v| v as usize)
                .collect::<Vec<_>>()
        })
        .unwrap()
}

fn apply_frame(tree: &mut BitVectorHtmlNode, frame: &LayoutFrame, nfa: &NFA) {
    match frame.command_name.as_str() {
        "init" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            *tree =
                tree.json_to_html_node(frame.command_data.get("node").unwrap(), nfa.states.len());
            tree.fix_parent_pointers();
            // tree.recompute_styles(hm);
        }
        "add" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            if path.is_empty() {
                return;
            }
            tree.add_node_by_path(
                &path,
                frame.command_data.get("node").unwrap(),
                nfa.states.len(),
            );
        }
        "replace_value" | "insert_value" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "recalculate" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            let ii = vec![false; nfa.states.len()];
            let s = rdtsc();

            tree.recompute_styles(nfa, &ii);
            let e = rdtsc();
            println!("{}", e - s);
        }
        "remove" => {
            //  dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            tree.remove_node_by_path(&path);
        }
        _ => {
            // dbg!(frame.frame_id, frame.command_name.as_str());
        }
    }
}

fn main() {
    let css = parse_css(
        &std::fs::read_to_string(format!(
            "css-gen-op/{0}/{0}.css",
            std::env::var("WEBSITE_NAME").unwrap(),
        ))
        .unwrap(),
    );
    //dbg!(&css);
    let hm = {
        let mut hm = HashMap::new();
        for rule in &css {
            let CssRule::Complex { parts } = rule;
            let mut current_parts = vec![];
            for part in parts {
                current_parts.push(part.clone());
                let ss = if current_parts.len() == parts.len() {
                    CssMatch::Done {
                        parts: current_parts.clone(),
                    }
                } else {
                    CssMatch::Doing {
                        parts: current_parts.clone(),
                    }
                };
                if !hm.contains_key(&ss) {
                    hm.insert(ss, hm.len());
                }
            }
        }
        hm
    };
    // in this step, we map the Match status to usize
    let nfa = build_nfa(&hm);

    let mut bit = BitVectorHtmlNode::default();
    let trace = parse_trace();
    for i in &trace {
        apply_frame(&mut bit, i, &nfa);
    }
    // dbg!(&bit);
    let rev_hm = hm
        .iter()
        .filter_map(|(x, y)| match x {
            CssMatch::Doing { .. } => None,
            CssMatch::Done { .. } => Some((*y, x.clone())),
        })
        .collect();

    let mut final_matches = HashMap::new();
    bit.collect_all_matches(&rev_hm, &mut final_matches);
    let mut sorted_matches: Vec<_> = final_matches.into_iter().collect();
    sorted_matches.sort_by_key(|(rule, _)| format!("{rule:?}"));

    for (rule, mut node_ids) in sorted_matches {
        node_ids.sort_unstable();
        node_ids.dedup();
        println!("MATCH {}  -> {:?}", rule, node_ids);
    }
    dbg!(unsafe { MISS_CNT });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mixed_combinators() {
        // Test for "div a > h1 h2"
        let css_content = "div a > h1 h2 { color: red; }";
        let rules = parse_css(css_content);

        assert_eq!(rules.len(), 1);
        let CssRule::Complex { parts } = &rules[0];
        {
            assert_eq!(parts.len(), 4);

            // div (no combinator, it's the first)
            assert_eq!(parts[0].selector, Selector::Type("div".to_string()));
            assert_eq!(parts[0].combinator, Combinator::None);

            // a (descendant of div)
            assert_eq!(parts[1].selector, Selector::Type("a".to_string()));
            assert_eq!(parts[1].combinator, Combinator::Descendant);

            // h1 (child of a)
            assert_eq!(parts[2].selector, Selector::Type("h1".to_string()));
            assert_eq!(parts[2].combinator, Combinator::Child);

            // h2 (descendant of h1)
            assert_eq!(parts[3].selector, Selector::Type("h2".to_string()));
            assert_eq!(parts[3].combinator, Combinator::Descendant);
        }
    }

    #[test]
    fn test_child_selector() {
        let mut state_map = HashMap::new();

        // Create a rule for .foo > #bar
        let parts = vec![
            SelectorPart {
                selector: Selector::Class("foo".into()),
                combinator: Combinator::None,
            },
            SelectorPart {
                selector: Selector::Id("bar".into()),
                combinator: Combinator::Child,
            },
        ];

        let rule_father = CssMatch::Doing {
            parts: vec![parts[0].clone()],
        };
        let rule = CssMatch::Done {
            parts: parts.clone(),
        };

        state_map.insert(rule_father.clone(), state_map.len());
        state_map.insert(rule.clone(), state_map.len());
        let nfa = build_nfa(&state_map);

        let mut node = BitVectorHtmlNode::default();
        node.class = ["foo".into()].iter().cloned().collect::<HashSet<_>>();
        node.output_state = vec![false; nfa.states.len()];

        let mut child_node = BitVectorHtmlNode::default();
        child_node.html_id = Some("bar".into());
        child_node.output_state = vec![false; nfa.states.len()];

        node.children = vec![child_node];
        node.fix_parent_pointers();
        node.children[0].set_dirty();

        let input = vec![true, false];
        let output = node.children[0].new_output_state(&input, &nfa);
        assert_eq!(output, [true, true]);
    }
}
