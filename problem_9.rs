//link of problem --> https://projecteuler.net/problem=9
//problem description:
//A Pythagorean triplet is a set of three natural numbers, a < b < c for which,
// a^2 + b^2 = c^2
// for example 3^2 + 4^2 = 9 + 16 = 25 = 5^2
//There exist exctly one Pythagorean number triplet for which a+b+c = 1000
//Find product abc
fn main(){
    let mut a: f64 = 1.0;
    while a < 333.0{
        let mut b: f64 = 1.0;
        while b < 499.0{
            if is_pythagorean_triplet(a, b, pythagorean(a, b)) && pythagorean(a,b) % 1.0 == 0.0 && a + b + pythagorean(a, b) == 1000.0{
                println!("{}", a * b * pythagorean(a, b));
            }
            b+=1.0;
        }
        a+=1.0;
    }
}
fn is_pythagorean_triplet (a: f64, b: f64, c: f64) -> bool{
    if a < b && b < c && (a * a) + (b * b) == (c * c) {
        return true;
    }else{
        return false;
    }
}
fn pythagorean (a: f64, b: f64) -> f64 {
    let c: f64 = a.powf(2.0) + b.powf(2.0);
    c.sqrt()
}
