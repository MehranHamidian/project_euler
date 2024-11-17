//link of problem -> https://projecteuler.net/problem=1

//title of problem:
//if we list all the natural number below 10 that art multiples of 3 or 5 we get 3, 5, 6 and 9
//the sum of this multiples is 23
//find the sum of all multiples of 3 or 5 below 1000

fn main() {
    //first of all define a variable to store sum of multiples
    let mut sum: u32 = 0;

    //now itrate over the number between 1-999 to find multiples of 3 or 5
    for i in 1..1000{
        if i % 5 == 0 || i % 3 == 0{ //this condition find multiples of 3 or 5
            sum += i; //if condition find a multiple, add it to the sum variable
        }
    }
    //and finaly println the sum variable
    println!("{}", sum);
}
