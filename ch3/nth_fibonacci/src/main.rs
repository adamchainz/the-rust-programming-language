use std::io;

fn main() {
    println!("Which fibonacci number would you like?");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Not a number");

    let root_5 = (5.0_f64).sqrt();
    let phi = (1.0 + root_5) / 2.0;

    let nth_fib = ((phi.powi(n) - ((-phi).powi(-n))) / root_5).round();

    println!("The answer is: {}", nth_fib);
}
