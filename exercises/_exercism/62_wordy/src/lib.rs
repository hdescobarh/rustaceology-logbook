use regex_lite::Regex;
use std::{error::Error, fmt::Display};

pub fn answer(command: &str) -> Option<i32> {
    let operations: Vec<Operation> = Parser::read(command).ok()?;
    operations
        .into_iter()
        .try_fold(Calculator::new(), |calc, op| calc.read(op))
        .and_then(|calc| calc.finish())
        .ok()
}

struct Calculator {
    num1: Option<i32>,
    num2: Option<i32>,
    operation: Option<Operation>,
    total: Option<i32>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            num1: None,
            num2: None,
            operation: None,
            total: None,
        }
    }

    fn read(self, operation: Operation) -> Result<Self, Box<dyn Error>> {
        //if first is not Init => InvalidCommandFormat
        todo!()
    }

    fn finish(self) -> Result<i32, Box<dyn Error>> {
        // if total None => Err InvalidCommandFormat
        todo!()
    }
}
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Operation {
    Init(i32),
    Add(i32),
    Sub(i32),
    Prod(i32),
    Div(i32),
    Pow(u32),
    Total,
}

impl TryFrom<(&str, &str)> for Operation {
    type Error = Box<dyn Error>;

    fn try_from(duple: (&str, &str)) -> Result<Self, Self::Error> {
        let operation = match (duple.0, duple.1) {
            ("What", num_str) => Self::Init(num_str.parse::<i32>()?),
            ("plus", num_str) => Self::Add(num_str.parse::<i32>()?),
            ("minus", num_str) => Self::Sub(num_str.parse::<i32>()?),
            ("multiplied by", num_str) => Self::Prod(num_str.parse::<i32>()?),
            ("divided by", num_str) => Self::Div(num_str.parse::<i32>()?),
            ("raised", num_str) => Self::Pow(num_str.parse::<u32>()?),
            ("?", "") => Self::Total,
            ("?", _) => Err(OperationError::InvalidCommandFormat)?,
            _ => Err(OperationError::UnknownOp)?,
        };
        Ok(operation)
    }
}

#[derive(Debug)]
pub enum OperationError {
    UnknownOp,
    InvalidCommandFormat,
}

impl Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OperationError::UnknownOp => writeln!(f, "Unknown operation string."),
            OperationError::InvalidCommandFormat => writeln!(f, "Invalid command syntax."),
        }
    }
}

impl Error for OperationError {}
struct Parser;

impl Parser {
    fn read(command: &str) -> Result<Vec<Operation>, Box<dyn Error>> {
        let pattern_beginning = "(What) is (-?\\d+)";
        let pattern_operations = "(?: (plus|minus|multiplied by|divided by) (-?\\d+))|\
                (?: (raised) to the (?:(1)st|(2)nd|(3)rd|([4-9])th|(\\d\\{2,\\}?)th) power)";
        let pattern_ending = "\\?";

        let re_command = Regex::new(&format!(
            "^{pattern_beginning}({pattern_operations})*{pattern_ending}$",
        ))?;
        if !re_command.is_match(command) {
            Err(OperationError::InvalidCommandFormat)?;
        }
        let re_args = Regex::new(&format!(
            "(?:{pattern_beginning})|{pattern_operations}|(?:({pattern_ending})())"
        ))?;
        re_args
            .captures_iter(command)
            .map(|caps| {
                let (_, [param, value]) = caps.extract();
                Operation::try_from((param, value))
            })
            .collect::<Result<Vec<Operation>, Box<dyn Error>>>()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parses_single_instructions() {
        let cases = [
            (
                "What is -365?",
                vec![Operation::Init(-365), Operation::Total],
            ),
            (
                "What is -65 plus 5?",
                vec![Operation::Init(-65), Operation::Add(5), Operation::Total],
            ),
            (
                "What is 5 minus 43?",
                vec![Operation::Init(5), Operation::Sub(43), Operation::Total],
            ),
            (
                "What is 31415 multiplied by 3?",
                vec![Operation::Init(31415), Operation::Prod(3), Operation::Total],
            ),
            (
                "What is 100000 divided by 2?",
                vec![Operation::Init(100000), Operation::Div(2), Operation::Total],
            ),
            (
                "What is 5 raised to the 1st power?",
                vec![Operation::Init(5), Operation::Pow(1), Operation::Total],
            ),
            (
                "What is 15 raised to the 2nd power?",
                vec![Operation::Init(15), Operation::Pow(2), Operation::Total],
            ),
            (
                "What is -15 raised to the 2nd power?",
                vec![Operation::Init(-15), Operation::Pow(2), Operation::Total],
            ),
            (
                "What is 445 raised to the 3rd power?",
                vec![Operation::Init(445), Operation::Pow(3), Operation::Total],
            ),
            (
                "What is 6563 raised to the 7th power?",
                vec![Operation::Init(6563), Operation::Pow(7), Operation::Total],
            ),
            (
                "What is 2 raised to the 19th power?",
                vec![Operation::Init(2), Operation::Pow(19), Operation::Total],
            ),
        ];
        for (input, expect) in cases {
            assert_eq!(Parser::read(input).unwrap(), expect)
        }
    }

    #[test]
    fn parses_combined_instructions() {
        let command = "What is 5 plus 6 minus 1 divided by 2 \
            multiplied by 3 raised to the 2nd power plus 5?";
        let parsed = Parser::read(command).unwrap();
        assert_eq!(
            vec![
                Operation::Init(5),
                Operation::Add(6),
                Operation::Sub(1),
                Operation::Div(2),
                Operation::Prod(3),
                Operation::Pow(2),
                Operation::Add(5),
                Operation::Total
            ],
            parsed
        )
    }
}
