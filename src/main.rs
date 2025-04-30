use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use csv::ReaderBuilder;



#[derive(Debug, Clone)]
enum columnval {
   One(String),
   Two(f64),
}
#[derive(Debug)]
struct DataFrame{
   label: Vec<String>,
   columns: Vec<Vec<columnval>>,
   types: Vec<u32>,


}


// copied from homework 8  starter code
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
   fn new() -> Self {
       DataFrame{
           label: Vec::new(),
           columns: Vec::new(),
           types: Vec::new(),


       }
   }
   fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(),Box<dyn Error>>{
       self.types = types.clone();
       let mut read = csv::ReaderBuilder::new().delimiter(b',').has_headers(true).from_path(path)?;
       let headers = read.headers()?;
       self.label = headers.iter().map(|s| s.to_string()).collect();


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
