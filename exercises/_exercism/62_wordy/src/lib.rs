use regex_lite::Regex;
use std::error::Error;

pub fn answer(command: &str) -> Option<i32> {
    Parser::read(command)
        .and_then(|operations| {
            operations
                .into_iter()
                .try_fold(Calculator::new(), |calculator, operation| {
                    calculator.read(operation)
                })
                .and_then(|calculator| calculator.finish())
        })
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
        todo!()
    }

    fn finish(self) -> Result<i32, Box<dyn Error>> {
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

struct Parser;

impl Parser {
    fn read(command: &str) -> Result<Vec<Operation>, Box<dyn Error>> {
        todo!()
    }
}
