use xml_parser::{parse_xml, XmlValue};

fn main() {
    let xml = r#"<note id="10"><to>Tove</to><from>Jani</from><heading>Reminder</heading><body>Don't forget me this weekend!</body></note>"#;
    match parse_xml(xml) {
        Ok(values) => {
            for value in values {
                println!("{:?}", value);
            }
        }
        Err(e) => {
            eprintln!("Error parsing XML: {}", e);
        }
    }
}