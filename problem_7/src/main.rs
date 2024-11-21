//link of problem --> https://projecteuler.net/problem=7
//problem description:
//By listing the first six prime number:2, 3, 5, 7, 11 and 13, we can see that the 6th prime number is 13.
//What is the 10_001st prime number?
use std::io;
fn main() {
    //to solve this number we need loop over the positive number to find 10_001st prime number
    let mut input:String = String::new(); //define a variable to store user input

    println!("");
    println!("**************** The xth_prime_number ****************");
    println!("");
    println!("plese input number");
    io::stdin().read_line(&mut input).expect("Failed to read line"); //read user input
    let user_input_number: u32 = input.trim().parse().expect("Not a Valid number"); //convert user input to u32 type
    println!("The {}th prime number is {}",user_input_number, xth_prime_number(user_input_number)); //print resault
}

//this function define xth prime number
fn xth_prime_number(user_input: u32) -> u32 {

    let mut number: u32 =2; //cause prime number started from 2
    let mut counter: u32 = 0; //define a counter to counting prime number
    loop{
        if is_prime(number){
            counter +=1;
        }
        if counter == user_input{
            break number
        }
        number += 1;
    }
}

fn is_prime(number: u32) -> bool{
    let mut is_prime: bool = true;
    //define square root of number
    let square_root: f64 = (number as f64).sqrt().floor();
    for i in 2..=(square_root as u32){
        if number % i == 0 {
            is_prime = false;
            break
        }
    }
    is_prime
}
