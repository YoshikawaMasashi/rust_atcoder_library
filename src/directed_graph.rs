use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct DirectedGraph<T> {
    nodes: HashSet<T>,
    edges: HashSet<(T, T)>,
    nexts: Option<HashMap<T, HashSet<T>>>,
}

impl<T: Clone + Copy + Eq + Hash> DirectedGraph<T> {
    pub fn new() -> Self {
        DirectedGraph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            nexts: None,
        }
    }

    pub fn get_nodes_num(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edges_num(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edges(&self) -> HashSet<(T, T)> {
        self.edges.clone()
    }

    pub fn set_node(&mut self, node: T) {
        self.nodes.insert(node);
    }

    pub fn set_edge(&mut self, edge: (T, T)) {
        self.edges.insert(edge);
    }

    pub fn calc_nexts(&mut self) {
        let mut nexts = HashMap::new();
        for &node in self.nodes.iter() {
            nexts.insert(node, HashSet::new());
        }
        for &edge in self.edges.iter() {
            nexts.get_mut(&edge.0).unwrap().insert(edge.1);
        }
        self.nexts = Some(nexts);
    }

    fn search_example_of_circuit(
        &self,
        current_path: &mut Vec<T>,
        current_nodes: &mut HashSet<T>,
    ) -> Option<Vec<T>> {
        if self.nexts.is_none() {
            panic!("not calculated `nexts`")
        }
        let current_node = current_path[current_path.len() - 1];
        for &next_node in self
            .nexts
            .as_ref()
            .unwrap()
            .get(&current_node)
            .unwrap()
            .iter()
        {
            if next_node == current_path[0] {
                return Some(current_path.clone());
            }
            if !current_nodes.contains(&next_node) {
                current_path.push(next_node);
                current_nodes.insert(next_node);

                if let Some(path) = self.search_example_of_circuit(current_path, current_nodes) {
                    return Some(path);
                }
                current_path.pop();
                current_nodes.remove(&next_node);
            }
        }
        None
    }

    pub fn get_example_of_circuit(&self, start_node: T) -> Option<Vec<T>> {
        let mut current_path = Vec::new();
        let mut current_nodes = HashSet::new();
        current_path.push(start_node);
        current_nodes.insert(start_node);
        self.search_example_of_circuit(&mut current_path, &mut current_nodes)
    }

    pub fn get_example_of_circuit_with_any_start(&self) -> Option<Vec<T>> {
        for &node in self.nodes.iter() {
            if let Some(path) = self.get_example_of_circuit(node) {
                return Some(path);
            }
        }
        None
    }

    pub fn get_induced_subgraph(&self, nodes: HashSet<T>) -> Self {
        let new_nodes: HashSet<T> = HashSet::from_iter(self.nodes.intersection(&nodes).cloned());
        let mut new_edges: HashSet<(T, T)> = HashSet::new();
        for &edge in self.edges.iter() {
            if new_nodes.contains(&edge.0) && new_nodes.contains(&edge.1) {
                new_edges.insert(edge);
            }
        }
        DirectedGraph {
            nodes: new_nodes,
            edges: new_edges,
            nexts: None,
        }
    }
}
