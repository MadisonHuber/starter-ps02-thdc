#![allow(unused)]
// TODO: Remove the line above before submission and fix warnings!

/**
 * thdc - Tar Heel Desk Calculator
 *
 * This program begins to implement the essence of the `dc` utility.
 *
 * Author: <author>
 * ONYEN: <onyen>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/*
 * Starter code follows. You should understand how this code works
 * before getting started. However, you should not modify this file
 * for this part of the problem set.
 */

// For this program we'll use a library to parse command line options.
// Generally, you should not hand parse CLI options and should rely
// on a library like `structopt` instead: https://docs.rs/structopt
extern crate structopt;
use structopt::StructOpt;

// thdc is a REPL that reads input from stdin
use std::io;

// We will split up project across multiple files/modules.
pub mod tokenizer;
use self::tokenizer::Tokenizer;

// Constants work as you'd expect given experience in other languages.
// They'll help us avoid magic numbers and strings.
const QUIT_STRING: &str = "q\n";
const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

// The following struct, with its annotations, describes the options/flags
// our program has built-in. These options will automatically be parsed
// for us from std::env::args().
#[derive(Debug, StructOpt)]
#[structopt(name = "thdc", about = "Tar Heel Desk Calculator")]
struct Options {
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    // Gather options from command-line arguments.
    let options = Options::from_args();

    loop {
        eval(&read(), &options);
    }
}

/**
 * Read input from the user. We'll handle the case of quitting
 * via the string "q" and exit the program from here. Additionally,
 * in the event of a read failure from standard input we'll exit
 * with a failure status code.
 */
fn read() -> String {
    match read_line() {
        Ok(line) => {
            if line == QUIT_STRING {
                // Exit the process with an Ok exit code.
                std::process::exit(EXIT_OK);
            } else {
                line
            }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
            std::process::exit(EXIT_ERR);
        }
    }
}

/**
 * Helper function to read a line of input from stdin.
 */
fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    // There are two interesting things happening on the line below:
    // 1. We are giving the read_line method a mutable reference to the input
    //    string. It is filling this String as a buffer via the reference
    //    rather than returning a String value. Notice the way the argument
    //    is being passed _tells_ us the method will change input's value.
    // 2. The trailing ? is a handy feature in Result/Error propagation.
    //    If read_line results in an Err, then it will automatically
    //    be returned in a propagated fashion to the caller of this fn.
    //    Otherwise, if it's Ok, it is automatically unwrapped. We are
    //    not using the returned value (number of bytes read) here since
    //    we only care about the input buffer being filled.
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

/**
 * Evaluate user input under the given options.
 */
fn eval(input: &str, options: &Options) {
    if options.debug {
        // For now, in debug mode, debug mode prints input as tokens.
        println!("== TOKENS ==");
        let mut tokens = Tokenizer::new(&input);
        while let Some(token) = tokens.next() {
            println!("{:?}", token);
        }
        print!("\n");
    } else {
        eprintln!("Only debug mode (-d) is implemented. Press q to quit.");
    }
}
