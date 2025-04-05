use regex::Regex;
use std::fs;
use std::io::Write;

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

    let mut code = String::new();
    let mut enum_curl_brace_activated = false;
    let mut enum_brace_activated = false;

    let cddl_lines: Vec<&str> = cddl_content.lines().collect();

    for (index, line) in cddl_lines.iter().enumerate() {
        if let Some(assign_cap) = re_assignment_w_curl_brace.captures(line) {
            let attribute_name = assign_cap.get(2).unwrap().as_str();

            // Check next line safely
            if index + 1 < cddl_lines.len() && re_choice_next_attr.is_match(cddl_lines[index + 1]) {
                code.push_str(&format!("enum {} {{\n", attribute_name));
                enum_curl_brace_activated = true;
                continue;
            }
        }

        if let Some(assign_cap) = re_assignment_w_brace.captures(line) {
            let attribute_name = assign_cap.get(1).unwrap().as_str();

            // Check next line safely
            if index + 1 < cddl_lines.len() && re_choice_next_attr.is_match(cddl_lines[index + 1]) {
                code.push_str(&format!("enum {} {{\n", attribute_name));
                enum_brace_activated = true;
                continue;
            }
        }

        if enum_curl_brace_activated {
            if let Some(choice_cap) = re_choice_attr.captures(line) {
                let variant_name = choice_cap.get(2).unwrap().as_str();
                code.push_str(&format!("    {}({}),\n", variant_name, variant_name));
            } else if line.trim() == "}" {
                code.push_str("}\n");
                enum_curl_brace_activated = false;
            }
        }
        if enum_brace_activated {
            if let Some(choice_cap) = re_choice_attr.captures(line) {
                let variant_name = choice_cap.get(2).unwrap().as_str();
                code.push_str(&format!("    {}({}),\n", variant_name, variant_name));
            } else if line.trim() == ")" {
                code.push_str("}\n");
                enum_curl_brace_activated = false;
            }
        }
    }

    code
}
