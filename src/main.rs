mod sum;
mod file;
mod user;
mod area;
mod cords;
mod guess;
mod people;
mod lockers;
mod vectors;
mod temperature;

use std::io;

fn main() {    
    let f = vec![   
                    "sum",
                    "file",
                    "user",
                    "guess",
                    "cords",
                    "area",
                    "lockers",
                    "vectors",
                    "people",
                    "temperature"
                ];

    println!("====================================================");
    println!("Enter the name of the operation you want to perform!");
    println!("Enter ls to list all the operations!");
    println!("Enter exit to terminate the program");
    println!("====================================================");
    
    let mut exec = true;

    
    while exec {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");
        
        if input.trim() == "ls" {
            println!("====================================================");
            println!("OPERATIONS:");
            let mut i = 0;
            while i != f.len() {
                println!("{}",f[i]);
                i += 1;
            }            
        }        
        else {            
            match input.trim() {
                "exit" => exec = false,
                "sum" => sum::sum(),
                "file" => file::file(),
                "user" => user::name(),
                "area" => area::area(),
                "cords" => cords::cords(),
                "guess" => guess::guessing_game(),
                "people" => people::people(),
                "lockers" => lockers::lockers(),
                "vectors" => vectors::vectors(),
                "temperature" => temperature::temperature(),
                _ => println!("Enter a valid input"),
            }
        }
        println!("====================================================");
    }    
    println!("Terminated");
}   