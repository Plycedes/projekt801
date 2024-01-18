mod sum;
mod user;
mod cords;
mod people;
mod lockers;
mod vectors;
mod temperature;

use std::io;

fn main() {
    ///Stores each command
    let f = vec!["sum", "user", "cords", "lockers", "vectors", "people", "temperature"];

    println!("Enter the name of the operation you want to perform!");
    println!("Enter ls to list all the operations!");
    println!("Enter exit to terminate the program");

    //Execution control variable
    let mut exec = true;

    ///Main response control block
    while exec {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");        

        ///Prints all the operations
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
            ///Matches command to respective action
            match input.trim() {
                "exit" => exec = false,
                "sum" => sum::sum(),
                "user" => user::name(),
                "cords" => cords::cords(),
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