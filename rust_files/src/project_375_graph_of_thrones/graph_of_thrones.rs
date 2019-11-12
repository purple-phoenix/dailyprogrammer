use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd{
    num_nodes: usize,
    num_edges: usize,
    graph: HashMap<T, Vec<T>>
}



impl <T> UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd{
    pub fn is_balanced() -> bool {
        return false
    }

    pub fn make_graph(graph: HashMap<T, Vec<T>>) -> UndirectedCompleteGraph<T> {
        let mut num_nodes = 0;
        let mut num_edges = 0;
        let mut seen_edges = Vec::with_capacity(graph.len() * 2);
        for (node, edges) in &graph {
            for edge in edges {
                if !(seen_edges.contains(&(node, edge)) || seen_edges.contains(&(edge, node))) {
                    num_edges += 1;
                    seen_edges.push((node, edge));
                }
            }
            num_nodes += 1;
        }
        return UndirectedCompleteGraph {num_nodes, num_edges, graph}
    }

    pub fn get_num_nodes(&self) -> usize {
        return self.num_nodes;
    }

    pub fn get_num_edges(&self) -> usize {
        return self.num_edges;
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::project_375_graph_of_thrones::graph_of_thrones::UndirectedCompleteGraph;

    #[test]
    fn test_make_graph() {
        let node1 = "Node1";
        let node2 = "Node2";
        let node3 = "Node3";
        let mut simple_graph = HashMap::new();
        simple_graph.insert(node1, vec![node2, node3]);
        simple_graph.insert(node2, vec![node1, node3]);
        simple_graph.insert(node3, vec![node1, node2]);
        let actual_graph =
            UndirectedCompleteGraph::make_graph(simple_graph);

        assert_eq!(actual_graph.get_num_edges(), 3)

    }

    #[test]
    fn test_is_balanced() {
        let simple_graph = 1;


    }

}