use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum NoodlezError {
    #[error("Whut?!")]
    #[diagnostic(help = "Ask the maintainers to implement better error handling 😮‍💨")]
    Universal,
}
