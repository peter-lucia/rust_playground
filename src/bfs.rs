use std::collections::HashMap;
// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#defining-post-and-creating-a-new-instance-in-the-draft-state

pub struct Graph {
    graph: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Graph {
        return Graph {
            graph: [
                (1, vec![1,2,3]),
                (2, vec![4]),
                (3, vec![]),
                (4, vec![]),
            ].iter().cloned().collect(),
        }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) -> ()  {
        if (!self.graph.contains_key(&u)) {
            self.graph.insert(u, vec![v]);
        } else {
            self.graph.get_mut(&u).unwrap().push(v);
        }
    }

    pub fn bfs(&self) -> () {
        // traverse the graph with breadth first search
        // print the order of the traversal
    }

}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge(
            1,
            2
        );
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        graph.bfs()
    }
}