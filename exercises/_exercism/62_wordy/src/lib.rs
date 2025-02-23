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
    type Error = OperationError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        // Err UnknownOp
        todo!()
    }
}

#[derive(Debug)]
pub enum OperationError {
    UnknownOp,
    InvalidCommandFormat,
}

impl Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for OperationError {}
struct Parser;

impl Parser {
    fn read(command: &str) -> Result<Vec<Operation>, Box<dyn Error>> {
        todo!()
    }
}
