use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("[line {line}] Error {location}: {message}")]
pub struct InterpreterError {
    pub line: usize,
    pub location: String,
    pub message: String,
}
