use std::io::BufRead;

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
    /// assert!(LineSpec::from_spec("3:5").is_ok());
    /// assert!(LineSpec::from_spec("5:3").is_err());
    /// assert!(LineSpec::from_spec("0:5").is_err());
    ///
    /// let line_spec = LineSpec::from_spec("3:5").unwrap();
    /// assert!(!line_spec.line_in(2));
    /// assert!(line_spec.line_in(3));
    /// assert!(line_spec.line_in(4));
    /// assert!(line_spec.line_in(5));
    /// assert!(!line_spec.line_in(6));
    /// ```
    impl LineSpec {
        fn new(start: u32, end: u32) -> super::ResultStrErr<Self> {
            if start == 0 {
                return Err("Line number must start at 1");
            }
            if end < start {
                return Err("End line before start line");
            };
            Ok(Self { start: start, end: end})
        }

        pub fn from_spec(spec: &str) -> super::ResultStrErr<Self> {
            let fragments: Vec<&str> = spec.split(":").collect();
            let start = parse_number(&fragments[0])?;
            let end = match fragments.get(1) {
                Some(number) => parse_number(&number)?,
                None => start
            };
            Self::new(start, end)
        }

        pub fn line_in(&self, line_number: u32) -> bool {
            line_number >= self.start && line_number <= self.end
        }
    }

    fn parse_number(number: &str) -> super::ResultStrErr<u32> {
        number.parse::<u32>().or(Err("Invalid line spec"))
    }
}

pub use linespec::LineSpec;

/// Extract lines from buffer based on given specification
pub fn getline<T>(stream: T, line_spec: linespec::LineSpec) where T: BufRead {
    let line_iter = stream.lines().enumerate().
                    filter(|enumeration| line_spec.line_in(enumeration.0 as u32 + 1));
    for enumeration in line_iter {
        println!("{}", enumeration.1.unwrap());
    }
}
