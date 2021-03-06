use std::collections::{HashMap, VecDeque};

pub struct Graph {
    graph: HashMap<usize, Vec<usize>>,
    size: usize,
}

impl Graph {
    pub fn new(s: usize) -> Graph {

        let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..s+1 {
            // ensures a value is in the entry by inserting the default if empty
            adj_list.entry(i).or_insert(vec![]);
        }

        return Graph {
            size: s,
            graph: adj_list,
        }
    }

    /// Add an edge to the graph stored as an adj list
    pub fn add_edge(&mut self, u: usize, v: usize) -> ()  {
        if !self.graph.contains_key(&u) {
            self.graph.insert(u, vec![v]);
        } else {
            self.graph.get_mut(&u).unwrap().push(v);
        }
    }

    /// Driver function to run recursive dfs from the starting vertex
    /// Returns a vector of the vertices encountered
    /// during the dfs traversal
    pub fn dfs_driver(&self, u: usize) -> Vec<usize> {

        let visited: &mut Vec<bool>  = &mut vec![false; self.graph.len() + 1];
        let result: &mut Vec<usize> = &mut vec![];

        self.dfs(u, visited, result);
        return (*result).clone();
    }

    /// Traverses the graph with depth first search
    /// starting at the provided vertex, u.
    /// Modifies the result in place since it's mutable
    fn dfs(&self, u: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) -> () {

        if visited[u] {
            return;
        }

        visited[u] = true;
        result.push(u);

        for neighbor in &self.graph[&u] {
            // use take to pass ownership to recursive call
            self.dfs(*neighbor, visited, result);
        }
        return;
    }

    /// traverse the graph with breadth first search
    /// return the order of the traversal
    pub fn bfs(&self, v_start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.graph.len()+1];
        let mut result = vec![];

        let mut queue: VecDeque<usize> = VecDeque::from(vec![v_start]);

        while !queue.is_empty() {

            let w = queue.pop_front().unwrap();

            visited[w] = true;
            result.push(w);

            let neighbors  = self.graph.get(&w);

            if neighbors.is_none() {
                continue;
            }

            for neighbor in neighbors.unwrap() {
                if visited[*neighbor] {
                    continue;
                }
                visited[*neighbor] = true;
                queue.push_back(*neighbor);
            }
        }
        return result;
    }

}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        assert_eq!(graph.size, 2);
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        let result = graph.bfs(0);
        assert_eq!(result, vec![0,1,2,3])
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        let result = graph.dfs_driver(0);
        assert_eq!(result, vec![0,1,2,3])
    }
}