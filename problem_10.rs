fn main() {
    let mut sum: u64 = 0;
    for i in 1..2_000_000_u64{
        if trail_division(i as i32) {
            sum += i;
        }
    }
    println!("{}", sum);
}

fn trail_division(number: u64) -> bool {
    let mut boolean = true;
    if number < 2 {
        boolean = false;
    }else {
        let square_root: u64 = (number as f64).sqrt().floor() as u64;
        for i in 2..=square_root{
            if number % i == 0{
                boolean = false;
            }
        }
    }
    boolean
}
