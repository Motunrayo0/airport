use std::collections::HashMap;
use crate::{columnval, DataFrame};
#[derive(Debug, Clone)]
pub struct FlightStats {
   pub times: Vec<f64>,
   pub count: usize,
   pub average: f64,
   pub std_dev: f64,
}
fn calculate(times:&Vec<f64>) -> (f64, f64){
   let count = times.len() as f64;
   let sum:f64 = times.iter().sum();
   let avg = sum/count;
   let vari = times.iter().map(|t| (t-avg).powi(2)).sum::<f64>() / count;
   let std = vari.sqrt();
   (avg, std)


}
pub type Graph = HashMap<String, HashMap<String, FlightStats>>;


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
