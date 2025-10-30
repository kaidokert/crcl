use xsd_parser::{Config, config::Schema, generate};
use std::fs;
use std::path::PathBuf;

mod flatten_types;

fn main() {
    println!("=== Attempting to generate CRCL types with schema includes ===\n");

    // Get the schemas directory
    let schemas_dir = PathBuf::from("..").join("schemas");
    let schemas_dir = fs::canonicalize(&schemas_dir)
        .expect("Failed to get absolute path for schemas directory");

    println!("Schemas directory: {}\n", schemas_dir.display());

    // Manually merging schemas
    println!("--- Approach: Manual schema merging ---");

    let data_primitives = fs::read_to_string(schemas_dir.join("DataPrimitives.xsd"))
        .expect("Failed to read DataPrimitives.xsd");
    let commands = fs::read_to_string(schemas_dir.join("CRCLCommands.xsd"))
        .expect("Failed to read CRCLCommands.xsd");

    // Try to merge by replacing the include directive
    let merged = merge_xsd_schemas(&commands, &data_primitives);

    let config = Config::default()
        .with_schema(Schema::schema(merged))
        .with_derive(vec!["Debug", "Serialize", "Deserialize"]);

    match generate(config) {
        Ok(module) => {
            let code = module.to_string();
            println!("✓ Success! Generated {} chars", code.len());

            if code.contains("ActuateJoints") {
                println!("✓ Found ActuateJointsType in generated code");
            }

            let formatted = code
                .replace("# [", "\n#[")
                .replace(" pub enum ", "\npub enum ")
                .replace(" pub struct ", "\npub struct ")
                .replace(" pub type ", "\npub type ")
                .replace(" ; ", ";\n");

            // Flatten Type/TypeContent pairs
            let flattened = flatten_types::flatten_generated_code(&formatted);

            // Add serde imports at the top
            let with_imports = format!("use serde::{{Serialize, Deserialize}};\n\n{}", flattened);

            let output_path = PathBuf::from("..").join("crcl").join("src").join("commands.rs");
            fs::write(&output_path, &with_imports)
                .expect("Failed to write output");
            println!("✓ Saved to {}", output_path.display());
            println!("✓ Flattened Type/TypeContent pairs for cleaner API");

            // Format with rustfmt
            format_file(output_path.to_str().unwrap());
        }
        Err(e) => {
            eprintln!("✗ Error: {}\n", e);
        }
    }

    // Now generate Status types with JointStatusesType
    println!("\n--- Generating CRCL Status types ---");

    let status = fs::read_to_string(schemas_dir.join("CRCLStatus.xsd"))
        .expect("Failed to read CRCLStatus.xsd");

    let merged_status = merge_xsd_schemas(&status, &data_primitives);

    let config = Config::default()
        .with_schema(Schema::schema(merged_status))
        .with_derive(vec!["Debug", "Serialize", "Deserialize"]);

    match generate(config) {
        Ok(module) => {
            let code = module.to_string();
            println!("✓ Success! Generated {} chars", code.len());

            if code.contains("JointStatuses") {
                println!("✓ Found JointStatusesType in generated code");
            }

            let formatted = code
                .replace("# [", "\n#[")
                .replace(" pub enum ", "\npub enum ")
                .replace(" pub struct ", "\npub struct ")
                .replace(" pub type ", "\npub type ")
                .replace(" ; ", ";\n");

            // Flatten Type/TypeContent pairs
            let flattened = flatten_types::flatten_generated_code(&formatted);

            // Add serde imports at the top
            let with_imports = format!("use serde::{{Serialize, Deserialize}};\n\n{}", flattened);

            let output_path = PathBuf::from("..").join("crcl").join("src").join("status.rs");
            fs::write(&output_path, &with_imports)
                .expect("Failed to write output");
            println!("✓ Saved to {}", output_path.display());
            println!("✓ Flattened Type/TypeContent pairs for cleaner API");

            // Format with rustfmt
            format_file(output_path.to_str().unwrap());
        }
        Err(e) => {
            eprintln!("✗ Error: {}\n", e);
        }
    }
}

fn merge_xsd_schemas(main_schema: &str, included_schema: &str) -> String {
    // Find the include statement
    let include_pattern = r#"<xs:include schemaLocation="DataPrimitives.xsd"/>"#;

    if let Some(include_pos) = main_schema.find(include_pattern) {
        // Extract the types from included schema (skip the XML header and schema tag)
        let included_types = extract_schema_body(included_schema);

        // Replace the include with the actual content
        let before = &main_schema[..include_pos];
        let after = &main_schema[include_pos + include_pattern.len()..];

        format!("{}\n    <!-- Inlined from DataPrimitives.xsd -->\n{}\n    <!-- End of DataPrimitives.xsd -->\n{}",
                before, included_types, after)
    } else {
        main_schema.to_string()
    }
}

fn extract_schema_body(xsd: &str) -> String {
    // Find the content between <xs:schema> tags, excluding the opening and closing schema tags
    if let Some(start) = xsd.find("<xs:schema") {
        if let Some(schema_end) = xsd[start..].find('>') {
            let schema_start = start + schema_end + 1;
            if let Some(end) = xsd.rfind("</xs:schema>") {
                // Return everything between the schema tags, properly indented
                let body = &xsd[schema_start..end];
                return body.to_string();
            }
        }
    }
    xsd.to_string()
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
