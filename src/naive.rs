use std::collections::HashSet;

use css_bitvector_compiler::Cache;

// note: do nt pull out bitvector result; - absvector will change that laters
// to other type struct NaiveCache {
// no dirty node anywhere, have to recompute from scratch
// bitvector result;
//}
impl NaiveHtmlNode {
    fn init(&mut self) {
        let s = std::fs::read_to_string(format!(
            "css-gen-op/{}/command.json",
            std::env::var("WEBSITE_NAME").unwrap()
        ))
        .unwrap();
        let first_line = s.lines().next().unwrap();
        let trace_data: serde_json::Value = serde_json::from_str(first_line).unwrap();
        *self = self.json_dom_to_html_node(&trace_data["node"]);
    }

    fn json_dom_to_html_node(&mut self, json_node: &serde_json::Value) -> Self {
        let mut node = Self::default();
        //  dbg!(&json_node);
        node.tag_name = json_node["name"].as_str().unwrap().into();
        node.id = json_node["id"].as_u64().unwrap();

        // Add classes from attributes
        node.classes = {
            let attributes = json_node["attributes"].as_object().unwrap();
            let class_str = attributes
                .get("class")
                .map(|x| x.as_str().unwrap())
                .unwrap_or_default();
            class_str
                .split_whitespace()
                .map(|x| x.into())
                .collect::<HashSet<String>>()
        };

        // Add children recursively
        node.children = {
            let children = json_node["children"].as_array().unwrap();
            children
                .into_iter()
                .map(|x| self.json_dom_to_html_node(x))
                .collect()
        };
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
}
#[derive(Debug, Default)]

struct NaiveHtmlNode {
    pub tag_name: String,
    pub id: u64,
    pub classes: HashSet<String>,
    pub children: Vec<NaiveHtmlNode>,
    pub parent: Option<*mut NaiveHtmlNode>, // TODO: use u64 in future
}

impl Cache<NaiveHtmlNode> for NaiveHtmlNode {
    fn dirtied(&mut self, path: &[u64]) {
        unimplemented!()
    }
    fn recompute(&mut self, root: &mut NaiveHtmlNode) {
        unimplemented!()
    }
}

// Maybe called Cached?

// 分离 3 种不同的 node, naive , bit, tri
// 对每种 node, 实现一个公共的 trait, recompute, dirtied.
// recompute 是实际做计算的
// dirtied 只是做脏标记
fn main() {
    let mut bit = NaiveHtmlNode::default();
    bit.init();
    dbg!(bit);
}
