mod sum;
mod user;
mod cords;
mod guess;
mod people;
mod lockers;
mod vectors;
mod temperature;

use std::io;

fn main() {    
    let f = vec!["sum", "user","guess",  "cords", "lockers", "vectors", "people", "temperature"];

    println!("Enter the name of the operation you want to perform!");
    println!("Enter ls to list all the operations!");
    println!("Enter exit to terminate the program");

    
    let mut exec = true;

    
    while exec {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        
        if input.trim() == "ls" {
            println!("------------");
            println!("OPERATIONS:");
            let mut i = 0;
            while i != 2{
                println!("{}",f[i]);
                i += 1;
            }
            println!("------------");
        }        
        else {            
            match input.trim() {
                "exit" => exec = false,
                "sum" => sum::sum(),
                "user" => user::name(),
                "cords" => cords::cords(),
                "guess" => guess::guessing_game(),
                "people" => people::people(),
                "lockers" => lockers::lockers(),
                "vectors" => vectors::vectors(),
                "temperature" => temperature::temperature(),
                _ => println!("Enter a valid input"),
            }
        }
    }    
    println!("Terminated");
}   