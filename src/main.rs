
extern crate getopts;
use getopts::{optopt, optflag, getopts, OptGroup, usage};

use std::io::{stdio, BufferedReader, LineBufferedWriter};
use std::os;
use std::io::{File, Open, ReadWrite};

fn leet_conversion(unleet: String) -> String {
    unleet.as_slice().chars()
        .map(|char| leet_char_conversion(char))
        .collect()
}

fn leet_char_conversion(unleet: char) -> char {
    match unleet {
        'a' | 'A' => '4',
        'b' | 'B' => '8',
        'e' | 'E' => '3',
        'l' | 'i' | 'L' | 'I' => '1',
        'o' | 'O' => '0',
        's' | 'S' => '5',
        't' | 'T' => '7',
        'z' | 'Z' => '2',
        chr => chr
    }
}

fn print_usage(opts: &[OptGroup]) {
    let generated_usage = 
        usage(format!("Usage: leet [OPTION] ... [FILENAME]").as_slice(),
            opts.as_slice());
    println!("{}", generated_usage);
}

fn leetification(input: &mut BufferedReader<File>, output: &mut Box<Writer>) {
    match input.read_line() {
        Ok(line) => {
            let _ = output.write_line(leet_conversion(line).as_slice()); 
            leetification(input, output) },
        Err(_) => {}
    }
}

fn main() {
    let args: Vec<String> = os::args();

    let opts = [
        optflag("", "std", "use stdio for input"),
        optopt("o", "output", "set output file name. Flushes to stdout if not given.", "NAME"),
        optflag("h", "help", "print this help menu")
    ];
    
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") || 
            (!matches.opt_present("std") && matches.free.is_empty()) {
        print_usage(opts);
        
    } else {
        
        let input_path = Path::new(matches.free[0].clone());
        let mut input = 
            match File::open(&input_path) {
                Ok(file) => BufferedReader::new(file),
                Err(ex)  => panic!(ex.to_string())
            };
        
        let mut output = match matches.opt_str("o") {
            Some(output_file) => {
                let output_path = Path::new(output_file);
                match File::open_mode(&output_path, Open, ReadWrite) {
                    Ok(output) => box LineBufferedWriter::new(output) as Box<Writer>,
                    Err(_) => box stdio::stdout() as Box<Writer>
                } },
            None => box stdio::stdout() as Box<Writer>
        };
        
        leetification(&mut input, &mut output);
    }
}