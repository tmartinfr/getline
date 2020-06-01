pub type ResultStrErr<T> = std::result::Result<T, &'static str>;

#[derive(Debug)]
pub struct LineSpec {
    start: u32,
    end: u32,
}

impl LineSpec {
    pub fn new(spec: &str) -> ResultStrErr<Self> {
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

    pub fn line_in(&self, line_number: u32) -> bool {
        line_number >= self.start && line_number <= self.end
    }
}

fn parse_number(number: &str) -> ResultStrErr<u32> {
    number.parse::<u32>().or(Err("Invalid line spec"))
}
