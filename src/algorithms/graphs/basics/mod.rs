use std::collections::HashMap;

pub mod bfs;
pub mod dfs;

pub static mut LIST_GRAPH: Option<HashMap<usize, Vec<usize>>> = None;

pub fn fill_list_graph() {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // Crear nodos y aristas (incluyendo ciclos)
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![0, 3]); // Ciclo: 0 -> 1 -> 0
    graph.insert(2, vec![1, 3]); // Ciclo: 0 -> 2 -> 1 -> 0
    graph.insert(3, vec![2, 4]); // Ciclo: 1 -> 3 -> 2 -> 1
    graph.insert(4, vec![0, 3]); // Ciclos: 3 -> 4 -> 0 -> 1 -> 3

    unsafe {
        LIST_GRAPH = Some(graph);
    }
}
