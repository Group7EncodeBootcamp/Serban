fn main() {
    fizz_buzz();
}

fn fizz_buzz() {
    let count_max = 301;

    for x in 1..=count_max {
        match (x % 3 == 0, x % 5 == 0) {
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (true, true) => println!("fizzbuzz"),
            _ => (),
        }
    }
}