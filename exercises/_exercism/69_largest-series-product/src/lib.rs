#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?
        .windows(span)
        .map(|serie| serie.iter().map(|&d| d as u64).product::<u64>())
        .max()
        .ok_or(Error::SpanTooLong)
}
