extern crate clap;

use std::io::{stdin, stdout, Write};
use clap::{App, SubCommand};

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Does some basic math things.")
        .author("John B. <johnboydiv@gmail.com>")
        .subcommand(SubCommand::with_name("polydiv").about(
            "Perform polynomial division",
        ))
        .get_matches();

    // Determine which subcommand was chosen
    match cli_args.subcommand_name() {
        Some("polydiv") => polynomial_division(),
        _ => {
            println!("Error: No subcommand provided.");
            println!("Run 'rusty-math -h' for a list of available commands.");
        }
    }
}

fn polynomial_division() {
    // Read in numerator
    let n = get_polynomial_coefficients("Numerator coefficients: ");

    // Read in denominator
    let d = get_polynomial_coefficients("Denominator coefficients: ");

    // Divide polynomials
    let (q, r) = divide_polynomials(n, &d);

    // Print result
    print!("Quotient = ");
    print_polynomial(&q);
    print!("Remainder = ");
    print_polynomial(&r);
}

fn get_polynomial_coefficients(msg: &str) -> Vec<f32> {
    // Input string
    let mut s = String::new();

    // Prompt for input
    print!("{}", msg);
    stdout().flush().unwrap();

    // Read input
    stdin().read_line(&mut s).expect(
        "Please enter the coefficients of a polynomial!",
    );

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    if s.len() <= 0 {
        println!("Error: Please provide coefficients for the denominator polynomial!");
        std::process::exit(1);
    }

    // Parse the input string into a reverse ordered vector of coefficients
    let mut c: Vec<f32> = s.split_whitespace()
        .rev()
        .map(|s| {
            s.parse().unwrap_or_else(|_e| {
                println!("Error: Coefficients must be integer values.");
                std::process::exit(1);
            })
        })
        .collect();

    // Remove trailing zeros
    for i in (0..c.len()).rev() {
        if c[i] == 0.0 {
            c.pop();
        } else {
            break;
        }
    }

    // Return vector of coefficients
    return c;
}

fn divide_polynomials(mut n: Vec<f32>, d: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    // Handle case where denominator is higher order than the numerator
    if d.len() > n.len() {
        return (vec![0.0], n);
    }

    // Find the difference in order between the two polynomials
    let ord_diff = n.len() - d.len();

    // Create vector large enough to hold the quotient
    let mut q = vec![0.0; ord_diff + 1];

    // Loop over quotient
    for i in (0..q.len()).rev() {
        let n_idx = n.len() - 1 - ord_diff + i;
        q[i] = n[n_idx] / d[d.len() - 1];

        // Loop over denominator
        for j in 0..d.len() {
            n[n_idx - j] -= q[i] * d[d.len() - j - 1];
        }
    }

    // Remove trailing zeros from remainder
    for i in (1..n.len()).rev() {
        if n[i] == 0.0 {
            n.pop();
        } else {
            break;
        }
    }

    return (q, n);
}

fn print_polynomial(c: &Vec<f32>) {
    // Print polynomial from coefficients
    if c.len() > 2 {
        for i in (2..c.len()).rev() {
            print!("{}x^{} + ", c[i], i);
        }
    }
    if c.len() > 1 {
        print!("{}x + ", c[1]);
    }
    println!("{}", c[0]);
}
