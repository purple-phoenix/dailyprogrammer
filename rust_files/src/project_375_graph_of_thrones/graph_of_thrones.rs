use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd{
    num_nodes: usize,
    num_edges: usize,
    graph: HashMap<T, Vec<(T, bool)>>
}



impl <T> UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd + Copy {
    pub fn is_balanced(&self) -> bool {
        if self.is_simple_graph() {
            return false;
        }
        else {
            let sub_graphs = self.make_sub_graphs();
            let mut all_sub_graphs_are_balanced = true;
            for a_sub_graph in sub_graphs {
                all_sub_graphs_are_balanced &= a_sub_graph.is_balanced();
            }
            return all_sub_graphs_are_balanced;
        }
    }

    pub fn make_graph(graph: &HashMap<T, Vec<(T, bool)>>) -> UndirectedCompleteGraph<T> {
        let mut num_nodes = 0;
        let mut num_edges = 0;
        let mut seen_edges = Vec::with_capacity(graph.len() * 2);
        let mut graph_clone = HashMap::new();
        for (node, edges) in graph {
            let mut new_edges = vec![];
            for edge in edges {
                if !(seen_edges.contains(&(node, edge)) ||
                    seen_edges.contains(&(&edge.0, &(*node, edge.1)))) {
                    num_edges += 1;
                    seen_edges.push((node, edge));
                    new_edges.push(edge.clone())
                }
            }
            graph_clone.insert(node.clone(), new_edges.clone());
            num_nodes += 1;
        }
        return UndirectedCompleteGraph {num_nodes, num_edges, graph: graph_clone}
    }

    pub fn get_num_nodes(&self) -> usize {
        return self.num_nodes;
    }

    pub fn get_num_edges(&self) -> usize {
        return self.num_edges;
    }

    pub fn get_graph(&self) -> HashMap<T, Vec<(T, bool)>> {
        return copy_map(&self.graph);
    }

    pub fn copy_graph(&self) -> UndirectedCompleteGraph<T> {
        return UndirectedCompleteGraph {
            num_edges: self.num_edges,
            num_nodes: self.num_nodes,
            graph: copy_map(&self.graph)
        }
    }

    fn is_simple_graph(&self) -> bool {
        return self.num_nodes == 3;
    }

    fn make_sub_graphs(&self) -> Vec<UndirectedCompleteGraph<T>> {
        if self.is_simple_graph() {
            return vec![self.copy_graph()]
        }
        return vec![]
    }
}


fn maps_are_equal<T, E>(map1: &HashMap<T, E>, map2: &HashMap<T, E>) -> bool
    where T: Eq + Hash, E: Eq
{
    if map1.len() != map2.len() {
        return false;
    }
    else {
        let mut all_key_vals_are_equal = true;
        for (key, val) in map1 {
            if !map2.contains_key(key) {
                return false;
            }
            else {
                let map2_value = map2.get(key).unwrap();
                all_key_vals_are_equal &= val == map2_value
            }
        }
        return all_key_vals_are_equal;
    }
}

fn copy_map<T, E>(map: &HashMap<T, E>) -> HashMap<T, E> where T: Hash + Clone + Eq, E: Clone + Eq {
    let mut new_map = HashMap::new();
    for (key, value) in map {
        new_map.insert(key.clone(), value.clone());
    }
    return new_map;
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::project_375_graph_of_thrones::graph_of_thrones::{UndirectedCompleteGraph, maps_are_equal};

    #[test]
    fn test_make_graph() {
        let node1 = "Node1";
        let node2 = "Node2";
        let node3 = "Node3";
        let mut simple_graph = HashMap::new();
        simple_graph.insert(node1, vec![(node2, true), (node3, false)]);
        simple_graph.insert(node2, vec![(node1, true), (node3, false)]);
        simple_graph.insert(node3, vec![(node1, false), (node2, false)]);
        let actual_graph =
            UndirectedCompleteGraph::make_graph(&simple_graph);

        assert_eq!(actual_graph.get_num_edges(), 3)

    }

    #[test]
    fn test_is_balanced() {
        let node1 = "Node1";
        let node2 = "Node2";
        let node3 = "Node3";
        let mut simple_graph = HashMap::new();
        simple_graph.insert(node1, vec![(node2, true), (node3, false)]);
        simple_graph.insert(node2, vec![(node1, true), (node3, false)]);
        simple_graph.insert(node3, vec![(node1, false), (node2, false)]);
        let actual_graph =
            UndirectedCompleteGraph::make_graph(&simple_graph);

        assert!(actual_graph.is_balanced());

        let node4 = "Node4";

        let mut unbalanced_graph = HashMap::new();
        unbalanced_graph.insert(node1, vec![(node2, false), (node3, false), (node4, false)]);
        unbalanced_graph.insert(node2, vec![(node3, true), (node4, true)]);
        unbalanced_graph.insert(node3, vec![(node4, false)]);
        unbalanced_graph.insert(node4, vec![]);
        let unbalanced_graph_struct =
            UndirectedCompleteGraph::make_graph(&unbalanced_graph);

        assert!(!unbalanced_graph_struct.is_balanced());

    }

    #[test]
    fn test_is_simple_graph() {
        let node1 = "Node1";
        let node2 = "Node2";
        let node3 = "Node3";
        let mut simple_graph = HashMap::new();
        simple_graph.insert(node1, vec![(node2, true), (node3, false)]);
        simple_graph.insert(node2, vec![(node1, true), (node3, false)]);
        simple_graph.insert(node3, vec![(node1, false), (node2, false)]);
        let actual_graph =
            UndirectedCompleteGraph::make_graph(&simple_graph);

        assert!(actual_graph.is_simple_graph());

        let node4 = "Node4";

        let mut unbalanced_graph = HashMap::new();
        unbalanced_graph.insert(node1, vec![(node2, false), (node3, false), (node4, false)]);
        unbalanced_graph.insert(node2, vec![(node3, true), (node4, true)]);
        unbalanced_graph.insert(node3, vec![(node4, false)]);
        unbalanced_graph.insert(node4, vec![]);
        let unbalanced_graph_struct =
            UndirectedCompleteGraph::make_graph(&unbalanced_graph);

        assert!(!unbalanced_graph_struct.is_simple_graph());

    }



    #[test]
    fn test_make_sub_graphs() {
        let node1 = "Node1";
        let node2 = "Node2";
        let node3 = "Node3";
        let node4 = "Node4";

        let mut simple_graph = HashMap::new();
        simple_graph.insert(node1, vec![(node2, true), (node3, false)]);
        simple_graph.insert(node2, vec![(node1, true), (node3, false)]);
        simple_graph.insert(node3, vec![(node1, false), (node2, false)]);
        let actual_graph =
            UndirectedCompleteGraph::make_graph(&simple_graph);

        let sub_graphs = actual_graph.make_sub_graphs();
        assert_eq!(sub_graphs.len(), 1);
        let before_graph = actual_graph.get_graph();
        let sub_graph = sub_graphs.get(0).unwrap().get_graph();
        println!("{:?}...{:?}", before_graph, sub_graph);
        assert!(maps_are_equal(&before_graph, &sub_graph));



        let mut unbalanced_graph = HashMap::new();
        unbalanced_graph.insert(node1, vec![(node2, false), (node3, false), (node4, false)]);
        unbalanced_graph.insert(node2, vec![(node3, true), (node4, true)]);
        unbalanced_graph.insert(node3, vec![(node4, false)]);
        unbalanced_graph.insert(node4, vec![]);
        let unbalanced_graph_struct =
            UndirectedCompleteGraph::make_graph(&unbalanced_graph);

        let sub_graphs = unbalanced_graph_struct.make_sub_graphs();

    }

}