use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Message(&'static str),
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(s) => write!(f, "{s}"),
        }
    }
}
