use std::io;

fn main() {
    println!("Printing the nth number in the fibonacci sequence!");

    println!("Please input your n.");
    
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i128 = n.trim().parse().expect("Please put in a positive number!");

    if n < 1 {
        println!("Please put in a number greater than 0!");
        return
    }

    let mut i: i128 = 0;
    let mut j: i128 = 1;

    for _ in 0..n-1 {
        let k = j;
        j += i;
        i = k;
    }

    println!("The {}th number in the fibonacci sequence is: {}", n, i)
}
