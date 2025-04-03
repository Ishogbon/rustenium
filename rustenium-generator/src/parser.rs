use regex::Regex;
use std::fs;

pub fn parse_bs_file(content: &str) {
    // Regex to find module declarations
    let re_module = Regex::new(r"##\s+(?:The\s+)?(\w+)\s+Module\s+##\s+\{#module-([^}]+)\}").unwrap();

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
                    let types_file = fs::File::create(codes_dir.clone() + "/" + module_keyword + "/types.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = parse_pre_tag(cddl_content);
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
                let command_end = command_caps.get(0).unwrap().end();
                // Look at the content after this definition
                let remaining_content = &content[command_end..];

                if let Some(cddl_caps) = re_pre_cddl.captures(remaining_content) {
                    let commands_file = fs::File::create(codes_dir.clone() + "/" + module_keyword + "/commands.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = parse_pre_tag(cddl_content);
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
                    let events_file = fs::File::create(codes_dir.clone() + "/" + module_keyword + "/events.rs").unwrap();
                    let cddl_content = cddl_caps.get(1).unwrap().as_str();
                    let parsed = parse_pre_tag(cddl_content);
                    println!("Event CDDL content: {}", parsed);
                }
            }
        }
    }
}

fn parse_pre_tag(tag: &str) -> String {
    // Basic implementation - you might want to add more processing here
    tag.trim().to_string()
}