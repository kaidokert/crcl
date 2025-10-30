use xsd_parser::{Config, config::Schema, generate};
use std::fs;
use std::path::PathBuf;

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

    // Generate types from DataPrimitives schema
    let config = Config::default()
        .with_schema(Schema::schema(data_primitives_content));

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

            // Also save to a file
            let output_path = PathBuf::from("generated_data_primitives.rs");
            fs::write(&output_path, &formatted)
                .expect("Failed to write generated code to file");
            println!("\n✓ Code also saved to: {}", output_path.display());
        }
        Err(e) => {
            eprintln!("Error generating from DataPrimitives.xsd: {}", e);
        }
    }
}
