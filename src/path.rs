use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use ordered_float::OrderedFloat;
use crate::graph::{Graph, FlightStats};


/// Finds the fastest route between two airports using Dijkstra's algorithm.
/// 
/// # Arguments
/// - `graph`: A reference to a graph representing airport connections and flight statistics.
/// - `start`: The starting airport code.
/// - `goal`: The destination airport code.
/// 
/// # Returns
/// - `Some((total_time, path))`: A tuple containing the total average flight time and a vector of airport codes representing the path, if a route is found.
/// - `None`: If there is no valid route between the start and goal.
pub fn fastest_route(graph: &Graph, start: &str, goal: &str) -> Option<(f64, Vec<String>)> {
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<String, f64> = HashMap::new();
    let mut prev: HashMap<String, String> = HashMap::new();

    heap.push(Reverse((OrderedFloat(0.0), start.to_string())));
    distances.insert(start.to_string(), 0.0);
    while let Some(Reverse((OrderedFloat(current_dist), current_airport))) = heap.pop() {
        
        if current_airport == goal {
            let mut path = Vec::new();
            let mut node = goal.to_string();
            while let Some(prev_node) = prev.get(&node) {
                path.push(node.clone());
                node = prev_node.clone();
            }
            path.push(start.to_string());
            path.reverse();
            return Some((current_dist, path));
        }

        if let Some(neighbors) = graph.get(&current_airport) {
            for (neighbor, stats) in neighbors {
                let time = stats.average; // Use average flight time as weight
                let new_dist = current_dist + time;

                if new_dist < *distances.get(neighbor).unwrap_or(&f64::INFINITY) {
                    distances.insert(neighbor.clone(), new_dist);
                    heap.push(Reverse((OrderedFloat(new_dist), neighbor.clone())));
                    prev.insert(neighbor.clone(), current_airport.clone());
                }
            }
        }
    }

    
    None
}

