use std::collections::HashSet;

use css_bitvector_compiler::Cache;

//}
#[derive(Debug, Default)]
struct BitVectorCache {
    dirtynode: bool,
    result: Vec<bool>,
}

#[derive(Debug, Default)]

struct BitVectorHtmlNode {
    pub tag_name: String,
    pub id: u64,
    pub classes: HashSet<String>,
    pub children: Vec<BitVectorHtmlNode>,
    pub parent: Option<*mut BitVectorHtmlNode>, // TODO: use u64 in future
    cache: BitVectorCache,
}

impl BitVectorHtmlNode {
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

impl Cache<BitVectorHtmlNode> for BitVectorHtmlNode {
    fn dirtied(&mut self, path: &[u64]) {
        if path.is_empty() {
            self.cache.dirtynode = true;
            return;
        }
        self.dirtied(&path[1..]);
    }
    fn recompute(&mut self, root: &mut BitVectorHtmlNode) {
        unimplemented!()
    }
}

fn main() {
    let mut bit = BitVectorHtmlNode::default();
    bit.init();
    dbg!(bit);
}
