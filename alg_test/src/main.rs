extern crate num;
use num::integer;

fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 2,
        _ => fib(n-2) + fib(n-1)
    }
}


fn fizzbuzz(n: i32) {
    for x in 1..(n+1) {
        let test_tuple = (integer::mod_floor(x, 3), integer::mod_floor(x, 5));
        let out = match test_tuple {
            (0, 0) => "fizzbuzz".to_string(),
            (0, _) => "fizz".to_string(),
            (_, 0) => "buzz".to_string(),
            _ => x.to_string()
            };
        println!("{}: {}", x, out);
    }
}

fn main() {
    println!("The {}th fibonacci number is {}", 40, fib(40));
    fizzbuzz(20);
}
