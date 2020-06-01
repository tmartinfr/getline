use std::io;
use std::io::prelude::*;

pub type ResultStrErr<T> = std::result::Result<T, &'static str>;

pub mod linespec {
    #[derive(Debug)]
    pub struct LineSpec {
        start: u32,
        end: u32,
    }

    /// Represents a line range specification
    ///
    /// # Examples
    /// ```
    /// # use ::getline::linespec::LineSpec;
    /// assert!(LineSpec::new("3:5").is_ok());
    /// assert!(LineSpec::new("5:3").is_err());
    /// assert!(LineSpec::new("0:5").is_err());
    ///
    /// let line_spec = LineSpec::new("3:5").unwrap();
    /// assert!(!line_spec.line_in(2));
    /// assert!(line_spec.line_in(3));
    /// assert!(line_spec.line_in(4));
    /// assert!(line_spec.line_in(5));
    /// assert!(!line_spec.line_in(6));
    /// ```
    impl LineSpec {
        pub fn new(spec: &str) -> super::ResultStrErr<Self> {
            let fragments: Vec<&str> = spec.split(":").collect();
            let start = parse_number(&fragments[0])?;
            let end = match fragments.get(1) {
                Some(number) => parse_number(&number)?,
                None => start
            };
            Self::init(start, end)
        }

        fn init(start: u32, end: u32) -> super::ResultStrErr<Self> {
            if start == 0 {
                return Err("Line number must start at 1");
            }
            if end < start {
                return Err("End line before start line");
            };
            Ok(Self { start: start, end: end})
        }

        pub fn line_in(&self, line_number: u32) -> bool {
            line_number >= self.start && line_number <= self.end
        }
    }

    fn parse_number(number: &str) -> super::ResultStrErr<u32> {
        number.parse::<u32>().or(Err("Invalid line spec"))
    }
}

pub fn getline(stdin: io::Stdin, line_spec: linespec::LineSpec) {
    let line_iter = stdin.lock().lines().enumerate().
                    filter(|enumeration| line_spec.line_in(enumeration.0 as u32 + 1));
    for enumeration in line_iter {
        println!("{}", enumeration.1.unwrap());
    }
}
