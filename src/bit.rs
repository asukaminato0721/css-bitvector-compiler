use css_bitvector_compiler::rdtsc;
use cssparser::{Parser, ParserInput, Token};
use std::collections::{HashMap, HashSet};

static mut MISS_CNT: usize = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CssRule {
    Descendant { selectors: Vec<Selector> },
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Selector {
    Type(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Default)]

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
    fn json_to_html_node(
        &mut self,
        json_node: &serde_json::Value,
        hm: &HashMap<CssRule, usize>,
    ) -> Self {
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
                .map(|x| self.json_to_html_node(x, &hm))
                .collect()
        };
        node.output_state = vec![false; hm.len()];
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
        hm: &HashMap<CssRule, usize>,
    ) {
        assert!(!path.is_empty());
        if path.len() > 1 {
            self.children[path[0]].add_node_by_path(&path[1..], json_node, &hm);
            return;
        }
        let new_n = self.json_to_html_node(json_node, hm);
        self.children.insert(path[0], new_n);
        self.set_dirty();
        self.fix_parent_pointers();
    }
    fn record_remove(&mut self, path: &[usize]) {
        assert!(!path.is_empty());
        if path.len() > 1 {
            self.children[path[0]].record_remove(&path[1..]);
            return;
        }
        self.children.remove(path[0]);
        self.set_dirty();
    }
    fn recompute_styles(&mut self, state_map: &HashMap<CssRule, usize>) {
        if !self.recursive_dirty {
            return;
        }
        if self.dirty {
            unsafe {
                MISS_CNT += 1;
            }
            let new_output_state = self.new_output_state(&self.output_state, state_map);
            if self.output_state != new_output_state {
                self.output_state = new_output_state;
                for c in self.children.iter_mut() {
                    c.set_dirty();
                }
            }
        }
        for child in self.children.iter_mut() {
            child.recompute_styles(state_map);
        }
        self.dirty = false;
        self.recursive_dirty = false;
    }
    fn new_output_state(&self, input: &[bool], state_map: &HashMap<CssRule, usize>) -> Vec<bool> {
        let mut new_state = input.to_vec();

        for (CssRule::Descendant { selectors }, &bit_index) in state_map {
            let last_selector = selectors.last().unwrap();

            if !self.matches_simple_selector(last_selector) {
                continue;
            }

            if selectors.len() == 1 {
                new_state[bit_index] = true;
            } else {
                let parent_selectors = &selectors[..selectors.len() - 1];
                let parent_rule = CssRule::Descendant {
                    selectors: parent_selectors.to_vec(),
                };

                if let Some(&parent_bit_index) = state_map.get(&parent_rule) {
                    if input[parent_bit_index] {
                        new_state[bit_index] = true;
                    }
                }
            }
        }

        new_state
    }
    fn collect_all_matches(
        &self,
        reverse_state_map: &HashMap<usize, CssRule>,
        final_matches: &mut HashMap<CssRule, Vec<u64>>,
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
}

fn parse_css(css_content: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut input = ParserInput::new(css_content);
    let mut parser = Parser::new(&mut input);

    let mut expecting_rule_body = false;
    let mut selector_chain: Vec<Selector> = Vec::new();
    let mut current_selector: Option<Selector> = None;

    while let Ok(token) = parser.next() {
        if expecting_rule_body {
            match token {
                Token::CurlyBracketBlock => {
                    if let Some(selector) = current_selector.take() {
                        selector_chain.push(selector);
                    }
                    if !selector_chain.is_empty() {
                        rules.push(CssRule::Descendant {
                            selectors: selector_chain,
                        });
                    }

                    selector_chain = Vec::new();
                    expecting_rule_body = false;
                }
                _ => {
                    expecting_rule_body = false;
                    current_selector = None;
                    selector_chain.clear();
                }
            }
        } else {
            match token {
                Token::Ident(name) => {
                    let type_name = name.to_string().to_lowercase();

                    if let Some(prev_selector) = current_selector.take() {
                        selector_chain.push(prev_selector);
                    }

                    current_selector = Some(Selector::Type(type_name));
                }
                Token::IDHash(id) => {
                    if let Some(prev_selector) = current_selector.take() {
                        selector_chain.push(prev_selector);
                    }

                    current_selector = Some(Selector::Id(id.to_string()));
                }
                Token::Delim('.') => {
                    if let Ok(Token::Ident(class_name)) = parser.next() {
                        if let Some(prev_selector) = current_selector.take() {
                            selector_chain.push(prev_selector);
                        }

                        current_selector = Some(Selector::Class(class_name.to_string()));
                    }
                }
                Token::CurlyBracketBlock => {
                    if let Some(selector) = current_selector.take() {
                        selector_chain.push(selector);
                        rules.push(CssRule::Descendant {
                            selectors: selector_chain,
                        });
                    }
                    selector_chain = Vec::new();
                }
                Token::WhiteSpace(_) => {
                    if current_selector.is_some() {
                        expecting_rule_body = true;
                    }
                }
                _ => {
                    current_selector = None;
                    selector_chain.clear();
                }
            }
        }
    }

    rules.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    rules.dedup();
    rules
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

    let mut frames = Vec::new();
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

fn apply_frame(tree: &mut BitVectorHtmlNode, frame: &LayoutFrame, hm: &HashMap<CssRule, usize>) {
    match frame.command_name.as_str() {
        "init" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            *tree = tree.json_to_html_node(frame.command_data.get("node").unwrap(), &hm);
            tree.fix_parent_pointers();
        }
        "add" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            if path.is_empty() {
                return;
            }
            tree.add_node_by_path(&path, frame.command_data.get("node").unwrap(), &hm);
        }
        "replace_value" | "insert_value" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
        }
        "recalculate" => {
            //   dbg!(frame.frame_id, frame.command_name.as_str());
            let s = rdtsc();
            tree.recompute_styles(hm);
            let e = rdtsc();
            println!("{}", e - s);
        }
        "remove" => {
            //  dbg!(frame.frame_id, frame.command_name.as_str());
            let path = extract_path_from_command(&frame.command_data);
            tree.record_remove(&path);
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
        for CssRule::Descendant { selectors } in &css {
            let mut v = vec![];
            for s in selectors {
                v.push(s.clone());
                let ss = CssRule::Descendant {
                    selectors: v.clone(),
                };
                if !hm.contains_key(&ss) {
                    hm.insert(ss, hm.len());
                }
            }
        }
        hm
    };
    //  dbg!(&hm);

    let final_rules_map: HashMap<_, _> = css
        .iter()
        .map(|rule| {
            let bit_index = *hm.get(rule).unwrap();
            (rule.clone(), bit_index)
        })
        .collect();

    let mut bit = BitVectorHtmlNode::default();
    let trace = parse_trace();
    for i in &trace {
        apply_frame(&mut bit, &i, &hm);
    }
    let rev_hm = final_rules_map
        .iter()
        .map(|(x, &y)| (y, x.clone()))
        .collect::<HashMap<_, _>>();

    let mut final_matches = HashMap::new();
    bit.collect_all_matches(&rev_hm, &mut final_matches);
    let mut sorted_matches: Vec<_> = final_matches.into_iter().collect();
    sorted_matches.sort_by_key(|(rule, _)| format!("{rule:?}"));

    for (rule, mut node_ids) in sorted_matches {
        node_ids.sort_unstable();
        node_ids.dedup();
        println!("{:?} -> {:?}", rule, node_ids);
    }
    dbg!(unsafe { MISS_CNT });
}
