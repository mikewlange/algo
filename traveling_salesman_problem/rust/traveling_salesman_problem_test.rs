mod graph;
mod adjacency_matrix;
mod traveling_salesman_problem;

use graph::{Edge, Graph};
use adjacency_matrix::AdjacencyMatrix;
use traveling_salesman_problem::traveling_salesman_problem;

fn test(g: &Graph, optimal_distance: u32) {
    let tour = traveling_salesman_problem(g);
    assert!(is_tour(&tour, g));
    assert_eq!(AdjacencyMatrix::from_graph(&g).distance(&tour), optimal_distance);
}

fn is_tour(tour: &Vec<usize>, g: &Graph) -> bool {
    tour.len() == g.vertex_count && (0..g.vertex_count).all(|x| tour.contains(&x))
}

#[test]
fn test_zero_vertices() {
    test(&Graph {
        vertex_count: 0,
        edges: vec![],
    }, 0)
}

#[test]
fn test_one_vertex() {
    test(&Graph {
        vertex_count: 1,
        edges: vec![],
    }, 0)
}

#[test]
fn test_two_vertices_one_edge() {
    test(&Graph {
        vertex_count: 2,
        edges: vec![
            Edge { x: 0, y: 1, distance: 1 },
        ],
    }, 2);
}

#[test]
fn test_wikipedia_example() {
    // Example taken from
    // https://en.wikipedia.org/wiki/Travelling_salesman_problem#As_a_graph_problem
    // Furthest point insertion is unable to find the optimal solution; it needs 2-opt optimization.
    test(&Graph {
        vertex_count: 4,
        edges: vec![
            Edge { x: 0, y: 1, distance: 20 },
            Edge { x: 0, y: 2, distance: 42 },
            Edge { x: 0, y: 3, distance: 35 },
            Edge { x: 1, y: 2, distance: 30 },
            Edge { x: 1, y: 3, distance: 34 },
            Edge { x: 2, y: 3, distance: 12 },
        ],
    }, 97); // 0, 1, 2, 3, 0
}
