use xml_parser::{parse_xml, XmlValue};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use xml_parser::XmlValue;

    #[test]
    fn test_parse_valid_xml() -> Result<()> {
        let xml = r#"<note><to>Tove</to><from>Jani</from><heading>Reminder</heading><body>Don't forget me this weekend!</body></note>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "note");
            assert_eq!(children.len(), 4);
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_numeric_values() -> Result<()> {
        let xml = r#"<numbers><one>1</one><two>2</two><three>3</three></numbers>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "numbers");
            assert_eq!(children.len(), 3);
            assert!(matches!(children.get(0), Some(XmlValue::Element { name, children, .. }) if name == "one" && matches!(children.get(0), Some(XmlValue::Number(1)))));
            assert!(matches!(children.get(1), Some(XmlValue::Element { name, children, .. }) if name == "two" && matches!(children.get(0), Some(XmlValue::Number(2)))));
            assert!(matches!(children.get(2), Some(XmlValue::Element { name, children, .. }) if name == "three" && matches!(children.get(0), Some(XmlValue::Number(3)))));
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_mixed_values() -> Result<()> {
        let xml = r#"<data><name>John</name><age>30</age></data>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "data");
            assert_eq!(children.len(), 2);
            assert!(matches!(children.get(0), Some(XmlValue::Element { name, children, .. }) if name == "name" && matches!(children.get(0), Some(XmlValue::String(ref s)) if s == "John")));
            assert!(matches!(children.get(1), Some(XmlValue::Element { name, children, .. }) if name == "age" && matches!(children.get(0), Some(XmlValue::Number(30)))));
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_empty_xml() {
        let xml = r#"<empty></empty>"#;
        let result = parse_xml(xml);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "empty");
            assert!(children.is_empty());
        } else {
            panic!("Expected XmlValue::Element");
        }
    }

    #[test]
    fn test_parse_invalid_xml() {
        let xml = r#"<note><to>Tove</from></note>"#;
        let result = parse_xml(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_attributes() -> Result<()> {
        let xml = r#"<note id="1"><to>Tove</to><from>Jani</from></note>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, attributes, children } = &result[0] {
            assert_eq!(name, "note");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].0, "id");
            assert_eq!(attributes[0].1, "1");
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_nested_elements() -> Result<()> {
        let xml = r#"<root><parent><child>Value</child></parent></root>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "root");
            assert_eq!(children.len(), 1);
            if let XmlValue::Element { name, children, .. } = &children[0] {
                assert_eq!(name, "parent");
                assert_eq!(children.len(), 1);
                if let XmlValue::Element { name, children, .. } = &children[0] {
                    assert_eq!(name, "child");
                    assert_eq!(children.len(), 1);
                    assert!(matches!(children.get(0), Some(XmlValue::String(ref s)) if s == "Value"));
                } else {
                    panic!("Expected XmlValue::Element");
                }
            } else {
                panic!("Expected XmlValue::Element");
            }
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_with_whitespace() -> Result<()> {
        let xml = r#"<note> <to> Tove </to> <from> Jani </from> </note>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "note");
            assert_eq!(children.len(), 2);
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_with_special_characters() -> Result<()> {
        let xml = r#"<note><to>Tove &amp; Co.</to><from>Jani</from></note>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "note");
            assert_eq!(children.len(), 2);
            assert!(matches!(children.get(0), Some(XmlValue::Element { name, children, .. }) if name == "to" && matches!(children.get(0), Some(XmlValue::String(ref s)) if s == "Tove & Co.")));
            assert!(matches!(children.get(1), Some(XmlValue::Element { name, children, .. }) if name == "from" && matches!(children.get(0), Some(XmlValue::String(ref s)) if s == "Jani")));
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }

    #[test]
    fn test_parse_with_numeric_and_string() -> Result<()> {
        let xml = r#"<data><value>123</value><text>abc</text></data>"#;
        let result = parse_xml(xml)?;
        assert_eq!(result.len(), 1);
        if let XmlValue::Element { name, children, .. } = &result[0] {
            assert_eq!(name, "data");
            assert_eq!(children.len(), 2);
            assert!(matches!(children.get(0), Some(XmlValue::Element { name, children, .. }) if name == "value" && matches!(children.get(0), Some(XmlValue::Number(123)))));
            assert!(matches!(children.get(1), Some(XmlValue::Element { name, children, .. }) if name == "text" && matches!(children.get(0), Some(XmlValue::String(ref s)) if s == "abc")));
        } else {
            panic!("Expected XmlValue::Element");
        }
        Ok(())
    }
}