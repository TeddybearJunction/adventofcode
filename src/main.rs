use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() { 

    //Day 1 Input
    let lines: Vec<String> = get_input("./src/input1.txt");
    //Day 1, Q1
    fuel_tally(&lines);
    //Day 1, Q2
    fuel_fuel_tally(&lines)
    

}
///Return a Lines object for the file on the provided path
///
///@param filepath - string representing path to an input file
fn get_input(filepath : &str) -> Vec<String>
{
    let path = Path::new(&filepath);
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open file {}", Error::description(&why)),
        Ok(file) => file
    };
    let reader = BufReader::new(file);
    // |l| is the input vars for a {closure/lambda}
    let lines: Vec<String> = reader.lines().map(|l| {l.expect("Could not parse line")}).collect();
    lines
}
fn fuel_tally(data : &[String])
{
    let mut total: f64 = 0.0;
    let lines = data;
    for l in lines {
        //iterator returns a Result<T,E> where E is the error
        
        let mass = l.parse::<f64>().unwrap();
        let fuel: f64 = (mass/3.0).floor()-2.0;
        println!("Mass:{} \t-> Fuel:{}",&mass, &fuel);
        total += fuel;
        
    }
    println!("**Total is: {}**",total)
}

fn fuel_fuel_tally(data : &[String])
{
    let mut total: f64 = 0.0;
    let lines = data;
    for l in lines {
        //iterator returns a Result<T,E> where E is the error
        
        let mass = l.parse::<f64>().unwrap();
        let fuel: f64 = (mass/3.0).floor()-2.0;
        println!("Mass:{} \t-> Fuel:{}",&mass, &fuel);
        total += fuel;
        
    }
    println!("**Total is: {}**",total)
}
