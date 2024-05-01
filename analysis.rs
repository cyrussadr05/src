use std::collections::{HashMap, HashSet};
use crate::graph::{Graph, bfs};

pub fn graph_metrics(graph: &Graph) -> (usize, usize, f64, f64, f64) {
    let mut all_distances = Vec::new();
    for node in graph.keys() {
        let distances = bfs(graph, node);
        for distance in distances.values().filter(|&&d| d > 0) {
            all_distances.push(*distance);
        }
    }

    let max_path_length = *all_distances.iter().max().unwrap_or(&0);
    let min_path_length = *all_distances.iter().min().unwrap_or(&0);
    let total_distance: usize = all_distances.iter().sum();
    let count = all_distances.len();
    let average_distance = total_distance as f64 / count as f64;
    let variance: f64 = all_distances.iter()
        .map(|&d| (d as f64 - average_distance).powi(2))
        .sum::<f64>() / count as f64;
    let standard_deviation = variance.sqrt();
    let median = if count > 0 {
        all_distances.sort_unstable();
        if count % 2 == 0 {
            let mid = count / 2;
            (all_distances[mid - 1] + all_distances[mid]) as f64 / 2.0
        } else {
            all_distances[count / 2] as f64
        }
    } else {
        0.0
    };

    (max_path_length, min_path_length, median, standard_deviation, average_distance)
}

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();
    for node in graph.keys() {
        let degree = graph.get(node).map_or(0, Vec::len);
        *distribution.entry(degree).or_insert(0) += 1;
    }
    distribution
}

pub fn degree_distribution_at_distance_2(graph: &Graph) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();
    for node in graph.keys() {
        let mut neighbors_at_distance_2 = HashSet::new();
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if let Some(second_degree_neighbors) = graph.get(neighbor) {
                    for second_neighbor in second_degree_neighbors {
                        if second_neighbor != node && !neighbors.contains(second_neighbor) {
                            neighbors_at_distance_2.insert(second_neighbor.clone());
                        }
                    }
                }
            }
        }
        let degree = neighbors_at_distance_2.len();
        *distribution.entry(degree).or_insert(0) += 1;
    }
    distribution
}

pub fn calculate_average_degrees(graph: &Graph) -> (f64, f64) {
    let mut total_degree = 0;
    let mut total_degree_2 = 0;
    let n = graph.len() as f64;

    for node in graph.keys() {
        let neighbors = graph.get(node).unwrap();
        total_degree += neighbors.len();

        let mut neighbors_at_distance_2 = HashSet::new();
        for neighbor in neighbors {
            if let Some(second_neighbors) = graph.get(neighbor) {
                for second_neighbor in second_neighbors {
                    if second_neighbor != node {
                        neighbors_at_distance_2.insert(second_neighbor);
                    }
                }
            }
        }
        total_degree_2 += neighbors_at_distance_2.len();
    }

    let average_degree = total_degree as f64 / n;
    let average_degree_2 = total_degree_2 as f64 / n;
    (average_degree, average_degree_2)
}
