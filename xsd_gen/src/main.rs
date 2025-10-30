use xsd_parser::{Config, config::Schema, generate};
use std::fs;
use std::path::PathBuf;

mod flatten_types;

fn main() {
    // Get absolute path for DataPrimitives schema
    let data_primitives_path = PathBuf::from("..").join("schemas").join("DataPrimitives.xsd");
    let data_primitives_path = fs::canonicalize(&data_primitives_path)
        .expect("Failed to get absolute path for DataPrimitives.xsd");

    println!("=== Generating Rust types from DataPrimitives.xsd ===");
    println!("Reading from: {}\n", data_primitives_path.display());

    // Read DataPrimitives schema
    let data_primitives_content = fs::read_to_string(&data_primitives_path)
        .expect("Failed to read DataPrimitives.xsd");

    // Generate types from DataPrimitives schema with serde support
    let config = Config::default()
        .with_schema(Schema::schema(data_primitives_content))
        .with_derive(vec!["Debug", "Serialize", "Deserialize"]);

    match generate(config) {
        Ok(module) => {
            let code = module.to_string();

            println!("Successfully generated Rust code!");
            println!("Generated {} characters of code\n", code.len());

            // Pretty print by adding newlines after each type definition
            let formatted = code
                .replace("# [", "\n#[")
                .replace(" pub enum ", "\npub enum ")
                .replace(" pub struct ", "\npub struct ")
                .replace(" pub type ", "\npub type ")
                .replace(" ; ", ";\n");

            println!("{}", formatted);

            // Flatten Type/TypeContent pairs
            let flattened = flatten_types::flatten_generated_code(&formatted);

            // Add serde imports at the top
            let with_imports = format!("use serde::{{Serialize, Deserialize}};\n\n{}", flattened);

            // Also save to a file
            let output_path = PathBuf::from("..").join("crcl").join("src").join("primitives.rs");
            fs::write(&output_path, &with_imports)
                .expect("Failed to write generated code to file");
            println!("\n✓ Code also saved to: {}", output_path.display());
            println!("✓ Flattened Type/TypeContent pairs for cleaner API");

            // Format with rustfmt
            format_file(output_path.to_str().unwrap());
        }
        Err(e) => {
            eprintln!("Error generating from DataPrimitives.xsd: {}", e);
        }
    }
}

fn format_file(file_path: &str) {
    use std::process::Command;

    print!("✓ Formatting with rustfmt... ");
    match Command::new("rustfmt")
        .arg(file_path)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("done");
            } else {
                println!("warning: rustfmt had issues");
                if !output.stderr.is_empty() {
                    eprintln!("  {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            println!("warning: couldn't run rustfmt ({})", e);
            println!("  The code is valid but not formatted. Install rustfmt with: rustup component add rustfmt");
        }
    }
}
