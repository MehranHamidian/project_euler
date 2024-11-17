//link of problem -> https://projecteuler.net/problem=3

//problem description:
//the prime factor of 13195 is 5, 7, 13, and 29.
//what is the largest prime factor of the number 600851475143?

fn main() {
    //first find the factors of number
    //to do that, we should find the square root of number and round down to the closest whole number
    let number: f64 = 600851475143.0;
    let square_root: f64 = number.sqrt();
    let rounded_down: u64 = square_root.floor() as u64; //round down to closest whole number

    //define vector to store multiples
    let mut multiples: Vec<u64> = Vec::new();
    for i in 1..=rounded_down{
        if number as u64 % i == 0{
            multiples.push(i);
            multiples.push(number as u64 / i);
        }
    }

    //if any this multiples is prime find it by is_prime function and print it
    //defien a variable to store bigest prime factor
    let mut bigest_prime_factor = 0;

    for i in multiples{
        if is_prime(i) == true{
            if i > bigest_prime_factor{
                bigest_prime_factor = i;
            }
        }
    }

    println!("{}", bigest_prime_factor); //the out put will be 6857

}

//define a function to find prime number
fn is_prime(number: u64) -> bool {
    //first define a variable to store boolean variable
    //by default the variable is true
    let mut boolean: bool = true;
    for i in 2..number{
        if number % i == 0{
            boolean = false;
            break
        }
    }
    boolean
}
