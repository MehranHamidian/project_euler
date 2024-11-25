//link of problem --> https://projecteuler.net/problem=5
//problem description:
//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any reminder.
//What is the smallest positive number that that is evently divisible by all of the numbers form 1 to 20.
use project_euler_lib::lcm;
fn main(){
    //use lcm on project_euler_lib to find lcm between two numbers
    //see project_euler_lib directory src/lib.rs line 43
    let mut part1 = lcm(1,2);
    for next in 3..=20{
        let mut part2 = lcm(part1, next);
        println!("{}", part2);
        part1 = part2;
        part2 = lcm(part1,next);
    }
    //lock at last print line 
}
