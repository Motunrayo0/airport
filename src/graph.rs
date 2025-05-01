use std::collections::HashMap;
use crate::{columnval, DataFrame};
//stuct to represent the flight statistics 
//stores the times, count, average and standard deviation
#[derive(Debug, Clone)]
pub struct FlightStats {
   pub times: Vec<f64>,
   pub count: usize,
   pub average: f64,
   pub std_dev: f64,
}
// function to calculate the average and standard deviation
// 
// 3 Arguments 
// 1. `times`: a vector of f64 representing the times
// 2. `count`: a usize representing the count of the times
// 3. `sum`: a f64 representing the sum of the times
//
// return 
// - a tuple of f64 representing the average and standard deviation
fn calculate(times:&Vec<f64>) -> (f64, f64){
   let count = times.len() as f64;
   let sum:f64 = times.iter().sum();
   let avg = sum/count;
   let vari = times.iter().map(|t| (t-avg).powi(2)).sum::<f64>() / count;
   let std = vari.sqrt();
   (avg, std)


}
// the graph is hashmap of hashmap
// the outer hashmap is the origin airport
pub type Graph = HashMap<String, HashMap<String, FlightStats>>;
// builds a graph of flight  from the dataframe
// #Arguments 
// a refrecne to the data fram contaoning the flight data (orgin, destination and flight time )
// #returns 
// - A graph containing the calulated flight stats 
pub fn build_airport(df: &DataFrame) -> Graph{
   let mut map: Graph = HashMap::new(); 
   for row in &df.columns{
       if let (columnval::One(orig), columnval::One(dest), columnval::Two(time)) = (&row[0],&row[1], &row[2] ){
           let entry =  map.entry(orig.clone()).or_insert_with(HashMap::new).entry(dest.clone()).or_insert_with(||FlightStats{
               times: Vec::new(),
               count: 0,
               average: 0.0,
               std_dev: 0.0,


           });
           entry.times.push(*time);
           entry.count += 1;
       }
   }
   for(orig, edges) in map.iter_mut(){
       for(dest , stats ) in edges.iter_mut(){
           let(avg, s_t_d) = calculate(&stats.times);
           stats.average = avg;
           stats.std_dev = s_t_d;
       }
   }
   map
}


// tests for the build_airport function
// the test uses a mock dataframe to test the function
// the test checks if the graph is built correctly
mod tests {
    use super::*;
    use crate::columnval::{One, Two}; 
    use crate::DataFrame;             

    #[test]
    fn test_build_airport_runs() {
        let df = DataFrame {
            label: vec!["origin".to_string(), "destination".to_string(), "time".to_string()],
            columns: vec![
                vec![One("BOS".to_string()), One("LAX".to_string()), Two(6.0)],
            ],
            types: vec![1, 1, 2],
        };

        let graph = build_airport(&df);
        assert!(graph.contains_key("BOS")); 

        let dests = graph.get("BOS").unwrap();
        assert!(dests.contains_key("LAX")); 

        let stats = dests.get("LAX").unwrap();
        assert_eq!(stats.count, 1);         
        assert_eq!(stats.average, 6.0);     
        assert_eq!(stats.std_dev, 0.0);     
    }
}
