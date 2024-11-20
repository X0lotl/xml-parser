use anyhow::{Result, Error};
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
use std::num::ParseIntError;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct XmlParser;

#[derive(Error, Debug)]
pub enum XmlParseError {
    #[error("parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("integer parse error: {0}")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, PartialEq)]
pub enum XmlValue {
    Element {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<XmlValue>,
    },
    String(String),
    Number(i32),
}

pub fn parse_xml(xml: &str) -> Result<Vec<XmlValue>, XmlParseError> {
    let parse_result = XmlParser::parse(Rule::xml, xml)?;
    let mut values = Vec::new();
    for pair in parse_result {
        match pair.as_rule() {
            Rule::xml => {
                for inner_pair in pair.into_inner() {
                    if inner_pair.as_rule() == Rule::element {
                        let element = parse_element(inner_pair)?;
                        values.push(element);
                    }
                }
            }
            _ => {
                return Err(XmlParseError::PestError(pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::CustomError {
                        message: "expected xml rule".to_string(),
                    },
                    pair.as_span(),
                )));
            }
        }
    }
    Ok(values)
}

fn parse_element(pair: pest::iterators::Pair<Rule>) -> Result<XmlValue, XmlParseError> {
    let mut name = String::new();
    let mut attributes = Vec::new();
    let mut children = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::start_tag => {
                for tag_inner_pair in inner_pair.into_inner() {
                    if tag_inner_pair.as_rule() == Rule::name {
                        name = tag_inner_pair.as_str().to_string();
                    }

                    if tag_inner_pair.as_rule() == Rule::attribute {
                        let mut attr_inner = tag_inner_pair.into_inner();
                        let key = attr_inner.next().unwrap().as_str().to_string();
                        let value = attr_inner.next().unwrap().as_str().to_string();
                        attributes.push((key, value));
                    }
                }
            }
            Rule::attribute => {
                let mut attr_inner = inner_pair.into_inner();
                let key = attr_inner.next().unwrap().as_str().to_string();
                let value = attr_inner.next().unwrap().as_str().to_string();
                attributes.push((key, value));
            }
            Rule::element => {
                let child = parse_element(inner_pair)?;
                children.push(child);
            }
            Rule::text => {
                let value = inner_pair.as_str().trim();
                if let Ok(num) = value.parse::<i32>() {
                    children.push(XmlValue::Number(num));
                } else {
                    children.push(XmlValue::String(value.to_string()));
                }
            }
            _ => {}
        }
    }

    Ok(XmlValue::Element {
        name,
        attributes,
        children,
    })
}