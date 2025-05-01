use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use ordered_float::OrderedFloat;
use crate::graph::{Graph, FlightStats};

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

    // No path found
    None
}

