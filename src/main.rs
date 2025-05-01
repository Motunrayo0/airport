use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use csv::ReaderBuilder;
mod graph;
use graph::{build_airport, Graph};
use crate::graph::FlightStats;
mod path;
use path::shortest_path;
use std::io::{self, Write};


// Enum to represent the column values that can be a string or f64 
#[derive(Debug, Clone)]
enum columnval {
   One(String),
   Two(f64),
}
#[derive(Debug)]
// `DataFrame` struct to hold the data
// `label` is a vector of strings representing the column names
// `columns` is a vector of vectors of `columnval` representing the data
// `types` is a vector of u32 representing the types of the columns
struct DataFrame{
   label: Vec<String>,
   columns: Vec<Vec<columnval>>,
   types: Vec<u32>,


}


// copied from homework 8  starter code
// Custom error type for handling errors
#[derive(Debug)]
struct MyError(String);


impl fmt::Display for MyError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "There is an error: {}", self.0)
   }
}
impl Error for MyError {}
// copied form homework 8  starter code




impl DataFrame{
    // createing a new DataFrame
    // `new` function initializes a new `DataFrame` with empty vectors for `label`, `columns`, and `types`
   fn new() -> Self {
       DataFrame{
           label: Vec::new(),
           columns: Vec::new(),
           types: Vec::new(),


       }
   }
   /// read  data from a csv file and turn into a DataFrame
   /// 
   /// # Arguments
   /// * path - file path to the csv file
   /// types - A vector of type indentifier: 1 for string and 2 for f64
   /// 
   /// # returns
   /// * Result<(),Box<dyn Error>> - Ok if successful, Err if there is an error
   fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(),Box<dyn Error>>{
       self.types = types.clone();
       // create a csv reader with headers enable and comma delimiter 
       let mut read = csv::ReaderBuilder::new().delimiter(b',').has_headers(true).from_path(path)?;
       let headers = read.headers()?;
       self.label = headers.iter().map(|s| s.to_string()).collect();

        // process each row in the CSV file 
       for result in read.records(){
           let r = result?;
           let mut row: Vec<columnval> = vec![];
           for (i , elem) in r.iter().enumerate(){
               match types[i]{
                   1 => row.push(columnval::One(elem.to_string())),
                   2 => row.push(columnval::Two(elem.parse::<f64>()?)),
                   _=> return Err(Box::new(MyError("Unknow type".to_string()))),
               }
           }
           self.columns.push(row)
       }
       Ok(())
   }

   /// print the DataFrame
   /// 
   /// # Arguments
   /// * &self - a reference to the DataFrame
   /// # prints the DataFrame to the console
   fn print_dataframe(&self) {


       for label in &self.label {
           print!("{:<20}", label);
       }
       println!();




       for row in &self.columns {
           for cell in row {
               match cell {
                   columnval::One(s) => print!("{:<20}", s),
                   columnval::Two(n) => print!("{:<20}", format!("{:.1}", n)),
               }
           }
           println!();
       }
   }


 
  
}



fn main() -> Result<(), Box<dyn Error>> {
    // Load data and read CSV
    let mut df = DataFrame::new();
    let types = vec![1, 1, 2]; 
    df.read_csv("airport.csv", &types)?;
   
 
 
    // Build the graph
    let graph = build_airport(&df);
 
 
    // Get user input
    print!("Enter your starting airport code: ");
    io::stdout().flush()?;
    let mut start = String::new();
    io::stdin().read_line(&mut start)?;
    let start = start.trim();
 
 
    print!("Enter your destination airport code ");
    io::stdout().flush()?;
    let mut goal = String::new();
    io::stdin().read_line(&mut goal)?;
    let goal = goal.trim();
 
 
    // Find shortest path
    match shortest_path(&graph, start, goal) {
        Some((cost, path)) => {
            println!("Shortest path from {} to {} is:", start, goal);
            println!("Travel Time: {:.2} hours", cost/60.0);
            println!("Shortest path: {:?}", path);
        }
        None => println!("No path found from {} to {}.", start, goal),
    }
 
 
    Ok(())
 }
 