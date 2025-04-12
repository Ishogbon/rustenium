use std::ffi::c_int;
use regex::Regex;
use std::fs;
use std::io::Write;

fn snakify(input: &str) -> String {
    let mut snake = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.push(c.to_ascii_lowercase());
        } else if c == '-' {
            snake.push('_');
        } else {
            snake.push(c);
        }
    }

    snake
}
fn to_lowercase_first(s: &str) -> String {
    if s.is_empty() {
        return s.to_string(); // Return an empty string as is
    }

    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_lowercase().to_string(); // Convert the first character to lowercase
    let rest_of_string: String = chars.collect(); // Collect the rest of the string unchanged

    first_char + &rest_of_string // Concatenate the lowercase first char with the rest of the string
}
pub fn parse_bs_file(content: &str) {
    // Regex to find module declarations
    let re_module = Regex::new(r"##\s+(?:The\s+)?(\w+)\s+Module\s+##\s+\{#module-([^}]+)\}").unwrap();
    let re_general = Regex::new(r"([A-Z]\w+)([A-Z][a-z]+)\s*=\s*\(([^)]*)\)").unwrap();


    // Regex to find definitions
    let re_type = Regex::new(
        r"####\s+(?:The\s+)?([^\s.]+)\.([^\s]+)(?:\s+Type)?\s+####\s+\{#(?:type|types)-([^}]+)-([^}]+)\}"
    ).unwrap();
    let re_command = Regex::new(r"####\s+The\s+([^\s.]+)\.([^\s]+)\s+Command\s+####\s+\{#command-([^}]+)-([^}]+)\}").unwrap();
    let re_event = Regex::new(r"####\s+The\s+([^\s.]+)\.([^\s]+)\s+Event\s+####\s+\{#event-([^}]+)-([^}]+)\}").unwrap();

    // Regex to find CDDL in pre tags
    let re_pre_cddl = Regex::new(r#"<pre class="[^"]*cddl[^"]*">([\s\S]*?)</pre>"#).unwrap();
    let base_dir = String::from("./raw/source_generated");
    let codes_dir = base_dir + "/codes";
    fs::create_dir_all(codes_dir.clone()).unwrap();
    for caps in re_module.captures_iter(content) {
        let module_keyword = caps.get(1).unwrap().as_str();
        println!("Found module: '{}'", module_keyword);
        fs::create_dir_all(codes_dir.clone() + "/" + module_keyword).unwrap();

        for general_caps in re_general.captures_iter(content) {
            let general_module = general_caps.get(1).unwrap().as_str();
            let general_type = general_caps.get(2).unwrap().as_str();

            if to_lowercase_first(general_module) == module_keyword {
                if general_type == "Command" {
                    println!("Found general: '{} {}'", general_module, general_type);
                    let mut commands_file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/commands.rs").unwrap();
                    let parsed = conv_to_code_save_to_file(general_caps.get(0).unwrap().as_str(), &mut commands_file);
                    println!("General Command CDDL content: {}", parsed);
                } else if general_type == "Result" {
                    println!("Found general: '{} {}'", general_module, general_type);
                    let mut commands_file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/commands.rs").unwrap();
                    let parsed = conv_to_code_save_to_file(general_caps.get(0).unwrap().as_str(), &mut commands_file);
                    println!("General Result CDDL content: {}", parsed);
                } else if general_type == "Event" {
                    println!("Found general: '{} {}'", general_module, general_type);
                    let mut commands_file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/events.rs").unwrap();
                    let parsed = conv_to_code_save_to_file(general_caps.get(0).unwrap().as_str(), &mut commands_file);
                    println!("General Event CDDL content: {}", parsed);
                }
            }

        }

        // Now find all types belonging to this module
        for type_caps in re_type.captures_iter(content) {
            let type_module = type_caps.get(1).unwrap().as_str();
            let type_name = type_caps.get(2).unwrap().as_str();

            if type_module == module_keyword {
                println!("Found Type: '{}.{}'", module_keyword, type_name);

                // Find the position where this type definition ends
                let type_end = type_caps.get(0).unwrap().end();
                // Look at the content after this definition
                let remaining_content = &content[type_end..];

                // Find the first CDDL pre tag after this definition
                if let Some(cddl_caps) = re_pre_cddl.captures(remaining_content) {
                    let mut types_file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/types.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = conv_to_code_save_to_file(cddl_content, &mut types_file);
                    println!("Type CDDL content: {}", parsed);
                }
            }
        }

        for command_caps in re_command.captures_iter(content) {
            let command_module = command_caps.get(1).unwrap().as_str();
            let command_name = command_caps.get(2).unwrap().as_str();

            if command_module == module_keyword {
                println!("Found Command: '{}.{}'", module_keyword, command_name);

                // Find the position where this command definition ends
                let mut command_end = command_caps.get(0).unwrap().end();
                // Look at the content after this definition
                let remaining_content = &content[command_end..];

                if let Some(cddl_caps) = re_pre_cddl.captures(remaining_content) {
                    let mut commands_file =  fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/commands.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = conv_to_code_save_to_file(cddl_content, &mut commands_file);
                    println!("Command CDDL content: {}", parsed);
                }
            }
        }

        for event_caps in re_event.captures_iter(content) {
            let event_module = event_caps.get(1).unwrap().as_str();
            let event_name = event_caps.get(2).unwrap().as_str();

            if event_module == module_keyword {
                println!("Found Event: '{}.{}'", module_keyword, event_name);

                // Find the position where this event definition ends
                let event_end = event_caps.get(0).unwrap().end();
                // Look at the content after this definition
                let remaining_content = &content[event_end..];

                if let Some(cddl_caps) = re_pre_cddl.captures(remaining_content) {
                    let mut events_file =  fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(codes_dir.clone() + "/" + module_keyword + "/events.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = conv_to_code_save_to_file(cddl_content, &mut events_file);
                    println!("Event CDDL content: {}", parsed);
                }
            }
        }
    }
}

fn conv_to_code_save_to_file(tag: &str, file: &mut fs::File) -> String {
    let code = conv_to_code(tag); // Get the string from conv_to_code
println!("code: {}", code);
    // Write the contents to the file
    file.write_all(code.as_bytes()).unwrap(); // Write the code to the file
    code
}

fn conv_to_code(cddl_content: &str) -> String {
    let re_assignment_w_curl_brace = Regex::new(r"(\w+)\.(\w+)\s+=\s+\{").unwrap();
    let re_assignment_w_brace = Regex::new(r"(\w+)\s+=\s+\(").unwrap();
    let re_choice_next_attr = Regex::new(r"(\w+)\.(\w+)\s+/").unwrap();
    let re_choice_attr = Regex::new(r"(\w+)\.(\w+)").unwrap();

    let re_assignment_attr = Regex::new(r"(\w+):\s+(.+)").unwrap();

    // let re_nested_attr = Regex::new()

    let mut code = String::new();
    let mut curl_brace_activated = 0;
    let mut brace_activated = 0;

    let cddl_lines: Vec<&str> = cddl_content.lines().collect();

    for (index, line) in cddl_lines.iter().enumerate() {
        if let Some(assign_cap) = re_assignment_w_curl_brace.captures(line) {
            let attribute_name = assign_cap.get(2).unwrap().as_str();

            // Check next line safely
            if index + 1 < cddl_lines.len() {
                if re_choice_next_attr.is_match(cddl_lines[index + 1]) {
                    code.push_str(&format!("enum {} {{\n", attribute_name));
                    curl_brace_activated += 1;
                    continue;
                } else if re_assignment_attr.is_match(cddl_lines[index + 1]) {
                    code.push_str(&format!("#[derive(Debug, Serialize, Deserialize)]\nstruct {} {{\n", attribute_name));
                    curl_brace_activated += 1;
                    continue;
                }
            }
        }

        if let Some(assign_cap) = re_assignment_w_brace.captures(line) {
            let attribute_name = assign_cap.get(1).unwrap().as_str();

            // Check next line safely
            if index + 1 < cddl_lines.len() {
                if re_choice_next_attr.is_match(cddl_lines[index + 1]) {
                    code.push_str(&format!("enum {} {{\n", attribute_name));
                    brace_activated += 1;
                    continue;
                } else if re_assignment_attr.is_match(cddl_lines[index + 1]) {
                    code.push_str(&format!("#[derive(Debug, Serialize, Deserialize)]\nstruct {} {{\n", attribute_name));
                    brace_activated += 1;
                    continue;
                }
            }
        }//

        if curl_brace_activated > 0 {
            if let Some(attr_cap) = re_assignment_attr.captures(line) {
                let (attr_name, attr_value) = parse_value(line);
                let mut snakified = snakify(attr_name.as_str());
                if snakified == "type" {
                    snakified = String::from("type_");
                }
                code.push_str(&format!("\t#[serde(rename = \"{}\")]\n\t{}: {},\n", attr_name, snakified, attr_value));
            } else if let Some(choice_cap) = re_choice_attr.captures(line) {
                if re_assignment_attr.is_match(cddl_lines[index + 1]) {
                    let (attr_name, attr_value) = parse_value(line);
                    let mut snakified = snakify(attr_name.as_str());
                    if snakified == "type" {
                        snakified = String::from("type_");
                    }
                    code.push_str(&format!("\t#[serde(rename = \"{}\")]\n\t{}: {},\n", attr_name, snakified, attr_value));
                } else {
                    let variant_name = choice_cap.get(2).unwrap().as_str();
                    code.push_str(&format!("\t{}({}),\n", variant_name, variant_name));
                }
            } else if (line.to_string().contains("Extensible")) {
                code.push_str(&format!("\t#[serde(flatten)]\n\textension: Option<serde_cbor::Value>,\n"));
            } else if line.trim() == "}" {
                code.push_str("}\n\n");
                curl_brace_activated -= 1;
            }
        }
        if brace_activated > 0 {
            if let Some(attr_cap) = re_assignment_attr.captures(line) {
                let (attr_name, attr_value) = parse_value(line);
                let mut snakified = snakify(attr_name.as_str());
                if snakified == "type" {
                    snakified = String::from("type_");
                }
                code.push_str(&format!("\t#[serde(rename = \"{}\")]\n\t{}: {},\n", attr_name, snakified, attr_value));
            } else if let Some(choice_cap) = re_choice_attr.captures(line) {
                if re_assignment_attr.is_match(cddl_lines[index + 1]) {
                    let (attr_name, attr_value) = parse_value(line);
                    let mut snakified = snakify(attr_name.as_str());
                    if snakified == "type" {
                        snakified = String::from("type_");
                    }
                    code.push_str(&format!("\t#[serde(rename = \"{}\")]\n\t{}: {},\n", attr_name, snakified, attr_value));
                } else {
                    let variant_name = choice_cap.get(2).unwrap().as_str();
                    code.push_str(&format!("\t{}({}),\n", variant_name, variant_name));
                }
            }  else if (line.to_string().contains("Extensible")) {
                code.push_str(&format!("\t#[serde(flatten)]\n\textension: Option<serde_cbor::Value>,\n"));
            } else if line.trim() == ")" {
                code.push_str("}\n\n");
                brace_activated -= 1;
            }
        }
    }

    code
}

fn parse_value (value: &str) -> (String, String) {
    fn value_determiner(value: &str) -> String {
        let re_struct_type = Regex::new(r"^\w+\.(\w+)[,;]*$").unwrap();
        let re_primitive_type = Regex::new(r"^(bool),*$|^(js-int),*$|^(text),*$|^(js-uint),*$|^(float),*$|^(null),*$|^(\{*text => text}),*$|^\[\s*\*\s*([\w.]+)],*$|^\[\s*\+\s*([\w.]+)],*$").unwrap();
        let re_string_type = Regex::new(r#""([A-Za-z0-9.]+)""#).unwrap();
        let re_multi_union_type = Regex::new(r"/{1,2}").unwrap();


        let re_array_primitive_type = Regex::new(r"\s*\[\s*\s*[*+](text)],*").unwrap();

        let re_float = Regex::new(r"^\d+\.+\d+[,;]*$").unwrap();


        if value.contains("EmptyParams") {
            return "Option<serde_cbor::Value>".to_string();
        }
        if re_multi_union_type.is_match(value) {
            let parts: Vec<&str> = value.split(|c| c == '\\' || c == '/')
                .map(|s| s.trim())
                .collect();
            let mapped_types: Vec<String> = parts.iter().map(|part| value_determiner(part)).collect();
            return mapped_types.join(" | ");
        }
        if let Some(primitive_array_cap) = re_array_primitive_type.captures(value) {
            let cap = primitive_array_cap.get(1).unwrap().as_str();
            let value = value_determiner(cap);
            return format!("Vec<{}>", value);
        }
        if re_float.is_match(value) {
            return "f64".to_string();
        }

        if let Some (primitive_cap) = re_primitive_type.captures(value) {
            for i in 1..=primitive_cap.len() {
                if let Some(matched) = primitive_cap.get(i) {
                    return match matched.as_str() {
                        "js-int" => "i32".to_string(),
                        "js-uint" => "u32".to_string(),
                        "text" => "String".to_string(),
                        "bool" => "bool".to_string(),
                        "float" => "f64".to_string(),
                        "null" => "None".to_string(),
                        "{*text => text}" => "HashMap<String, String>".to_string(),
                        val => {
                            if val == primitive_cap.get(8).map(|m| m.as_str()).unwrap_or("") {
                                let type_str = val.split('.').last().unwrap();
                                format!("Vec<{}>", type_str)
                            } else if val == primitive_cap.get(9).map(|m| m.as_str()).unwrap_or("") {
                                let type_str = val.split('.').last().unwrap();
                                format!("Vec<{}>", type_str)
                            } else {
                                return value.to_string();
                            }
                        }
                    }
                }
            }
        }
        if let Some(cap) = re_struct_type.captures(value) {
            return cap.get(1).unwrap().as_str().to_string();
        }
        if let Some (string_cap) = re_string_type.captures(value) {
            // return format!("\"{}\"", string_cap.get(1).unwrap().as_str().to_string());
            return "&str".to_string()
        }
        return String::new();
    };
    let re_optional_type = Regex::new(r"\?\s+(\w+):\s*(.+)").unwrap();
    let re_type = Regex::new(r"(\w+):\s*(.+)").unwrap();

    let re_no_assignment = Regex::new(r"(\w+)\.(\w+)").unwrap();
    let re_no_assignment_optional = Regex::new(r"\s*\?\s*(\w+)\.(\w+)").unwrap();

    if let Some(re_optional_type_cap) = re_optional_type.captures(value) {
        let attr_name = String::from(re_optional_type_cap.get(1).unwrap().as_str());
        let attr_value = format!("Option<{}>", value_determiner(re_optional_type_cap.get(2).unwrap().as_str()));
        return (attr_name, attr_value);
    }
    else if let Some(re_type_cap) = re_type.captures(value) {
        let attr_name = String::from(re_type_cap.get(1).unwrap().as_str());
        let attr_value = format!("{}", value_determiner(re_type_cap.get(2).unwrap().as_str()));
        return (attr_name, attr_value);
    } else if let Some(re_no_assignment_caps_optional) = re_no_assignment_optional.captures(value) {
        let attr_name = re_no_assignment_caps_optional.get(2).unwrap().as_str();
        let attr_value = format!("Option<{}>", attr_name);
        return (attr_name.to_string(), attr_value);
    } else if let Some(re_no_assignment_cap) = re_no_assignment.captures(value) {
        let attr_name = re_no_assignment_cap.get(2).unwrap().as_str();
        let attr_value = format!("{}", attr_name);
        return (attr_name.to_string(), attr_value);
    }
    (String::from(""), String::from(""))
}