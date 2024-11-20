use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct XmlParser;

fn parse_xml(xml: &str) {
    let parse_result = XmlParser::parse(Rule::xml, xml);
    match parse_result {
        Ok(pairs) => {
            for pair in pairs {
                println!("{:?}", pair);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}