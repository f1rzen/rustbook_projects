use std::io;
use std::f32;

fn main() {
    println!("--Nth Fibonacci Finder---");
    println!("\nWrite your number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line!");
    let x: f32=input.trim().parse().expect("not an integer");
    calc(x);
}

fn calc(x:f32) {
    let fi: f32 = 1.618034;
    let result = ((fi.powf(x)-(1.0-fi).powf(x))/f32::sqrt(5.0)).round();
    println!("{result}");
}
