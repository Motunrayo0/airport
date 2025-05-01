use std::collections::{BinaryHeap, HashMap};
use crate::graph::FlightStats;
use std::cmp::Ordering;


// Struct to represent a state in the priority queue (BinaryHeap)
#[derive(Clone, PartialEq)]
struct State {
   cost: f64,           // Total cost (sum of averages so far) to reach this node
   position: String,    // Current airport (node)
}
// Implement `Eq` so we can use State in a BinaryHeap
impl Eq for State {}


// Implement `Ord` to reverse the comparison logic
// Rust's BinaryHeap is a max-heap by default, but we want a min-heap (lowest cost first)
// So we reverse the ordering here
impl Ord for State {
   fn cmp(&self, other: &Self) -> Ordering {
       other
           .cost
           .partial_cmp(&self.cost)
           .unwrap_or(Ordering::Equal) // In case of NaN, treat as equal
   }
}


impl PartialOrd for State {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other))
   }
}


/// Find the shortest path from `start` airport to `goal` airport
/// using Dijkstra's algorithm.
///
/// # Arguments
/// - `graph`: adjacency list, where keys are start airports, and values are maps of destination airports to FlightStats
/// - `start`: the starting airport
/// - `goal`: the target airport
///
/// # Returns
/// - `Some((total_cost, path))` if a path exists
/// - `None` if no path exists
pub fn shortest_path(
   graph: &HashMap<String, HashMap<String, FlightStats>>,
   start: &str,
   goal: &str,
) -> Option<(f64, Vec<String>)> {
   // Priority queue to always process the node with the least total cost so far
   let mut heap = BinaryHeap::new();


   // Map to store the shortest known distance to each node
   let mut distances: HashMap<String, f64> = HashMap::new();


   // Map to store the immediate predecessor of each node (for reconstructing the path)
   let mut predecessors: HashMap<String, String> = HashMap::new();


   // Start with the starting airport, with a total cost of 0
   heap.push(State {
       cost: 0.0,
       position: start.to_string(),
   });
   distances.insert(start.to_string(), 0.0);


   // Main loop: process airports in order of increasing cost
   while let Some(State { cost, position }) = heap.pop() {
       // If we've reached the destination, reconstruct the path
       if position == goal {
           let mut path = Vec::new();
           let mut current = goal.to_string();
           // Walk backward from goal to start using the predecessors map
           while let Some(prev) = predecessors.get(&current) {
               path.push(current.clone());
               current = prev.clone();
           }
           path.push(start.to_string());
           path.reverse(); // Because we built it backward
           return Some((cost, path));
       }


       // Otherwise, check all neighbors of the current airport
       if let Some(neighbors) = graph.get(&position) {
           for (neighbor, stats) in neighbors {
               let next_cost = cost + stats.average; // New total cost to reach neighbor


               // If this path to neighbor is better (lower cost) than any known path so far
               if next_cost < *distances.get(neighbor).unwrap_or(&f64::INFINITY) {
                   distances.insert(neighbor.clone(), next_cost); // Update shortest distance
                   predecessors.insert(neighbor.clone(), position.clone()); // Update predecessor
                   heap.push(State {
                       cost: next_cost,
                       position: neighbor.clone(),
                   });
               }
           }
       }
   }


   // If we exit the loop without finding the goal, there is no path
   None
}

#[cfg(test)]
mod tests {
   use super::*;
   use std::collections::HashMap;

   #[test]
   fn test_shortest_path() {
       let mut graph: HashMap<String, HashMap<String, FlightStats>> = HashMap::new();
       let mut edges: HashMap<String, FlightStats> = HashMap::new();
       edges.insert(
           "BOS".to_string(),
           FlightStats {
               times: vec![1.5, 2.0, 1.7],
               count: 3,
               average: 1.73,
               std_dev: 0.23,
           },
       );
       graph.insert("JFK".to_string(), edges);
       let result = shortest_path(&graph, "JFK", "BOS");
       assert!(result.is_some());
       let (cost, path) = result.unwrap();
       assert_eq!(cost, 1.73);
       assert_eq!(path, vec!["JFK", "BOS"]);
   }
}
