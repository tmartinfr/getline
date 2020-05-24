use std::env;
use std::io;
use std::io::prelude::*;

type ResultStrErr<T> = std::result::Result<T, &'static str>;

#[derive(Debug)]
pub struct LineSpec {
    start: u32,
    end: u32,
}

impl LineSpec {
    fn new(spec: &str) -> ResultStrErr<Self> {
        let fragments: Vec<&str> = spec.split(":").collect();
        let start = parse_number(&fragments[0])?;
        let end = match fragments.get(1) {
            Some(number) => parse_number(&number)?,
            None => start
        };
        Self::init(start, end)
    }

    fn init(start: u32, end: u32) -> ResultStrErr<Self> {
        if start == 0 {
            return Err("Line number must start at 1");
        }
        if end < start {
            return Err("End line before start line");
        };
        Ok(Self { start: start, end: end})
    }

    fn line_in(&self, line_number: u32) -> bool {
        line_number >= self.start && line_number <= self.end
    }
}

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

fn parse_number(number: &str) -> ResultStrErr<u32> {
    number.parse::<u32>().or(Err("Invalid line spec"))
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
