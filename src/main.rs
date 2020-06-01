use std::env;
use std::io;
use ::getline::linespec::LineSpec;
use ::getline::getline;
use ::getline::ResultStrErr;

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
        Ok(line_spec) => getline(io::stdin().lock(), line_spec),
        Err(e) => help(&args[0], e)
    };
}

fn parse_args(args: &Vec<String>) -> ResultStrErr<LineSpec> {
    if args.len() != 2 {
        return Err("Invalid number of arguments");
    }
    LineSpec::from_spec(&args[1])
}
