use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() { 

    //Day 1 Input
    //let lines: Vec<String> = get_input("./src/input1.txt");
    //Day 1, Q1
    //fuel_tally(&lines);
    //Day 1, Q2
    //fuel_fuel_tally(&lines)
    
    //Day 2 Input
    let test_mode = false;
    let lines = if !test_mode {
        get_input("./src/input2.txt")[0].clone()
    } else {
        String::from("1,9,10,3,2,3,11,0,99,30,40,50")
    };


    //Day 2, Q1
    intcode_solver(&lines);

    

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
        let fuel: f64 = calc_fuel(mass);
        println!("Mass:{} \t-> Fuel:{}",&mass, &fuel);
        total += fuel;
        
    }
    println!("**Total is: {}**",total)
}
fn calc_fuel(mass : f64) -> f64
{
    let mut sub_total : f64 = 0.0;
    let fuel : f64 = (mass/3.0).floor()-2.0;
    println!("{}",fuel);
    sub_total += fuel;
    if fuel > 0.0
    {
        let fuel_for_fuel : f64 = calc_fuel(fuel);
        if fuel_for_fuel > 0.0
        {
            sub_total += fuel_for_fuel;
        }
    }
    sub_total
}

//Day 2 Part 1 - intcode computer
fn intcode_solver(data : &str)
{
    let mut intcode : Vec<i64> = data.split(',').map(|i| i.parse::<i64>().expect("Invalid integer?")).collect();
    let mut count : usize = 0;
    while intcode[count] != 99
    {

        let value : i64;
        match intcode[count]
        {
            //if Instruction = 1, add
            1 => value = intcode[intcode[(count + 1) as usize] as usize] + intcode[intcode[(count + 2) as usize] as usize],
            //if Instruction = 2, multiply
            2 => value = intcode[intcode[(count + 1) as usize] as usize] * intcode[intcode[(count + 2) as usize] as usize],

            _ => panic!("Bad input, release smoke")
        }
        //Set the address to store the result and store it
        let location = intcode[(count + 3) as usize];
        intcode[location as usize] = value;
        //Increment by 4 to get next opcode/Instruction
        count+=4;
    }
    println!("{:?}",intcode );
    

}