use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>; // Index là ID của Node

// Breadth-First Search (BFS)
pub fn bfs(graph: &Graph, start_node: usize, target: usize) -> bool {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start_node] = true;
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        if current == target {
            return true;
        }

        for &neighbor in &graph[current] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    false
}

fn main() {}
