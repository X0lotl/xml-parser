use clap::{Parser, Subcommand};
use std::fs;
use xml_parser::{parse_xml, XmlValue};

#[derive(Parser)]
#[command(name = "XML Parser CLI")]
#[command(about = "A command-line interface for parsing XML files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        file: String,
    },
    Help,
    Credits,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let xml_content = fs::read_to_string(file).expect("Unable to read file");
            match parse_xml(&xml_content) {
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
        Commands::Help => {
            println!("XML Parser CLI\n\nCommands:\n  parse  Parse an XML file\n  help   Display help information\n  credits Display credits information");
        }
        Commands::Credits => {
            println!("XML Parser CLI\n\nDeveloped by: Danylo Khomichenko\nVersion: 1.0.0");
        }
    }
}
