use crate::{AbxError, AbxToXmlConverter, Result};
use clap::{Arg, Command};
use std::io::Read;
use std::str::FromStr;
use xmlem::Document;

pub struct Cli;

impl Cli {
    pub fn build_command() -> Command {
        Command::new("abx2xml")
            .about("Converts Android Binary XML (ABX) to human-readable XML")
            .long_about("Converts between Android Binary XML and human-readable XML.\n\nWhen invoked with the '-i' argument, the output of a successful conversion will overwrite the original input file. Input can be '-' to use stdin, and output can be '-' to use stdout.")
            .arg(
                Arg::new("in-place")
                    .short('i')
                    .long("in-place")
                    .help("Overwrite input file with converted output")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("input")
                    .help("Input file path (use '-' for stdin)")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("output")
                    .help("Output file path (use '-' for stdout)")
                    .index(2),
            )
            .arg(
                Arg::new("no-format")
                    .long("no-format")
                    .help("Skip XML formatting (output raw XML)")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    pub fn run() -> Result<()> {
        let matches = Self::build_command().get_matches();
        Self::run_with_matches(matches)
    }

    pub fn run_with_matches(matches: clap::ArgMatches) -> Result<()> {
        let input_path = matches.get_one::<String>("input").unwrap();
        let output_path = matches.get_one::<String>("output");
        let in_place = matches.get_flag("in-place");
        let no_format = matches.get_flag("no-format");

        if in_place && input_path == "-" {
            return Err(AbxError::ParseError(
                "Cannot use -i option with stdin input".to_string(),
            ));
        }

        let output_path = match output_path {
            Some(path) => path.clone(),
            None => {
                if in_place {
                    input_path.clone()
                } else {
                    "-".to_string()
                }
            }
        };

        // Convert to XML string first
        let xml_content = Self::convert_to_xml_string(input_path)?;
        
        // Format the XML unless --no-format is specified
        let formatted_xml = if no_format {
            xml_content
        } else {
            Self::format_xml(&xml_content)?
        };

        // Write to output
        Self::write_output(&formatted_xml, &output_path)?;

        Ok(())
    }

    fn convert_to_xml_string(input_path: &str) -> Result<String> {
        // This function should capture the XML output as a string
        // instead of writing directly to files/stdout
        match input_path {
            "-" => {
                // Read all data from stdin
                let mut buffer = Vec::new();
                std::io::stdin().read_to_end(&mut buffer)
                    .map_err(|e| AbxError::ParseError(format!("Failed to read from stdin: {}", e)))?;
                AbxToXmlConverter::convert_bytes(&buffer)
            },
            input => {
                // Read file and convert
                let file_data = std::fs::read(input)
                    .map_err(|e| AbxError::ParseError(format!("Failed to read file {}: {}", input, e)))?;
                AbxToXmlConverter::convert_bytes(&file_data)
            }
        }
    }

    fn format_xml(xml_content: &str) -> Result<String> {
        // Use xmlem to format the XML properly
        match Document::from_str(xml_content) {
            Ok(doc) => Ok(doc.to_string_pretty()),
            Err(e) => {
                // If xmlem fails to parse, return the original content
                // This handles cases where the XML might not be well-formed
                eprintln!("Warning: Failed to format XML ({}), returning unformatted", e);
                Ok(xml_content.to_string())
            }
        }
    }

    fn write_output(content: &str, output_path: &str) -> Result<()> {
        match output_path {
            "-" => {
                print!("{}", content);
                Ok(())
            }
            path => {
                std::fs::write(path, content)
                    .map_err(|e| AbxError::ParseError(format!("Failed to write to {}: {}", path, e)))
            }
        }
    }
}
