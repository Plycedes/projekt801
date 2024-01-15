mod user;
mod sum;
use std::io;

fn main() {
    let f: [&str; 2] = ["sum", "user"];

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
        // else if input.trim() == "exit" {
        //     exec = false;
        // }
        // else if input.trim() == "sum"{
        //     sum::sum();
        // }
        // else if input.trim() == "user"{
        //     user::name();
        // }
        // else {
        //     println!("Enter a valid input!!");
        // }
        else {
            match input.trim() {
                "exit" => exec = false,
                "sum" => sum::sum(),
                "user" => user::name(),
                _ => println!("Enter a valid input"),
            }
        }
    }    
    println!("Terminated");
}   