//link of problem --> https://projecteuler.net/problem=4
//problem description:
//A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit number is
//9009 = 91*99.
//find the largest palindrome made form the product of two 3-digit number

fn main() {
    //define variable to hold bigest number
    let mut bigest = 0;
    //first of all we need generate all 3-digit numbers
    for mut i in 100..=999 {
        i+=1; //to prevent duplicate number
        for j in 100..=999{
            //if number and revesr number be same the number is palindromic
            //and if result number biger than bigest, equal bigest to result
            if i * j == reverse_number(i*j) && i * j > bigest {
                bigest = i * j;
            }
        }
    }
    println!("{}", bigest);
}
//this function get a number and return revers the number
fn reverse_number(mut number: i32) ->i32 {
    //first of all define lenght of number
    let mut length: i8 = length(number.clone());
    let mut reverse_number: i32 = 0;
    loop{
        let remian: i32 = number % 10;
        reverse_number += remian * 10_i32.pow(length as u32 - 1);
        number/=10;
        length -=1;
        if number == 0{
            break
        }
    }
    reverse_number
}

//this function return length of number 
fn length (mut number: i32) -> i8{
    let mut counter: i8 = 0;
    while number >= 1{
        number /= 10;
        counter +=1;
    }
    counter
}
