extern crate clap;
extern crate regex;

use std::io;
use std::fs::File;
use std::io::{
    BufReader,
    prelude::*,
};

use regex::Regex;
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (idx, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        let line_number = idx + 1;
        match re.find(&line) {
            Some(_) => println!("{}: {}", line_number, line.trim_start()),
            None => (),
        }
    }
}

// fn process_lines_with_context<T: BufRead + Sized>(reader: T, re: Regex, context: usize) {
//     // Initialization
//     let mut tags: Vec<usize> = Vec::new();
//     let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

//     // First pass
//     for (i, line_) in reader.lines().enumerate() {
//         let line = line_.unwrap();
//         match re.find(&line) {
//             Some(_) => {
//                 tags.push(i);
//                 let v = Vec::with_capacity(2*context + 1);
//                 ctx.push(v);
//             },
//             None => (),
//         }
//     }

//     if tags.len() == 0 {
//         return;
//     }

//     // Second pass
//     for (i, line_) in reader.lines().enumerate() {
//         let line = line_.unwrap();
//         for (j, tag) in tags.iter().enumerate() {
//             let lowbound = tag.saturating_sub(context);
//             let highbound = tag + context;

//             if (i >= lowbound) && (i <= highbound) {
//                 let line_as_string = String::from(&line);
//                 let local_context = (i, line_as_string);
//                 ctx[j].push(local_context);
//             }
//         }
//     }
// }

fn main() {
    // Command line arguments
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for patterns")
        .author("Yannick Formaggio")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("context")
            .help("Number of lines around pattern match")
            .short("C")
            .takes_value(true)
            .required(false))
        .get_matches();
    
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    // let context = args.value_of("context").unwrap_or("2");
    
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }
    else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}