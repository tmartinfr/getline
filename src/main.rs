use std::env;
use std::io;
use std::io::prelude::*;
mod linespec;
use linespec::{LineSpec,ResultStrErr};

fn help(prog_name: &String, error: &str) {
    eprintln!("Error: {}\n\n\
              Usage: {} LINE_SPEC\n\
              Filter line numbers from standard input.\n\
              LINE_SPEC can be a single line number, or a start:end range.\n\
              Example: {} 3:12 </etc/services", error, prog_name, prog_name);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_args(&args) {
        Ok(line_spec) => getline(io::stdin(), line_spec),
        Err(e) => help(&args[0], e)
    };
}

fn getline(stdin: io::Stdin, line_spec: LineSpec) {
    let line_iter = stdin.lock().lines().enumerate().
                    filter(|enumeration| line_spec.line_in(enumeration.0 as u32 + 1));
    for enumeration in line_iter {
        println!("{}", enumeration.1.unwrap());
    }
}

fn parse_args(args: &Vec<String>) -> ResultStrErr<LineSpec> {
    if args.len() != 2 {
        return Err("Invalid number of arguments");
    }
    LineSpec::new(&args[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_spec_new() {
        assert!(LineSpec::new("3:5").is_ok());
        assert!(LineSpec::new("5:3").is_err());
        assert!(LineSpec::new("0:5").is_err());
    }

    #[test]
    fn test_line_spec_line_in() {
        let line_spec = LineSpec::new("3:5").unwrap();
        assert!(!line_spec.line_in(2));
        assert!(line_spec.line_in(3));
        assert!(line_spec.line_in(4));
        assert!(line_spec.line_in(5));
        assert!(!line_spec.line_in(6));
    }
}
