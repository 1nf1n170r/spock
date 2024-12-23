mod logic;
pub use logic::*;
mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    use chemistry::element::elements;
    use types::result::Result;

    use super::*;

    #[test]
    fn spock() -> Result<()> {
        let molecule = elements::H + elements::H;
        let molecule = elements::H + elements::H;
        Ok(())
    }
}
