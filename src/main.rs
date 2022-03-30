// Author : SkwalExe
// Github : https://github.com/SkwalExe

use rand::Rng;
use std::io::{self, BufRead, Write};
use std::process;

const RED: &str = "\x1b[91m";
const YELLOW: &str = "\x1b[93m";
const MAGENTA: &str = "\x1b[95m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";

// parameters :
//
// - text : the text to print
// - speed : the delay in milliseconds between each chars
// - random_speed : if true, the speed is random between 20 and speed
// - newline : if true, a newline is added at the end of the text

fn typrint(text: &String, speed: &u32, random_speed: &bool, newline: &bool) {
    for c in text.chars() {
        // for each chars
        if *random_speed {
            // if random speed is true
            let random = rand::thread_rng().gen_range(if *speed <= 20 { 0 } else {20}..*speed); // generate a random number between 20 and speed
            std::thread::sleep(std::time::Duration::from_millis(random as u64));
        // sleep for the random number of milliseconds
        } else {
            std::thread::sleep(std::time::Duration::from_millis(*speed as u64));
            // sleep for the speed number of milliseconds
        }
        print!("{}", c); // print the char
        let _ = io::stdout().flush(); // flush the output
    }

    if *newline {
        println!(); // print a newline
    }
}

fn main() {
    let mut command = "print"; // command to execute (print, version, help)
    let mut newline = true; // if a newline should be printed after the message
    let mut text = String::new(); // the text to print
    let mut args: Vec<String> = std::env::args().collect(); // arguments vector
    let mut speed = 70; // the delay in milliseconds between each chars
    let mut random_speed = true; // if the speed is random between 20 and speed
    args.remove(0); // remove the program name

    while args.len() > 0 {
        match args[0].as_str() {
            "--" => {
                command = "print";
                args.remove(0);
                text = args.join(" "); // the text to print is set the everything after --
                break; // stop parsing arguments
            }
            "--version" | "-v" => {
                command = "version";
                args.remove(0);
            }
            "--nonewline" | "-n" => {
                newline = false;
                args.remove(0);
            }
            "--help" | "-h" => {
                command = "help";
                args.remove(0);
            }
            "--speed" | "-s" => {
                speed = match args[1].parse::<u32>() {
                    Ok(n) => {
                        if n == 0 {
                            println!(
                                "{}[ x ] : Speed cannot be 0 (just use normal print?){}",
                                RED, RESET
                            );
                            process::exit(1);
                        } else {
                            n
                        }
                    }
                    Err(_) => {
                        println!("{}Invalid speed value.{}", RED, RESET);
                        process::exit(1);
                    }
                };
                args.remove(0);
                args.remove(0);
            }
            "--no-random-speed" | "-r" => {
                random_speed = false;
                args.remove(0);
            }
            _ => {
                println!(
                    "{}Invalid argument: {}{} {} {}",
                    RED, WHITE, BG_RED, args[0], RESET
                );
                process::exit(1);
            }
        }
    }

    match command {
        "print" => {
            if text.is_empty() {
                // if no text is directly given
                let stdin = io::stdin();
                let lines = stdin.lock().lines();

                let mut line_count = 0;

                for line in lines {
                    // try to read from pipe
                    line_count += 1;
                    let line = line.expect("Could not read line from standard in");
                    typrint(&line, &speed, &random_speed, &newline);
                }

                if line_count == 0 {
                    // if there is no text being piped into the program
                    println!("{}No text specified{}", RED, RESET);
                    process::exit(1);
                }
            }

            typrint(&text, &speed, &random_speed, &newline);

            // just lsd-print the text
        }
        "version" => println!(
            "{}typrint, by Skwal => {}{}{}",
            MAGENTA,
            RED,
            env!("CARGO_PKG_VERSION"),
            RESET
        ),
        "help" => {
            println!("LSD print");
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            typrint(
                &String::from("Print text with a typing effect - Made with rust"),
                &70,
                &true,
                &true,
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Options : ");
            println!(
                "\t{}--version, -v: {}Prints the version of the program{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--help, -h: {}Prints this help message{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}-- [text]: {}Prints the text you give{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--nonewline, -n: {}Prints without a newline{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--speed, -s: {}Sets the delay between each chars in milliseconds{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--no-random-speed, -r: {}Disable random delay between each char from 20 to speed {}",
                MAGENTA, YELLOW, RESET 
            );

            println!(
                "{}\tprogram | typrint-print: {}Prints the output of program{}",
                MAGENTA, YELLOW, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
        }

        _ => {}
    }
}
