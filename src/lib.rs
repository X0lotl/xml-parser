use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct XmlParser;

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error("parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

fn parse_xml(xml: &str) -> Result<(), XmlParseError> {
    let parse_result = XmlParser::parse(Rule::xml, xml)?;
    for pair in parse_result {
        println!("{:?}", pair);
    }
    Ok(())
}