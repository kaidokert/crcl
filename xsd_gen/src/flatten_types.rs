// Post-processor to flatten Type/TypeContent pairs into single structs
//
// The xsd-parser generates a two-level structure:
//   pub struct XyzType { pub content: XyzTypeContent }
//   pub struct XyzTypeContent { actual fields }
//
// This flattens them to:
//   pub struct XyzType { actual fields }

use std::collections::HashMap;

pub fn flatten_generated_code(code: &str) -> String {
    let lines: Vec<&str> = code.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    // First pass: collect all TypeContent definitions
    let mut content_types: HashMap<String, Vec<String>> = HashMap::new();
    let mut j = 0;
    while j < lines.len() {
        let line = lines[j];

        // Look for: pub struct XyzTypeContent { fields }
        if line.contains("pub struct") && line.contains("TypeContent") {
            if let Some(type_name) = extract_type_name_content(line) {
                let mut fields = Vec::new();

                // Collect the struct definition
                if line.contains('{') && line.contains('}') {
                    // Single line struct
                    if let Some(field_part) = line.split('{').nth(1) {
                        if let Some(fields_str) = field_part.split('}').next() {
                            fields.push(fields_str.trim().to_string());
                        }
                    }
                } else {
                    // Multi-line struct (less common with our formatting)
                    fields.push(line.to_string());
                    j += 1;
                    while j < lines.len() && !lines[j].contains('}') {
                        fields.push(lines[j].to_string());
                        j += 1;
                    }
                    if j < lines.len() {
                        fields.push(lines[j].to_string());
                    }
                }

                content_types.insert(type_name, fields);
            }
        }
        j += 1;
    }

    // Second pass: process and flatten
    while i < lines.len() {
        let line = lines[i];

        // Check if this is a wrapper type: pub struct XyzType { pub content : XyzTypeContent , }
        if line.contains("pub struct") && line.contains("Type {") &&
           line.contains("pub content :") && line.contains("TypeContent") {

            if let Some((type_name, content_type)) = extract_wrapper_info(line) {
                // Check if we have the Content type definition
                if let Some(fields) = content_types.get(&content_type) {
                    // Get the derive line (should be just before)
                    if i > 0 && lines[i-1].contains("#[derive") {
                        result.push(lines[i-1].to_string());
                    }

                    // Output flattened struct
                    let fields_str = fields.join(" ");
                    result.push(format!("pub struct {} {{ {} }}", type_name, fields_str.trim()));

                    // Skip this line and any following TypeContent definition
                    i += 1;

                    // Skip the next derive + TypeContent struct if it's right after
                    if i < lines.len() && lines[i].contains("#[derive") {
                        i += 1; // skip derive
                        if i < lines.len() && lines[i].contains(&content_type) {
                            i += 1; // skip TypeContent struct
                        } else {
                            i -= 1; // backtrack if we skipped wrong thing
                        }
                    }
                    continue;
                }
            }
        }

        // Check if this line is a TypeContent struct we should skip
        if line.contains("pub struct") && line.contains("TypeContent") {
            // This should already be handled, but skip it anyway
            i += 1;
            continue;
        }

        // Normal line - add it unless it's a TypeContent derive
        if i == 0 || !(line.contains("#[derive") && i + 1 < lines.len() &&
                       lines[i+1].contains("TypeContent")) {
            result.push(line.to_string());
        }

        i += 1;
    }

    result.join("\n")
}

fn extract_type_name_content(line: &str) -> Option<String> {
    // Extract "XyzTypeContent" from "pub struct XyzTypeContent {"
    if let Some(after_struct) = line.split("pub struct").nth(1) {
        if let Some(name) = after_struct.trim().split_whitespace().next() {
            return Some(name.to_string());
        }
    }
    None
}

fn extract_wrapper_info(line: &str) -> Option<(String, String)> {
    // Extract ("XyzType", "XyzTypeContent") from line like:
    // pub struct XyzType { pub content : XyzTypeContent , }

    if let Some(after_struct) = line.split("pub struct").nth(1) {
        let parts: Vec<&str> = after_struct.split_whitespace().collect();
        if parts.len() >= 5 {
            let type_name = parts[0].to_string();
            // Find the TypeContent name
            for part in parts {
                if part.contains("TypeContent") {
                    let content_name = part.trim_end_matches(',').to_string();
                    return Some((type_name, content_name));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_simple_type() {
        let input = r#"#[derive (Debug , Serialize , Deserialize)]
pub struct JointStatusType { pub content : JointStatusTypeContent , }
#[derive (Debug , Serialize , Deserialize)]
pub struct JointStatusTypeContent { pub joint_number : IntType , pub joint_position : Option < DoubleType > , }"#;

        let result = flatten_generated_code(input);

        assert!(result.contains("pub struct JointStatusType { pub joint_number"));
        assert!(!result.contains("TypeContent"));
        assert!(!result.contains("pub content :"));
    }
}
