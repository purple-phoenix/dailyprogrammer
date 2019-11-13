use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug)]
pub struct UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd{
    num_nodes: usize,
    num_edges: usize,
    graph: HashMap<T, Vec<(T, bool)>>
}



impl <T> UndirectedCompleteGraph<T> where T: Eq + Hash + PartialOrd + Copy + Debug {
    pub fn is_balanced(&self) -> bool {
        if self.is_simple_graph() {
            let mut num_falses = 0;
            for (node, edges) in self.get_graph() {
                for an_edge in edges {
                    if !an_edge.1 {
                        num_falses += 1;
                    }
                }
            }
            return num_falses % 2 == 0;
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


        let mut known_edges: HashMap<(T, T), bool> = HashMap::with_capacity(self.num_edges);
        for (node, edges) in &self.graph {
            for edge in edges {
                known_edges.insert((node.clone(), edge.0.clone()), edge.1);
            }
        }

        let mut sub_graphs = vec![];


        for known_edge in &known_edges {
            let first_node = (known_edge.0).0;
            let second_node = (known_edge.0).1;
            let first_second_edge = *known_edge.1;
            for maybe_third_edge in copy_map(&known_edges) {
                let maybe_third_edge_node_1 = (maybe_third_edge.0).0;
                let maybe_third_edge_node_2 = (maybe_third_edge.0).1;
                if (first_node == maybe_third_edge_node_1 || second_node == maybe_third_edge_node_1)
                    && maybe_third_edge_node_2 != first_node && maybe_third_edge_node_2 != second_node {
                    //Maybe third edge 2 is unique
                    if *(&known_edges.contains_key(&(first_node, maybe_third_edge_node_2))){
                        let maybe_third_edge =
                            *known_edges.get(&(first_node, maybe_third_edge_node_2)).unwrap();

                        if *(&known_edges.contains_key(&(second_node, maybe_third_edge_node_2))) {
                            let final_edge = *known_edges.get(&(second_node, maybe_third_edge_node_2)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_2, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_2, final_edge)]);
                            graph.insert(maybe_third_edge_node_2, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }
                        }
                        else if *(&known_edges.contains_key(&(maybe_third_edge_node_2, second_node))) {
                            let final_edge = known_edges.get(&(maybe_third_edge_node_2, second_node)).unwrap();
                        }



                    }else if *(&known_edges.contains_key(&(maybe_third_edge_node_2, first_node))) {
                        let maybe_third_edge =
                            *known_edges.get(&(maybe_third_edge_node_2, first_node)).unwrap();

                        if *(&known_edges.contains_key(&(second_node, maybe_third_edge_node_2))) {
                            let final_edge = *known_edges.get(&(second_node, maybe_third_edge_node_2)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_2, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_2, final_edge)]);
                            graph.insert(maybe_third_edge_node_2, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }
                        }
                        else if *(&known_edges.contains_key(&(maybe_third_edge_node_2, second_node))) {
                            let final_edge = known_edges.get(&(maybe_third_edge_node_2, second_node)).unwrap();
                        }


                    }
                }
                else if (first_node == maybe_third_edge_node_2 || second_node == maybe_third_edge_node_2)
                    && maybe_third_edge_node_1 != first_node && maybe_third_edge_node_1 != second_node{
                    //Maybe third edge 1 is unique
                    if *(&known_edges.contains_key(&(first_node, maybe_third_edge_node_1))){
                        let maybe_third_edge =
                            *known_edges.get(&(first_node, maybe_third_edge_node_1)).unwrap();

                        if *(&known_edges.contains_key(&(second_node, maybe_third_edge_node_1))) {
                            let final_edge = *known_edges.get(&(second_node, maybe_third_edge_node_1)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_1, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_1, final_edge)]);
                            graph.insert(maybe_third_edge_node_1, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }

                        }
                        else if *(&known_edges.contains_key(&(maybe_third_edge_node_1, second_node))) {
                            let final_edge = *known_edges.get(&(maybe_third_edge_node_1, second_node)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_1, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_1, final_edge)]);
                            graph.insert(maybe_third_edge_node_1, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }
                        }



                    }else if *(&known_edges.contains_key(&(maybe_third_edge_node_1, first_node))) {
                        let maybe_third_edge =
                            *known_edges.get(&(maybe_third_edge_node_1, first_node)).unwrap();

                        if *(&known_edges.contains_key(&(second_node, maybe_third_edge_node_1))) {
                            let final_edge = *known_edges.get(&(second_node, maybe_third_edge_node_1)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_1, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_1, final_edge)]);
                            graph.insert(maybe_third_edge_node_1, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }

                        }
                        else if *(&known_edges.contains_key(&(maybe_third_edge_node_1, second_node))) {
                            let final_edge = *known_edges.get(&(maybe_third_edge_node_1, second_node)).unwrap();
                            let mut graph = HashMap::with_capacity(3);
                            graph.insert(first_node, vec![(maybe_third_edge_node_1, maybe_third_edge), (second_node, first_second_edge)]);
                            graph.insert(second_node, vec![(maybe_third_edge_node_1, final_edge)]);
                            graph.insert(maybe_third_edge_node_1, vec![]);
                            if !graph_contains_map(&sub_graphs, &graph) {
                                sub_graphs.push(UndirectedCompleteGraph::make_graph(&graph));
                            }

                        }




                    }
                }
            }
        }

        return sub_graphs;
        }



    }

fn graph_contains_map<T>(graphs: &Vec<UndirectedCompleteGraph<T>>, map: &HashMap<T, Vec<(T, bool)>>)  -> bool
    where T: PartialOrd + Eq + Hash + Debug + Copy {
    if graphs.is_empty() {
        return false;
    }
    let mut maps = vec![];
    let mut no_maps_equal = true;
    for graph in graphs {
        maps.push(graph.get_graph());
    }

    for a_map in maps {
        no_maps_equal &= !maps_contain_same_elms(&a_map, map)
    }
    return !no_maps_equal
}


fn maps_contain_same_elms<T>(map1: &HashMap<T, Vec<(T, bool)>>, map2: &HashMap<T, Vec<(T, bool)>>) -> bool
    where T: Eq + Hash + Clone + Copy + Debug {
    let mut same_elms = true;
    let mut reference_dictionary_1 = HashMap::new();
    let mut reference_dictionary_2 = HashMap::new();
    let mut nodes1 = vec![];
    let mut nodes2 = vec![];
    for (node, edges) in copy_map(map1) {
        nodes1.push(node.clone());
        for edge in edges {
            let edge_node = edge.0;
            *reference_dictionary_1.entry(edge_node.clone()).or_insert(0) += 1;
        }
    }

    for (node, edges) in copy_map(map2) {
        nodes2.push(node.clone());
        for edge in edges {
            let edge_node = edge.0;
            let edge_node_clone = edge_node.clone();
            *reference_dictionary_2.entry(edge_node_clone).or_insert(0) += 1;
        }
    }

    let mut same_nodes = true;
    for a_node in nodes1 {
        same_nodes &= nodes2.contains(&a_node);
    }

    let references_equal = maps_are_equal(&reference_dictionary_1, &reference_dictionary_2);
    return references_equal || same_nodes;
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

fn make_graph_from_lines(lines: Vec<&str>) -> UndirectedCompleteGraph<&str> {
    let num_nodes = 0;
    let num_edges = 0;
    let mut graph: HashMap<&str, Vec<(&str, bool)>> = HashMap::new();
    for line in lines[1..].to_vec() {
        println!("{}", line);
        if line.starts_with("%d %d") {
            continue;
        }
        let node_and_edge = convert_line_to_graph_input(line);
        let node = node_and_edge.0;
        let edge = node_and_edge.1;
        let maybe_current_list = graph.get_mut(node);
        match maybe_current_list {
            Some(list) => {
                list.push(edge);
            }
            None => {
                graph.insert(node, vec![edge]);
            }
        }
    }
    let mut nodes: Vec<&str>= vec![];
    for (node, edges) in &graph {
        for edge in edges {
            let edge_node = edge.0;
            if !nodes.contains(&edge_node) {
                nodes.push(edge_node);
            }
        }
    }

    for node in nodes {
        if !graph.contains_key(node) {
            graph.insert(node, vec![]);
        }
    }

    for (node, edges) in &graph {
        println!("{}   {:?}", node, edges);
    }
    return UndirectedCompleteGraph{
        num_nodes,
        num_edges,
        graph
    }
}

fn convert_line_to_graph_input(line: &str) -> (&str, (&str, bool)) {
    let pos_sep = " ++ ";
    if line.contains(pos_sep) {
        let node_edge_node_list: Vec<&str> = line.split(pos_sep).collect();
        let node = node_edge_node_list[0];
        let edge_node = node_edge_node_list[1];
        return (node, (edge_node, true))
    }
    else {
        let node_edge_node_list: Vec<&str> = line.split(" -- ").collect();
        let node = node_edge_node_list[0];
        let edge_node = node_edge_node_list[1];
        return (node, (edge_node, false))
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::project_375_graph_of_thrones::graph_of_thrones::{UndirectedCompleteGraph, maps_are_equal, graph_contains_map, make_graph_from_lines, convert_line_to_graph_input};

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
        assert!(maps_are_equal(&before_graph, &sub_graph));


        let mut unbalanced_graph = HashMap::new();
        unbalanced_graph.insert(node1, vec![(node2, false), (node3, false), (node4, false)]);
        unbalanced_graph.insert(node2, vec![(node3, true), (node4, true)]);
        unbalanced_graph.insert(node3, vec![(node4, false)]);
        unbalanced_graph.insert(node4, vec![]);
        let unbalanced_graph_struct =
            UndirectedCompleteGraph::make_graph(&unbalanced_graph);

        let sub_graphs = unbalanced_graph_struct.make_sub_graphs();

        assert_eq!(sub_graphs.len(), 4);

    }

    #[test]
    fn test_graph_contains_map() {
        let node1 = "Node1";
        let node3 = "Node3";
        let node2 = "Node2";
        let node4 = "Node4";
        let mut graph1 = HashMap::new();
        graph1.insert(node3, vec![(node4, false)]);
        graph1.insert(node1, vec![(node4, false), (node3, false)]);
        graph1.insert(node4, vec![]);
        let mut graph2 = HashMap::new();
        graph2.insert(node2, vec![]);
        graph2.insert(node3, vec![(node2, false)]);
        graph2.insert(node1, vec![(node2, false), (node3, false)]);

        let graph1_struct
            = UndirectedCompleteGraph::make_graph(&graph1);

        let list_of_graph
            = vec![UndirectedCompleteGraph::make_graph(&graph1)];

        assert!(graph_contains_map(&list_of_graph, &graph1));

        let mut graph3 = HashMap::new();
        graph3.insert(node3, vec![]);
        graph3.insert(node4, vec![(node3, false)]);
        graph3.insert(node1, vec![(node3, false), (node4, false)]);

        let mut graph4 = HashMap::new();
        graph4.insert(node4, vec![]);
        graph4.insert(node1, vec![(node4, false), (node3, false)]);
        graph4.insert(node3, vec![(node4, false)]);

        let list_of_graph2 =
            vec![UndirectedCompleteGraph::make_graph(&graph3)];
        assert!(graph_contains_map(&list_of_graph2, &graph4));

    }

    #[test]
    fn test_make_graph_from_lines() {
        let lines = vec![
            "6 15",
            "Superman ++ Green Lantern",
            "Superman ++ Wonder Woman",
            "Superman -- Sinestro",
            "Superman -- Cheetah",
            "Superman -- Lex Luthor",
            "Green Lantern ++ Wonder Woman",
            "Green Lantern -- Sinestro",
            "Green Lantern -- Cheetah",
            "Green Lantern -- Lex Luthor",
            "Wonder Woman -- Sinestro",
            "Wonder Woman -- Cheetah",
            "Wonder Woman -- Lex Luthor",
            "Sinestro ++ Cheetah",
            "Sinestro ++ Lex Luthor",
            "Cheetah ++ Lex Luthor"
        ];

        let superman= "Superman";
        let green_lantern = "Green Lantern";
        let sinestro = "Sinestro";
        let cheetah = "Cheetah";
        let lex_luthor = "Lex Luthor";
        let wonder_woman = "Wonder Woman";

        let mut graph = HashMap::with_capacity(6);
        graph.insert(superman, vec![
            (green_lantern, true),
            (wonder_woman, true),
            (sinestro, false),
            (cheetah, false),
            (lex_luthor, false)
        ]);
        graph.insert(green_lantern, vec![
            (wonder_woman, true),
            (sinestro, false),
            (cheetah, false),
            (lex_luthor, false)
        ]);
        graph.insert(wonder_woman, vec![
            (sinestro, false),
            (cheetah, false),
            (lex_luthor, false)
        ]);
        graph.insert(sinestro, vec![
            (cheetah, true),
            (lex_luthor, true)
        ]);
        graph.insert(cheetah, vec![
            (lex_luthor, true)
        ]);
        graph.insert(lex_luthor, vec![]);

        let undirected_graph = make_graph_from_lines(lines);
        assert!(maps_are_equal(&graph, &undirected_graph.get_graph()));


    }

    #[test]
    fn test_convert_line_to_graph_input() {
        assert_eq!(convert_line_to_graph_input("Superman ++ Green Lantern"),
                   ("Superman", ("Green Lantern", true)));
        assert_eq!(convert_line_to_graph_input("Wonder Woman -- Lex Luthor"),
                   ("Wonder Woman", ("Lex Luthor", false))
        );
    }




}