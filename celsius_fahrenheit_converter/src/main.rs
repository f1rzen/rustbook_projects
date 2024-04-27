use std::io;

fn main() {
    println!("\n---CELSIUS-FAHRENHEIT CONVERTER---");
    println!("Select Unit:\n1: Celsius \n2: Fahrenheit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i8  = input.trim().parse().expect("Input not an integer");
    if x==1 {
        println!("You selected Celsius");
        c_to_f();
    }else if x==2 {
        println!("You selected Fahrenheit");
        f_to_c();
    }else {
        println!("Wrong selection!");

    }
}

fn c_to_f() {
    println!("Write your celsius degree: ");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line!");
    let c_float: f64 = c.trim().parse().expect("Input not a float");
    let c_to_f = (c_float*9.0/5.0)+32.0;
    println!("{c_float} celsius is {c_to_f} Fahrenheit");
}

fn f_to_c() {
    println!("Write your Fahrenheit degree:");
    let mut f=String::new();
    io::stdin().read_line(&mut f).expect("Failed to read line");
    let f_float: f64 = f.trim().parse().expect("Input not a float");
    let f_to_c = (f_float - 32.0)*5.0/9.0;
    println!("{f_float} fahrenheit is {f_to_c} Celsius");
}
