# XML Parser

A command-line interface for parsing XML files. This tool allows you to parse XML files and display their content in a structured format.

## Technical description

The XML Parser processes XML files by breaking them down into their constituent elements, attributes, and text content. The parsing process involves the following steps:

1. **Lexical Analysis**: The parser reads the XML file and identifies the different tokens such as tags, attributes, and text.
2. **Syntax Analysis**: The parser checks the structure of the XML file against the defined grammar rules to ensure it is well-formed.
3. **Semantic Analysis**: The parser interprets the meaning of the elements and attributes, converting them into a structured format.

The results of the parsing are used to create a tree-like structure representing the XML document. This structure can be used for various purposes, such as data extraction, transformation, and validation.

## Features
- Parse XML files
- Display help information
- Display credits information

## Features
- Parse XML files
- Display help information
- Display credits information

## Usage

### Build the project
```sh
cargo build
```

### Run the project
```sh
cargo run -- parse --file <path_to_xml_file>
```

### Display help information
```sh
cargo run -- help
```

### Display credits information
```sh
cargo run -- credits
```
### Run tests
```sh
cargo test
```

## Grammar rules

### Define whitespace characters
```ebnf
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
```

### Define the root XML rule
```ebnf
xml = { SOI ~ element ~ EOI }
```

### Define an XML element, which consists of a start tag, zero or more nested elements or text, and an end tag
```ebnf
element = { start_tag ~ (element | text)* ~ end_tag }
```

### Define a start tag, which includes a name and optional attributes
```ebnf
start_tag = { "<" ~ name ~ (WHITESPACE ~ attribute)* ~ WHITESPACE? ~ ">" }
```

### Define an end tag, which includes a name
```ebnf
end_tag = { "</" ~ name ~ ">" }
```

### Define an attribute, which consists of a name and a value
```ebnf
attribute = { name ~ "=" ~ "\"" ~ value ~ "\"" }
```

### Define a name, which can include alphanumeric characters, hyphens, underscores, and colons
```ebnf
name = { (ASCII_ALPHANUMERIC | "-" | "_" | ":")+ }
```

### Define a value, which can include any character except a double quote
```ebnf
value = { (!"\"" ~ ANY)* }
```
### Define text, which can include any character except a less-than sign
```ebnf
text = { (!"<" ~ ANY)+ }
```