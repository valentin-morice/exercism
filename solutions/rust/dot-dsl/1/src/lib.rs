pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::{edge::Edge, extend_attrs, node::Node};

    pub mod graph_items {
        use std::collections::HashMap;

        pub(crate) fn extend_attrs(
            attrs: &mut HashMap<String, String>,
            attributes: &[(&str, &str)],
        ) {
            attrs.extend(
                attributes
                    .iter()
                    .map(|&(k, v)| (k.to_string(), v.to_string())),
            );
        }

        pub mod edge {
            use std::collections::HashMap;

            use super::extend_attrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                endpoints: (String, String),
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(first: &str, second: &str) -> Self {
                    Self {
                        endpoints: (first.to_string(), second.to_string()),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                    extend_attrs(&mut self.attrs, attributes);
                    self
                }

                pub fn attr(&self, attribute: &str) -> Option<&str> {
                    self.attrs.get(attribute).map(|v| v.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            use super::extend_attrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn name(&self) -> &str {
                    &self.name
                }

                pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
                    extend_attrs(&mut self.attrs, attributes);
                    self
                }

                pub fn attr(&self, attribute: &str) -> Option<&str> {
                    self.attrs.get(attribute).map(|v| v.as_str())
                }
            }
        }
    }

    #[derive(PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
            extend_attrs(&mut self.attrs, attributes);
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name() == name).cloned()
        }
    }
}
