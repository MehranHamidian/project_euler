//link of problem --> https://projecteuler.net/problem=6
//problem description:
//The sum of squares of first ten natural numbers is 385
//The square of the sum of the first ten natural numbers is 3025
//Hence the diffrence between the sum of the squares of the frist ten natural numbers and the square of the sum is
//3025 - 385 = 2640
//Find diffrence between the sum of the square of all the first one hundred natural numbers and the square of the sum

fn main() {
    //first of all we need to define the sum of the squares
    let mut sum_of_squares: u64 = 0;
    for i in 1..=100{
        sum_of_squares += i * i;
    }
    //and now we need to define square of the sum
    let mut sum : u64 = 0;
    for i in 1..=100{
        sum +=i;
    }
    //and println the reuslt ----> so easy
    println!("{}", (sum * sum) - sum_of_squares);
}
