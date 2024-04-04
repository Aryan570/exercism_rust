pub mod graph {
    pub mod graph_items{
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone,Debug,PartialEq)]
            pub struct Edge{
                from : String,
                to : String,
                attributes : HashMap<String,String>
            }
            impl Edge {
                pub fn new(f : &str , t: &str) -> Self{
                    Edge{
                        from : f.to_string(),
                        to : t.to_string(),
                        attributes : HashMap::new()
                    }
                }
                pub fn with_attrs(mut self,v : &[(&str,&str)]) -> Self{
                    for (a,b) in v{
                        self.attributes.insert(a.to_string(), b.to_string());
                    }
                    self
                }
                pub fn attr(&self,s : &str) -> Option<&str>{
                    let ans = self.attributes.get(s);
                    if ans.is_some(){return Some(ans.unwrap())}
                    None
                }
            }

        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone,Debug,PartialEq)]
            pub struct Node{
                pub node : String,
                attributes : HashMap<String,String>
            }
            impl Node {
                pub fn new(n : &str) -> Self {
                    Node{
                        node : n.to_string(),
                        attributes : HashMap::new()
                    }
                }
                pub fn with_attrs(mut self,v : &[(&str,&str)]) -> Self{
                    for (a,b) in v{
                        self.attributes.insert(a.to_string(), b.to_string());
                    }
                    self
                }
                pub fn attr(&self,s : &str) -> Option<&str>{
                    let ans = self.attributes.get(s);
                    if ans.is_some(){return Some(ans.unwrap())}
                    None
                }
            }

        }
    }
    use std::collections::HashMap;
    use crate::graph::graph_items::*;
    pub struct Graph{
        pub nodes : Vec<node::Node>,
        pub edges : Vec<edge::Edge>,
        pub attrs : HashMap<String,String>
    }
    impl Graph {
        pub fn new() -> Self {
            Graph{
                nodes : Vec::new(),
                edges : Vec::new(),
                attrs : HashMap::new()
            }
        }
        pub fn with_nodes(mut self,nodes_array : &Vec<node::Node>) -> Self {
            self.nodes = nodes_array.to_vec();
            self
        }
        pub fn with_edges(mut self, edge_array: &Vec<edge::Edge>) -> Self{
            self.edges = edge_array.to_vec();
            self
        }
        pub fn with_attrs(mut self,v : &[(&str,&str)]) -> Self{
            for (a,b) in v{
                self.attrs.insert(a.to_string(), b.to_string());
            }
            self
        }
        pub fn node(&self,s : &str) -> Option<node::Node>{
            for p in &self.nodes{
                if p.node == s{
                    return Some(p.clone());
                }
            }
            None
        }
    }
}
