use std::io;

fn fermat_proof(a: u32, b: u32, c: u32, n: u32) -> bool {
    return !(a.pow(n) + b.pow(n) == c.pow(n) && n > 2);
}

fn main() {
    let mut input_string = String::new();
    println!("Enter values for a, b, c, and n (n > 2) separated by spaces:");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    let inputs: Vec<u32> = input_string
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    let result = fermat_proof(inputs[0], inputs[1], inputs[2], inputs[3]);
    if result {
        println!("Fermat was right!");
    } else {
        println!("Fermat was wrong!");
    }
}
