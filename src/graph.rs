use std::collections::HashMap;
use crate::{columnval, DataFrame};


//stuct to represent the flight statistics 
//stor the times, count, average and standard deviation
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
/* this is how the hash map looks like {
   "JFK": {
       "BOS": FlightStats {
           times: vec![1.5, 2.0, 1.7],
           count: 3,
           average: 1.73, 
           std_dev: 0.23, 
       },
       "LGR": FlightStats {
           times: vec![3.0],
           count: 1,
           average: 3.0,  
           std_dev: 0.0,  
       }
}
       */
