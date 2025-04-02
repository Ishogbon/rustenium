use std::fs;
use parser::parse_bs_file;
mod parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string("raw/index.bs")?;
    parse_bs_file(&text);
    Ok(())
}
